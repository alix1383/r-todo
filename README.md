# r-todo

A simple Rust CLI todo application for managing tasks from the terminal.

## Features

- Add new tasks
- Remove tasks
- Toggle task completion state
- List all tasks
- Reset all stored tasks
- Persistent storage in the local app data directory

## Requirements

- Rust 1.80+ (or any recent stable version)
- Cargo

## Installation

```bash
git clone https://github.com/alix1383/r-todo.git
cd r-todo
cargo build --release
```

## Usage

Run the CLI with commands such as:

```bash
cargo run -- add "Write project README"
cargo run -- list
cargo run -- toggle "Write project README"
cargo run -- remove "Write project README"
cargo run -- reset
```

You can also run the built binary directly:

```bash
./target/release/r-todo add "Buy groceries"
```

## Commands

- `add <task_name>`: add a new task
- `remove <task_name>`: remove a task by name
- `toggle <task_name>`: mark a task as done or not done
- `list`: display all tasks
- `reset`: delete all saved tasks

## Data Storage

Tasks are stored in the operating system's local application data directory using the `directories` crate. The app creates a JSON file automatically on first run if it does not already exist.

## Project Structure

```text
r-todo/
├── Cargo.toml
├── crates/
│   ├── cli/
│   │   └── src/
│   └── database/
│       └── src/
└── src/
    └── main.rs
```

## License

This project is provided as-is for personal use and learning.
