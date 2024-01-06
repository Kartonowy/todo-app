pub mod todo {
    use std::fs;
    pub fn print_tasks(tasks: Vec<String>) {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i, task)
        }
    }

    pub fn write_to_file(tasks: Vec<String>) -> Result<(), std::io::Error>{
        let mut data = String::from("");
        for text in tasks {
            println!("{}", &text);
            data.insert_str(0, &(text + "\n"));
        }

        let _ = fs::write("./todo.txt", data);
        Ok(())
    }

    pub fn read_from_file() -> Vec<String> {
        let stuff = match fs::read_to_string("./todo.txt") {
            Ok(x) => x.trim().to_string(),
            Err(_) => String::new(),
        };
        let mut list: Vec<String> = vec![];
        println!("stuff: \n{} \nend stuff", stuff);
        for text in stuff.split("\n") {
            list.push(text.to_string());
        }
        list
    }
}
