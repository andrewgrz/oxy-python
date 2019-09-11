//! The lexer module holds functions and structs
//! that assist with lexing a python program

use std::str::Chars;

/// A location in the file
#[derive(Debug)]
pub struct Location {
    /// The number of the line, starting with 1
    line: u64,
    /// The number of the column, starting with 1
    column: u64,
}

impl Location {
    fn new(line: u64, column: u64) -> Location {
        Location { line, column }
    }
}

/// Holds a lexed token and data with its position in the file
#[derive(Debug)]
pub struct Token {
    /// The start location of the token
    start: Location,
    /// The end location of the token
    end: Location,
    /// The value of the token
    token_type: TokenType,
}

impl Token {
    /// Asserts if the token is the given type
    pub fn is_type(&self, other: &TokenType) -> bool {
        other == &self.token_type
    }
}

/// A type of token with the data inside
#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    // ---- Arithmetic Tokens ----
    /// Plus Sign
    Plus,
    /// Minus Sign
    Minus,
    /// Asterisk/Multiply sign
    Star,
    /// Two Stars in a row
    StarStar,
    /// Single forward Slash
    Slash,

    // Data Tokens
    /// Name token, the value of the name is in the string
    Name(String),

    // ---- Keywords ----
    /// If keyword
    If,
    /// Else keyword
    Else,

    // ---- Layout Tokens ----
    /// The code has been indented one level
    Indent,
    /// The code has been dedented on level
    Dedent,
}

/// An error thrown when lexing fails
#[derive(Debug)]
pub enum LexError {
    UnexpectedToken(char, Location, Location),
}

/// Alias for what the lexer will return
pub type LexResult = Result<Vec<Token>, LexError>;

/// Lex this string.
///
/// ```
/// extern crate oxy_python;
/// use oxy_python::parser::lex;
/// let example = "2+2";
/// let result = lex(example);
/// ```
pub fn lex(string: &str) -> LexResult {
    use TokenType::*;
    let mut result: Vec<Token> = Vec::new();
    let mut chars = string.chars();
    let mut maybe_c: Option<char> = chars.next();
    let mut column = 1;
    let mut line = 1;

    macro_rules! advance {
        () => {{
            maybe_c = chars.next();
        }};
    }

    macro_rules! push_tok {
        ($tok: expr, $span: expr) => {{
            let start = Location::new(column, line);
            column += $span;
            let end = Location::new(column - 1, line);
            result.push(Token {
                token_type: $tok,
                start,
                end,
            });
        }};
    }

    loop {
        match maybe_c {
            Some(c) => {
                match c {
                    '+' => push_tok!(Plus, 1),
                    '-' => push_tok!(Minus, 1),
                    '*' => {
                        advance!();
                        if let Some(n) = maybe_c {
                            match n {
                                '*' => {
                                    push_tok!(StarStar, 2);
                                    advance!();
                                }
                                _ => push_tok!(Star, 1),
                            }
                        } else {
                            push_tok!(Star, 1)
                        }
                        continue;
                    }
                    _ if c.is_alphabetic() => {
                        let s = take_until(maybe_c, &mut chars, |x| x.is_alphabetic());
                        push_tok!(check_keyword(s), s.len() as u64)
                    }
                    ' ' => {}
                    '/' => push_tok!(Slash, 1),
                    _ => {
                        return Err(LexError::UnexpectedToken(
                            c,
                            Location::new(column, line),
                            Location::new(column, line),
                        ))
                    }
                }
                advance!();
            }
            None => break,
        }
    }

    Ok(result)
}

fn check_keyword(s: String) -> TokenType {
    match s.as_str() {
        "if" => TokenType::If,
        "else" => TokenType::Else,
        _ => TokenType::Name(s),
    }
}

fn take_until<P>(mut maybe_c: Option<char>, chars: &mut Chars, mut predicate: P) -> String
where
    P: FnMut(&char) -> bool,
{
    let mut data = String::new();

    loop {
        match maybe_c {
            Some(c) => {
                if predicate(&c) {
                    data.push(c);
                    maybe_c = chars.next();
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! print_diff {
        ($s: expr, $expected: expr, $actual: expr) => {{
            println!("Error! Expected did not equal Actual for: '{}'", $s);
            println!("Excepted: {:?}", $expected);
            println!(
                "Actual:   {:?}",
                $actual
                    .into_iter()
                    .map(|x| x.token_type.clone())
                    .collect::<Vec<TokenType>>()
            );
            panic!();
        }};
    }

    macro_rules! lex_test {
        ($s: expr, $( $x:expr ),*) => {{
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*

            match lex($s) {
            Ok(result) =>  {
                if result.len() != temp_vec.len() {
                    print_diff!($s, &temp_vec, &result)
                } else {
                    for (index, value) in result.iter().enumerate() {
                        if !value.is_type(&temp_vec[index]) {
                            print_diff!($s, &temp_vec, &result)
                        }
                    }
                }
            } Err(e) => {
                panic!("Did not lex correctly. {}. Received: {:?}", $s, e);
            }
    }
        }};
    }

    #[test]
    fn test_lex_1_token() {
        use super::TokenType::*;
        lex_test!("**", StarStar);
        lex_test!("+", Plus);
        lex_test!("-", Minus);
        lex_test!("*", Star);
        lex_test!("/", Slash);
        lex_test!("if", If);
        lex_test!("else", Else);
    }

    #[test]
    fn test_name_token() {
        use super::TokenType::*;
        lex_test!("test", Name("test".to_owned()));
        lex_test!(
            "test tested",
            Name("test".to_owned()),
            Name("tested".to_owned())
        );
    }

    #[test]
    fn test_lex_multi_token() {
        use super::TokenType::*;
        lex_test!("* *", Star, Star);
        lex_test!("+ *", Plus, Star);
        lex_test!("*-", Star, Minus);
    }
}
