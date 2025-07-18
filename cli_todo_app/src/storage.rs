use crate::model::Todo;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

const FILE_PATH: &str = "todos/tasks.json";

pub fn load_tasks() -> Vec<Todo> {
    if !Path::new(FILE_PATH).exists() {
        fs::create_dir_all("todos").unwrap();
        save_tasks(&vec![]).unwrap();
    }

    let file = File::open(FILE_PATH).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_default()
}

pub fn save_tasks(tasks: &Vec<Todo>) -> Result<(), std::io::Error> {
    let file = File::create(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?;
    Ok(())
}
