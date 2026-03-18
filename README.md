# rush

A Unix shell built from scratch in Rust, as a systems programming learning project.

## Features

- **Command execution** — run any program with arguments
- **Pipes** — chain multiple commands with `|` (e.g. `ls | grep foo | wc -l`)
- **Redirections** — `>` write to file, `>>` append to file, `<` read from file
- **Built-in commands** — `cd`, `pwd`, `exit`
- **Command history** — up/down arrows to cycle through previous commands
- **Dynamic prompt** — shows current working directory

## Structure

```
src/
├── main.rs       — main loop
├── executor.rs   — command execution (single, pipe, redirect, builtins)
└── input.rs      — raw mode input, history navigation
```

## Usage

```
cargo run
```

## Roadmap

- [ ] Combining pipes and redirections (`ls | grep foo > out.txt`)
- [ ] Ctrl+C to cancel current input
- [ ] Tab completion
