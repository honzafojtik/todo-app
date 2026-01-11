use clap::{Parser, Subcommand};
use std::fs::File;
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
}
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new_list() -> Self {
        Self { items: Vec::new() }
    }

    fn new_item(&mut self, task: String) {
        let next_id = self.items.len() as i32;
        let new_item = TodoItem {
            id: next_id,
            task,
            completed: false,
        };
        self.items.push(new_item);
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
    let path = Path::new(file_path);

    if path.exists() {
        let input = File::open(file_path)?;
        let buffered = BufReader::new(input);
        for line in buffered.lines() {
            println!("{}", line?);
        }
    } else {
        let mut output = File::create(file_path)?;
        write!(output, "another line")?;
    }

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

    Ok(())
}
