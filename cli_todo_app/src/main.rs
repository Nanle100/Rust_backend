
// use std::io::{self, Write, BufReader}; 
// use std::fs::File;
// use std::io::BufWriter;

// fn main() -> Result<(), io::Error> {
//     println!("  ======  Welcome to your Todo Application. Kindly select one of the following:  ====== ");

//     //let mut count: u32 = 0;
//     let mut todo_list:Vec<Todo> = vec![];

//     loop {

//         println!(" ======  1: Add Task,  2: List Tasks, 3: Update Tasks, 4: Delete Tasks, 5: Quit ====== ");


//         //Creates a mutable string to store user input.
//         let mut choice = String::new();

//      //Reads user input from the terminal and stores it in choice.
//      io::stdin().read_line(&mut choice).unwrap();

//      //Removes any leading/trailing whitespace from the input.
//      let choice = choice.trim();

//      match choice {

//          "1" => {
//             print!("Write task name: ");

//             io::stdout().flush().unwrap();
//             let mut task_name = String::new();
//             io::stdin().read_line(&mut task_name).unwrap();
//             let task_name = task_name.trim().to_string();


//             println!("Write task description: ");

//             io::stdout().flush().unwrap();
//             let mut task_desc = String::new();
//             io::stdin().read_line(&mut task_desc).unwrap();
//             let task_desc = task_desc.trim().to_string();

//             println!("Fix task time");

//             io::stdout().flush().unwrap();
//             let mut task_time = String::new();
//             io::stdin().read_line(&mut task_time).unwrap();
//             let task_time = task_time.trim().to_string();

//             let todo = Todo {
//                 id: todo_list.len() as u32 + 1,
//                 title: task_name,
//                 description: task_desc,
//                 time: task_time,
//             };

//            // let todo_id:u32 = todo.id;
//              todo_list.push(todo.clone());
//             // count += todo_id;

//              //  Serialize Vec<Todo> to JSON and write to file
//            let file = File::create("todos/tasks.json")?;
//            let writer = BufWriter::new(file);
//           serde_json::to_writer(writer, &todo_list)?;


//             println!(" == {:?} added to the list of todos ==", &todo);

            

//          },

//          "2" => {

//                // Read from file and deserialize back into Vec<Todo>
//          let file = File::open("todos/tasks.json")?;
//          let reader = BufReader::new(file);
//          let deserialized: Vec<Todo> = serde_json::from_reader(reader).unwrap();

//             println!("{:?}", deserialized);
//          },

//          "3" => {
           

//             loop {

//                let file = File::open("todos/tasks.json")?;
//             let reader = BufReader::new(file);
//             let mut deserialized: Vec<Todo> = serde_json::from_reader(reader).unwrap();

//                println!("Please enter task id: ");
//                io::stdout().flush().unwrap();
//                let mut task_id = String::new();
//                io::stdin().read_line(&mut task_id).unwrap();
//                // let task_id = task_id.trim().to_string();

//                let id: u32 = match task_id.trim().parse() {
//                   Ok(num) => num,
//                   Err(_) => {
//                       println!("❌ Please enter a valid number.");
//                       return Ok(());                  }
//               };



//                for _tasks in deserialized.iter_mut() {
//                   if _tasks.id == id {

//                      println!(" ======  Please enter 1: Edit title, 2: Edit description, 3: Edit time, 4: Cancel ====== ");

//                       //Creates a mutable string to store user input.
//                      let mut choice = String::new();

//                       //Reads user input from the terminal and stores it in choice.
//                        io::stdin().read_line(&mut choice).unwrap();
   
//                     //Removes any leading/trailing whitespace from the input.
//                     let choice = choice.trim();


//                     match choice {
//                      "1" => {

//                         println!("Welcome to editing title");
//                         let mut new_title = String::new();
//                         io::stdin().read_line(&mut new_title).unwrap();
//                         let new_title = new_title.trim();

//                         _tasks.title = new_title.trim().to_string();

                        

//                      }

//                      &_ => todo!()


//                     }

//                   } 



//                }



//             }

            

            
           
//          }


//          &_ => todo!()


mod cli;
mod handler;
mod model;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use handler::*;


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title, description, time } => {
            add_task(title, description, time);
        }
        Commands::List => {
            list_tasks();
        }
        Commands::Update { id, title, description, time } => {
            update_task(id, title, description, time);
        }
        Commands::Delete { id } => {
            delete_task(id);
        }
         Commands::Export { file } => {
               export_tasks(file);
         }
    }
}
