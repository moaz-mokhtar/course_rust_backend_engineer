# Milesonte project for Module 1 Rust Essentials: Todo App

## Project idea
- Infinite loop to receive user input and based on user input take action
- Avoid inputs which not supported and display error message

### Features
- Add a new todo item
- Remove a todo item
- Mark a todo item as completed
- View all todo items
- View all completed todo items

### Design
- Data structure for `TodoItem` which represents a todo item:
```rust
struct TodoItem {
    id: u32,
    description: String, 
    is_completed: bool,
}
```
- Methods for `TodoItem`:
```rust
- new(id: u32, description: String) -> TodoItem
- set_completed(self) -> bool
```

- Data structure for `TodoCollection` which represents a list of todo items and operations on it:
```rust
struct TodoCollection(Vec<TodoItem>);
```
- Methods for `TodoCollection`:
```rust
- add_todo(self, description: String) -> TodoItem
- remove_todo(self, id: u32) -> bool
- mark_todo_as_completed(self, id: u32) -> bool
- print_all(self)
- print_all_completed(self)
```

- User input will be commanded in below format:
```shell
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
=> 

```

