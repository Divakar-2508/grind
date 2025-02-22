pub mod db;
pub mod error;
pub mod models;

use std::{
    io::{self, Write},
    path::PathBuf,
};

use db::TaskBase;
use models::Task;

//testing
fn eval(db: &TaskBase) {
    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let parts: Vec<&str> = buffer.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        println!("{:?}", parts);
        match *parts.first().unwrap() {
            "show" => {
                let res = db.retrieve_tasks();
                match res {
                    Ok(tasks) => tasks.iter().for_each(|s| println!("{:?}", s)),
                    Err(err) => println!("{:?}", err),
                }
            }
            "add" => {
                let mut task = Task {
                    name: parts[1].to_string(),
                    ..Default::default()
                };
                println!("{:?}", db.create_task(&mut task));
            }
            "count" => {
                println!("{:?}", db.task_done_count());
            }
            "done" => {
                println!("{:?}", db.mark_task_done(parts[1].parse::<u32>().unwrap()));
            }
            "delete" => {
                println!("{:?}", db.delete_task(parts[1].parse::<u32>().unwrap()));
            }
            "last" => {
                println!("{:?}", db.last_login_date());
            }
            _ => (),
        }

        buffer.clear();
    }
}
