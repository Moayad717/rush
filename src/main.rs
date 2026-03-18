use std::io::{self, Write};

mod executor;
mod input;

fn main() {
    let mut history: Vec<String> = Vec::new();

    loop {
        let prompt = format!("rush: {} > ", std::env::current_dir().unwrap().display());
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush");

        let input = input::read_input(&history, &prompt);

        history.push(input.clone());

        let input = input.trim();

        if input.is_empty() {
            continue;
        } else if input == "exit" {
            break;
        } else if input.contains('|') {
            executor::run_pipe(input);
        } else if input.contains('<') || input.contains('>') {
            executor::run_redirect(input);
        } else if executor::is_builtin(input) {
            executor::run_builtin(input);
        } else {
            executor::run_single(input);
        }
    }
}
