use std::io::{self, Read, Write};
use termios::*;

const STDIN_FILENO: i32 = 0;

pub fn read_input(history: &[String], prompt: &str) -> String {
    let original = Termios::from_fd(STDIN_FILENO).unwrap();
    let mut termios = original.clone();
    cfmakeraw(&mut termios);
    tcsetattr(STDIN_FILENO, TCSANOW, &termios).unwrap();

    let mut line = String::new();
    let mut buf = [0u8; 1];
    let mut idx = history.len();

    loop {
        io::stdin().read(&mut buf).unwrap();

        match buf[0] {
            b'\r' => {
                print!("\r\n");
                io::stdout().flush().unwrap();
                break;
            }
            127 => {
                if !line.is_empty() {
                    line.pop();
                    print!("\x08 \x08");
                    io::stdout().flush().unwrap();
                }
            }
            27 => {
                let mut arrow = [0u8; 2];
                io::stdin().read(&mut buf).unwrap();
                arrow[0] = buf[0];
                io::stdin().read(&mut buf).unwrap();
                arrow[1] = buf[0];

                match arrow {
                    [91, 65] => {
                        if !history.is_empty() {
                            clear_line(line.len(), prompt);
                            if idx == 0 {
                                idx = history.len() - 1;
                            } else {
                                idx -= 1;
                            }
                            line = history[idx].clone();
                            print!("{}", line);
                            io::stdout().flush().unwrap();
                        }
                    }
                    [91, 66] => {
                        if !history.is_empty() {
                            clear_line(line.len(), prompt);
                            idx = (idx + 1) % history.len();
                            line = history[idx].clone();
                            print!("{}", line);
                            io::stdout().flush().unwrap();
                        }
                    }
                    _ => {}
                }
            }
            c => {
                line.push(c as char);
                print!("{}", c as char);
                io::stdout().flush().unwrap();
            }
        }
    }

    tcsetattr(STDIN_FILENO, TCSANOW, &original).unwrap();
    line
}

fn clear_line(len: usize, prompt: &str) {
    print!("\r{}\r{}", " ".repeat(prompt.len() + len), prompt);
    io::stdout().flush().unwrap();
}
