use std::fs;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use termios::*;

const STDIN_FILENO: i32 = 0;

fn main() {
    let mut history: Vec<String> = Vec::new();

    loop {
        let prompt = format!("rush: {} > ", std::env::current_dir().unwrap().display());
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush");

        let input = read_input(&history, &prompt);

        history.push(input.clone());

        let input = input.trim();

        if input.is_empty() {
            continue;
        } else if input == "exit" {
            break;
        } else if input.contains('|') {
            run_pipe(input);
        } else if input.contains('<') || input.contains('>') {
            run_redirect(input);
        } else if is_builtin(input) {
            run_builtin(input);
        } else {
            run_single(input);
        }
    }
}

fn run_single(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    let mut child = match Command::new(parts[0]).args(&parts[1..]).spawn() {
        Ok(c) => c,
        Err(_) => {
            eprintln!("rush: {}: command not found", parts[0]);
            return;
        }
    };

    child.wait().expect("failed to wait for process");
}

fn run_pipe(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    let pipe_pos = match parts.iter().position(|s| *s == "|") {
        Some(i) => i,
        None => return,
    };

    let left = &parts[..pipe_pos];
    let right = &parts[pipe_pos + 1..];

    let mut child1 = match Command::new(left[0])
        .args(&left[1..])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            eprintln!("rush: {}: command not found", left[0]);
            return;
        }
    };

    let stdout1 = child1.stdout.take().unwrap();

    let mut child2 = match Command::new(right[0])
        .args(&right[1..])
        .stdin(Stdio::from(stdout1))
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            eprintln!("rush: {}: command not found", right[0]);
            return;
        }
    };

    child1.wait().expect("failed to wait for process");
    child2.wait().expect("failed to wait for process");
}

fn run_redirect(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    let dir_pos = match parts
        .iter()
        .position(|s| *s == ">" || *s == ">>" || *s == "<")
    {
        Some(pos) => pos,
        None => return,
    };

    let left = &parts[..dir_pos];
    let right = &parts[dir_pos..];

    if parts[dir_pos] == "<" {
        let file = match fs::File::open(right[1]) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("rush: {}: no such file", right[1]);
                return;
            }
        };
        let mut child = match Command::new(left[0])
            .args(&left[1..])
            .stdin(Stdio::from(file))
            .spawn()
        {
            Ok(c) => c,
            Err(_) => {
                eprintln!("rush: {}: command not found", left[0]);
                return;
            }
        };
        child.wait().expect("failed to wait");
        return;
    }

    let file = match parts[dir_pos] {
        ">" => fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(right[1]),
        ">>" => fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(right[1]),
        _ => return,
    };

    let file = match file {
        Ok(f) => f,
        Err(_) => return,
    };

    let mut child = match Command::new(left[0])
        .args(&left[1..])
        .stdout(Stdio::from(file))
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            eprintln!("rush: {}: command not found", left[0]);
            return;
        }
    };

    child.wait().expect("failed to wait");
}

fn read_input(history: &Vec<String>, prompt: &str) -> String {
    let orignal = Termios::from_fd(STDIN_FILENO).unwrap();
    let mut termios = orignal.clone();
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

    tcsetattr(STDIN_FILENO, TCSANOW, &orignal).unwrap();
    line
}

fn clear_line(len: usize, prompt: &str) {
    print!("\r{}\r{}", " ".repeat(prompt.len() + len), prompt);
    io::stdout().flush().unwrap();
}

fn is_builtin(input: &str) -> bool {
    let input: Vec<&str> = input.split_whitespace().collect();
    if input[0] == "cd" || input[0] == "pwd" {
        return true;
    }
    false
}

fn run_builtin(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[0] {
        "cd" => {
            let path = if parts.len() < 2 {
                std::env::var("Home").unwrap_or_else(|_| "/".to_string())
            } else {
                parts[1].to_string()
            };

            if let Err(_) = std::env::set_current_dir(&path) {
                eprintln!("rush: cd {}: no such file or directory", path);
            }
        }
        "pwd" => match std::env::current_dir() {
            Ok(r) => println!("{}", r.display()),
            Err(_) => eprintln!("rush: pwd: failed"),
        },
        _ => {}
    }
}
