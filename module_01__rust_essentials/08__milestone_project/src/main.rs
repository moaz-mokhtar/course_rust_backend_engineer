use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let help_content = r#"
┌───────────────────────────────────┐
│        Welcome to Todo App        │
├───────────────────────────────────┤
│ How to use:                       │
│                                   │
│ **Add a new todo item**:          │
│   Type `+ NEW_TASK_DESCRIPTION`   │
│   Example: `+ Learn Rust`         │
│                                   │
│ **Remove a todo item**:           │
│   Type `- <TODO_ID>`              │
│   Example: `- 1`                  │
│                                   │
│ **Mark a todo item as completed**:│
│   Type `+ <TODO_ID>`              │
│   Example: `+ 1`                  │
│                                   │
│ **View all todo items**:          │
│   Type `a` or `A`                 │
│                                   │
│ **View all completed todo items**:│
│   Type `d` or `D`                 │
|                                   │
│ **Help**:                         │
│   Type `h` or `H`                 │
│                                   │
│ **Quit**:                         │
│   Type `q` or `Q`                 │
└───────────────────────────────────┘
"#;
    print!("{help_content}");

    let mut todo_collection = TodoCollection::new();
    todo_collection.add_todo("Learn Rust".to_string());
    todo_collection.add_todo("Learn Python".to_string());

    loop {
        print!("=> ");
        let mut input = String::new();
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        io::stdin().read_line(&mut input)?;

        input = input.trim().to_string();

        // Skip if user press Enter without typing anything
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let action = parts.get(0).unwrap_or(&"").to_lowercase();
        let statement = parts.get(1..).unwrap_or(&[""]).join(" ");

        match action.as_str() {
            "+" => {
                if statement.parse::<u32>().is_ok() {
                    // mark todo item as completed
                    if todo_collection.mark_todo_as_completed(statement.parse::<u32>().unwrap()) {
                        println!("=> DONE: mark todo item #{} as completed", statement);
                    }
                } else {
                    // add new todo item
                    let new_todo = todo_collection.add_todo(statement.to_string());
                    println!("=> DONE: added new todo {:?}", new_todo);
                }
            }
            "-" => {
                // remove todo item by Id ops
                if let Ok(id) = statement.trim().parse::<u32>() {
                    todo_collection.remove_todo(id);
                } else {
                    println!("=> Unknown command: '{}'. Kindly try again...", input);
                }
            }
            "a" => {
                // View all todo items ops
                todo_collection.print_all();
            }
            "d" => {
                // View all completed todo items ops
                todo_collection.print_all_completed();
            }
            "h" => {
                print!("{help_content}");
            }
            "q" => {
                return Ok(());
            }
            _ => {
                println!("=> Unknown command: '{}'. Kindly try again...", input);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct TodoItem {
    id: u32,
    description: String,
    is_completed: bool,
}

impl TodoItem {
    fn new(id: u32, description: String) -> TodoItem {
        TodoItem {
            id,
            description,
            is_completed: false,
        }
    }

    fn set_completed(&mut self) -> bool {
        self.is_completed = true;
        true
    }

    fn print(&self) {
        let status = if self.is_completed { "DONE" } else { "Not" };
        println!(
            "  | #{:>2} |  {:<4} | {:<10}",
            self.id, status, self.description
        );
    }
}

struct TodoCollection(Vec<TodoItem>);

impl TodoCollection {
    fn new() -> TodoCollection {
        TodoCollection(Vec::new())
    }

    fn add_todo(&mut self, description: String) -> TodoItem {
        let new_todo = TodoItem::new(self.0.len() as u32 + 1, description);
        self.0.push(new_todo.clone());
        new_todo
    }

    fn remove_todo(&mut self, id: u32) -> bool {
        self.0.retain_mut(|item| item.id != id);
        true
    }

    fn mark_todo_as_completed(&mut self, id: u32) -> bool {
        match self.0.iter_mut().find(|item| item.id == id) {
            Some(item) => {
                item.set_completed();
                true
            }
            None => false,
        }
    }

    fn print_all(&self) {
        println!("  =================");
        println!("  Todo List (all):");
        self.0.iter().for_each(|item| item.print());
        println!("  =================");
    }

    fn print_all_completed(&self) {
        println!("  =================");
        println!("  Todo List (completed):");
        self.0
            .iter()
            .filter(|item| item.is_completed)
            .for_each(|item| item.print());
        println!("  =================");
    }
}
