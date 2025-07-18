use std::fs::OpenOptions;
use std::io::Write;

use crate::model::Todo;
use crate::storage::{load_tasks, save_tasks};

use colored::*;


pub fn add_task(title: String, description: String, time: String) {
    let mut tasks = load_tasks();
    let new_task = Todo {
        id: tasks.len() as u32 + 1,
        title,
        description,
        time,
    };

    tasks.push(new_task.clone());
    save_tasks(&tasks).unwrap();

    println!("Task added: {:?}", new_task);
}

pub fn list_tasks() {
    let tasks = load_tasks();

    if !tasks.is_empty() {
        for task in tasks {
        // println!("ID: {} | Title: {} | Description: {} | Time: {}", task.id, task.title, task.description, task.time);

        println!(
            "ID: {} | Title: {} | Description: {} | Time: {}",
            task.id.to_string().cyan(),
            task.title.cyan(),
            task.description.cyan(),
            task.time.cyan()
        );
    }
    } else {
        println!("No tasks found.");
    }
  
}

pub fn update_task(id: u32, title: Option<String>, description: Option<String>, time: Option<String>) {
    let mut tasks = load_tasks();
     let mut task_updated = false;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        if let Some(t) = title { task.title = t; }
        if let Some(d) = description { task.description = d; }
        if let Some(tm) = time { task.time = tm; }

        println!("Task updated: {:?}", task);
         task_updated = true;
    } else {
        println!("Task not found");
    }

    if task_updated {
        save_tasks(&tasks).unwrap();
    }
}

pub fn delete_task(id: u32) {
    let mut tasks = load_tasks();
    let initial_len = tasks.len();
    tasks.retain(|t| t.id != id);

        for (i, task) in tasks.iter_mut().enumerate() {
        task.id = (i + 1) as u32;
    }

    if tasks.len() < initial_len {
        save_tasks(&tasks).unwrap();
        println!("Task with ID {} deleted.", id);
    } else {
        println!("Task not found");
    }
}

pub fn export_tasks(file_name: String) {
    let tasks = load_tasks();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name.clone())
        .expect("Unable to open export file");

    writeln!(file, "ID, Title, Description, Time").unwrap();

    for task in tasks {
        writeln!(
            file,
            "{},{},{},{}",
            task.id, task.title, task.description, task.time
        )
        .unwrap();
    }

    println!("Tasks exported to file: {}", file_name);
}
