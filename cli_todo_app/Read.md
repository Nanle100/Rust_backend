# 📝 CLI To-Do App in Rust

A command-line based To-Do list application written in Rust using the [`clap`](https://docs.rs/clap/latest/clap/) crate for argument parsing. This app allows users to manage daily tasks with a simple and intuitive interface.

---

## ✨ Features

- ✅ Add new tasks
- 📋 List all tasks
- ✏️ Edit existing tasks
- ❌ Delete tasks
- ☑️ Mark tasks as completed
- 📁 Export tasks to a table-formatted `.txt` file
- 🎨 Colored CLI output for better readability
- 💻 Cross-platform support (Windows, Linux, macOS)

---

## 🛠 Installation

### 1. Clone the repository:

```bash
git clone https://github.com/yourusername/cli_todo_app.git
cd cli_todo_app
cargo build --release
cargo run -- <COMMAND>
```
###   Usage
# 1 . cli_todo_app <COMMAND> [ARGS]

# CLI Todo App

## Commands

| Command | Description |
|---------|-------------|
| `add` | Add a new task |
| `list` | View all tasks |
| `edit` | Edit a task by ID |
| `delete` | Delete a task by ID |
| `complete` | Mark a task as completed |
| `export` | Export tasks to a `.txt` file |

## Usage Examples

```bash
# Add a new task
cargo run -- add --title "Read Bible" --description "Genesis 1-5" --time "7:00AM"

# List all tasks
cargo run -- list

# Mark task as completed
cargo run -- complete --id 2

# Export tasks to file
cargo run -- export --file tasks.txt
```
### Dependencies
clap – Command-line argument parser

serde – For serializing task data

serde_json – To store tasks in JSON

prettytable – Exporting table view to file (optional)


### License
#This project is open source and available under the MIT License.

yaml
Copy
Edit



