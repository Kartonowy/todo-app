use std::env;
use todo::todo;
fn main() {
    let args: Vec<String> =   env::args().collect();
    if args.len() < 2 {
        println!("Todo v. 0.1.0 \ncommands: \nadd, list, remove ");
        std::process::exit(69);
    }
    let command = &args[1];
    let mut tasks = todo::read_from_file();
    match command.as_str() {
        "add" => {
            for arg in args[2..].iter() {
                tasks.push(arg.to_string())
            }
            let _ = todo::write_to_file(tasks);
            println!("Added tasks! \n");
        },
        "list" => {
            println!("Here are your tasks: \n");
            todo::print_tasks(tasks);
        },
        
        "complete"|"remove" => {
            println!("Removed tasks");
            for (i, arg) in args[2..].iter().enumerate() {
                todo::remove_from_list(arg, i)
            }
        }
        _ => {}
    }
}
