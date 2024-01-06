pub mod todo {
    use std::fs;
    use std::collections::HashMap;
    pub fn print_tasks(tasks: HashMap<usize, String>) {
        for (i, task) in tasks {
            println!("{}: {}", i, task)
        }
    }

    pub fn write_to_file(tasks: HashMap<usize, String>) -> Result<(), std::io::Error>{
        let serialized_data = serde_json::to_string(&tasks).unwrap();


        let _ = fs::write("./todo.json", serialized_data);
        Ok(())
    }
    pub fn read_from_file() -> HashMap<usize, String>{
        let binding = match fs::read_to_string("./todo.json") {
            Ok(x) => x,
            Err(_) => String::new(),
        };
        let file_contents: &str =  binding.as_ref();
        let deserial: HashMap<usize, String> = match serde_json::from_str(file_contents) {
            Ok(x) => x,
            Err(_) => HashMap::new()
        }; 
        deserial
    }
}
