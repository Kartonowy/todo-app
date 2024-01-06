use std::env;
use todo::todo;
fn main() {
    let args: Vec<String> =   env::args().collect();
    let command = &args[1];
    println!("Starting up!");
    let mut tasks = todo::read_from_file();
    match command.as_str() {
        "add" => {
            for arg in args[2..].iter() {
                tasks.push(arg.to_string())
            }
            let _ = todo::write_to_file(tasks);
        },
        "list" => {
            todo::print_tasks(tasks);
        },
        
        "complete"|"remove" => {
        }
        _ => {}
    }
}
