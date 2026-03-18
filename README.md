# rush

A Unix shell built from scratch in Rust, as a systems programming learning project.

## Features

- **Command execution** — run any program with arguments
- **Pipes** — connect commands with `|` (e.g. `ls | grep foo`)
- **Redirections** — `>` write to file, `>>` append to file, `<` read from file
- **Built-in commands** — `cd`, `pwd`, `exit`
- **Command history** — press up/down arrows to cycle through previous commands
- **Dynamic prompt** — shows current working directory

## Usage

```
cargo run
```

## Roadmap

- [ ] Multiple pipes (`ls | grep foo | wc -l`)
- [ ] Tab completion
- [ ] Ctrl+C to cancel input
