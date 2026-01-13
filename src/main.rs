use clap::{Parser, Subcommand};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;

struct TodoItem {
    id: i32,
    task: String,
    completed: bool,
}

impl TodoItem {
    fn complete(&mut self) {
        self.completed = true;
    }

    fn pretty_print(&self) {
        println!(
            "id: {}, task: {}, completed: {}",
            self.id.to_string(),
            self.task,
            self.completed.to_string()
        );
    }

    fn db_print(&self) -> String {
        format!("{}, {}, {}\n", self.id, self.task, self.completed)
    }
}
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new_list() -> Self {
        Self { items: Vec::new() }
    }

    fn new_item(&mut self, task: String) {
        println!("in new_item");
        let next_id = self.items.len() as i32;
        let new_item = TodoItem {
            id: next_id,
            task,
            completed: false,
        };
        self.items.push(new_item);
    }

    fn read_items(&mut self, file_path: &str) -> Result<(), Error> {
        let path = Path::new(file_path);
        if path.exists() {
            let file_contents = File::open(file_path)?;
            let buff = BufReader::new(file_contents);
            for line in buff.lines() {
                let line = line?;
                let mut parts: Vec<&str> = line.split(",").collect();

                let id: i32 = String::from(parts[0]).parse().unwrap();
                let task: String = String::from(parts[1]);
                let completed: bool = String::from(parts[2]) == "true";

                let new_item: TodoItem = TodoItem {
                    id,
                    task,
                    completed,
                };

                self.items.push(new_item);
            }
        }
        Ok(())
    }

    fn complete_item(&mut self, task_id: i32) {
        for item in &mut self.items {
            if item.id == task_id {
                item.complete();
            }
        }
    }

    fn print_items(&self) {
        for item in &self.items {
            item.pretty_print();
        }
    }

    fn save_items(&self, file_path: &str) -> Result<(), Error> {
        let path = Path::new(file_path);
        if path.exists() {
            let mut file = OpenOptions::new()
                .append(true)
                .open(file_path)
                .expect("unable to open file for saving items");
            for item in &self.items {
                file.write_all(item.db_print().as_bytes())
                    .expect("failed writing item to file");
            }
        } else {
            let mut file = File::create(file_path)?;
            for item in &self.items {
                file.write_all(item.db_print().as_bytes())
                    .expect("failed writing item to file");
            }
        }
        Ok(())
    }

    fn delete_item(&mut self, task_id: i32) {
        self.items.retain(|item| item.id != task_id);
    }
}

// receive and action
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    Complete { id: i32 },
    Delete { id: i32 },
    List,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let mut todo_list = TodoList::new_list();

    let file_path = "todos.txt";
    todo_list.read_items(file_path)?;
    match args.command {
        Commands::Add { task } => {
            todo_list.new_item(task);
        }
        Commands::Complete { id } => {
            todo_list.complete_item(id);
        }
        Commands::Delete { id } => {
            todo_list.delete_item(id);
        }
        Commands::List {} => {
            todo_list.print_items();
        }
    }

    todo_list.save_items(file_path)?;
    todo_list.print_items();

    Ok(())
}
