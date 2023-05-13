use serde::{Deserialize, Serialize};
use std::{fs::{File, self}, io::Write};


#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    message: String,
    completed: bool
}

impl Task {
    pub fn new(id: u32, message: String) -> Task {
        Task { id: id, message: message, completed: false }
    }
    
}

fn get_elements_after_idx(vector: Vec<String>, target_idx: usize) ->Vec<String> {
    let iterator = vector.iter();
    let mut ans: Vec<String> = Vec::new();
    for (idx, element) in iterator.enumerate() {
        if idx <= target_idx {
            continue;
        }
        ans.push(element.to_string());
    }
    return ans;
}

fn concat_strings(strings: Vec<String>) -> String {
    let ans: String = strings.join(" ");
    return ans;
}

fn save(arr: Vec<Task>) {
    let data = match serde_json::to_string(&arr){
        Err(why) => panic!("Crikey, I could not serialize data! Message: {why}", why=why),
        Ok(data) => data,
    } ;
    let mut file = match File::create("tasks.json") {
        Err(why) => panic!("Crikey, I could not write into the save file! Message: {why}", why=why),
        Ok(file) => file,
    };
    let _res = write!(file, "{}", data);
}

fn load() -> Vec<Task> {
    let input: String = match fs::read_to_string("tasks.json") {
        Err(_why) => "[]".to_owned(),
        Ok(input) => input,
    };
    return serde_json::from_str(&input).unwrap();
}

fn main() {
    let mut tasks: Vec<Task> = load();
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Crikey, provide me with some arguments. Available options are:\nlist\nadd task_name\ncomplete task_id");
        std::process::exit(-1);
    }
    let mode: i8 = match args[1].as_str(){
        "list" => 1,
        "add" => 2,
        "complete" => 3,
        _ => 0
    };

    if mode == 0 {
        eprintln!("Crikey, I did not understand the command.");
        std::process::exit(-1);
    }
    else if mode == 1 {
        let iter = tasks.iter();
        println!("Completed tasks:");
        for task in iter {
            if task.completed {
                println!("{id}: [x] {text}", id=task.id, text=task.message);
            }
        }
        let iter = tasks.iter();
        println!("Uncompleted tasks:");
        for task in iter {
            if !task.completed {
                println!("{id}: [ ] {text}", id=task.id, text=task.message);
            }
        }
    }
    else {
        if args.len() < 3 {
            eprintln!("Crikey, too few arguments for that option.");
            std::process::exit(-1);
        }
        let other_text = concat_strings(get_elements_after_idx(args, 1));
        if mode == 2 {
            tasks.push(Task::new(tasks.len() as u32, other_text));
        }
        else {
            let task_id: usize = other_text.parse().expect("Crikey, Invalid task ID.");
            if task_id >= tasks.len() {
                eprintln!("Crikey, I could not find that task.");
                std::process::exit(-1);
            }
            tasks[task_id].completed = true;

        }
    }
    save(tasks);
    
}
