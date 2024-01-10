pub mod todo {
    use home;
    use std::fs;
    pub fn print_tasks(tasks: Vec<String>) {
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i, task)
        }
    }

    pub fn write_to_file(tasks: Vec<String>) {
        let mut data = String::new();
        for text in tasks {
            data.insert_str(data.len(), &(text + "\n"));
        }

        let _ = fs::write(format!("{}{}", home::home_dir().unwrap().display(), "/.config/.todo"), data);
    }

    pub fn read_from_file() -> Vec<String> {
        let stuff = match fs::read_to_string(format!("{}{}", home::home_dir().unwrap().display(), "/.config/.todo")) {
            Ok(x) => x.trim().to_string(),
            Err(_) => String::new(),
        };
        let mut list: Vec<String> = vec![];
        for text in stuff.split("\n") {
            list.push(text.to_string());
        }
        list
    }
    pub fn remove_from_list(index: &str, iteration: usize) {
       let mut list = read_from_file();
       list.remove(index.parse::<usize>().unwrap() - iteration);
       write_to_file(list);
    }
}
