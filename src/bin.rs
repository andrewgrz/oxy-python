
extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

const HISTORY_FILE: &'static str = "oxy-py-history.txt";

fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let _ = rl.load_history(HISTORY_FILE);
    println!("Oxy Python {}", "alpha");
    println!();

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(HISTORY_FILE).unwrap();
}
