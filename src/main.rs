use std::env::{self};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    contents: String,
}

impl Task {
    fn new(content: &str) -> Task {
        return Task { contents: content.to_string()}
    }
    fn print(&self, index: usize) {
        println!("{}: {}", index + 1, self.contents)
    }
}

fn print_tasks(tasks: Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        task.print(i)
    }
}

fn write_to_file(tasks: Vec<Task>) -> Result<(), std::io::Error>{
    let serialized_data = serde_json::to_string(&tasks).unwrap();
    

    let _ = fs::write("./todo.json", serialized_data);
    Ok(())
}
fn read_from_file() -> Vec<Task>{
    let binding = fs::read_to_string("./todo.json");
    let file_contents =  binding.as_ref();
    let deserial = serde_json::from_str(file_contents.expect("Coudnt read")).expect("Coudnt convert from json");
    deserial
}
fn main() {
    let args: Vec<String> =   env::args().collect();
    let command = &args[1];
    println!("Starting up!");
    let mut tasks = read_from_file();
    match command.as_str() {
        "add" => {
            for arg in &args[2..] {
                tasks.push(Task::new(arg));
            }
            let _ = write_to_file(tasks);
        },
        "list" => {
            print_tasks(tasks);
        },
        "complete"|"remove" => {
        }
        _ => {}
    }
}
