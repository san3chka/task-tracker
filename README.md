# 📝 Task Tracker CLI

A simple and lightweight command-line tool for managing tasks, written in Rust.  
Add, update, delete, list, and mark tasks as done or in-progress — all from your terminal!

---

## 📦 Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/task-tracker-cli.git
   cd task-tracker-cli
   ```

2. **Build the project (release mode):**
   ```bash
   cargo build --release
   ```

3. **Run the executable:**
   ```bash
   ./target/release/task_tracker
   ```

   > Replace `task_tracker` with the actual name of your binary if it's different.

---

## 🚀 Usage

All commands are available through the `task_tracker` binary.

### ✅ Add a new task
```bash
task_tracker add "Finish the Rust project"
```

### 🔄 Update a task description
```bash
task_tracker update --id 1 --description "Update the Rust project task"
```

### ❌ Delete a task
```bash
task_tracker delete --id 1
```

### 🟡 Mark a task as in progress
```bash
task_tracker mark-in-progress 1
```

### ✅ Mark a task as done
```bash
task_tracker mark-done 1
```

### 📋 List all tasks
```bash
task_tracker list
```

### 📋 List tasks by status
```bash
task_tracker list --status done
```

Available statuses: `todo`, `in-progress`, `done`.

---

## 📁 Task File Format

Tasks are stored in JSON format in the file specified by the `FILE_PATH` environment variable.  
Each task includes:

- `id`: unique numeric identifier  
- `description`: the task text  
- `status`: task status (`todo`, `in-progress`, or `done`)  
- `created_at`: creation timestamp  
- `updated_at`: last updated timestamp  

---

## 🛠️ Dependencies

- `clap` – Command-line argument parser  
- `serde` / `serde_json` – JSON serialization  
- `dotenv` – Environment variable management  
- `anyhow` – Flexible error handling  
- `chrono` – Date/time utilities  

---

## ✅ Roadmap Tasks

This CLI is part of the [roadmap.sh](https://roadmap.sh/) Rust project ideas.  
It implements a basic CRUD task manager with additional status management features.

---

## 📄 License

This project is licensed under the MIT License.  
Copyright (c) 2025 **san3chka**

Permission is hereby granted, free of charge, to any person obtaining a copy  
of this software and associated documentation files (the "Software"), to deal  
in the Software without restriction, including without limitation the rights  
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell  
copies of the Software, and to permit persons to whom the Software is  
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in  
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR  
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,  
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE  
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER  
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,  
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN  
THE SOFTWARE.
