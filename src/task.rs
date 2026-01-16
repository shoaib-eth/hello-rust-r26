use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub task_name: String,
    pub task_description: String,
    pub task_complete: String,
}

pub fn load_tasks() -> Vec<Task> {
    let mut tasks: Vec<Task> = vec![];

    let mut file = File::open("tasks.csv").unwrap_or_else(|_| File::create("tasks.csv").unwrap());
    println!("File:{:?}", file);
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            println!("contents:{:?}", contents);
            let mut reader = Reader::from_reader(contents.as_bytes());
            println!("reader:{:?}", reader);
            for result in reader.deserialize() {
                let task: Task = result.unwrap();
                println!("task:{:?}", task);
                tasks.push(task);
                println!("tasks:{:?}", tasks);
            }
        }
        Err(_) => (),
    }
    tasks
}
pub fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create("tasks.csv").unwrap();
    let mut writer = Writer::from_writer(file);

    for task in tasks {
        writer.serialize(task).unwrap();
    }
}
