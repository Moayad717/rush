use std::fs;
use std::process::{Command, Stdio};

pub fn run_single(input: &str) {
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

pub fn run_pipe(input: &str) {
    let commands: Vec<&str> = input.split('|').collect();
    let mut prev_stdout = None;
    let mut children = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let is_last = i == commands.len() - 1;

        let mut command = Command::new(parts[0]);
        command.args(&parts[1..]);

        if let Some(stdout) = prev_stdout.take() {
            command.stdin(Stdio::from(stdout));
        }

        if !is_last {
            command.stdout(Stdio::piped());
        }

        let mut child = match command.spawn() {
            Ok(c) => c,
            Err(_) => {
                eprintln!("rush: {}: command not found", parts[0]);
                return;
            }
        };

        prev_stdout = child.stdout.take();
        children.push(child);
    }

    for child in &mut children {
        child.wait().expect("failed to wait for process");
    }
}

pub fn run_redirect(input: &str) {
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

pub fn is_builtin(input: &str) -> bool {
    let parts: Vec<&str> = input.split_whitespace().collect();
    parts[0] == "cd" || parts[0] == "pwd"
}

pub fn run_builtin(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[0] {
        "cd" => {
            let path = if parts.len() < 2 {
                std::env::var("HOME").unwrap_or_else(|_| "/".to_string())
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
