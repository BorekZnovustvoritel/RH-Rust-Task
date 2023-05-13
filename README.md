# To-do list in Rust

## How to build:
- Make sure you have Rust installed.
- Clone the repo and `cd` into it.
- Run `cargo build`.
- Application then can be run with `./target/debug/main`.

## Run options
### List
- Shows all tasks that are present in the json file.
- Run with `./target/debug/main list`.

### Add
- Adds the task to the other tasks and persists all of them.
- Run with `./target/debug/main add task` (or replace the word `task` with any other word or a phrase).
- Even unquoted blank spaces are a viable option for the task names.

### Complete
- Marks a task as completed.
- Run with `./target/debug/main complete 2` (or replace `2` with any other available index).
- Check available task indices via the `list` subcommand.
