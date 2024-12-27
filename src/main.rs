use std::{env, task};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::ptr::read;

#[derive(Debug)]
struct Task {
    task_name: String,
    is_done: i32, 
}

fn add(task: &str, is_done: i32) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("task.txt")
        .expect("Unable to open the file");

    writeln!(file, "{} {}", task, is_done).expect("Unable to write to the file");
}

fn read_tasks() -> Vec<Task> {
    let content = fs::read_to_string("task.txt").unwrap_or_else(|_| "".to_string());
    content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let task_name = parts[0..parts.len() - 1].join(" "); 
                let is_done = parts.last()?.parse::<i32>().ok()?;
                Some(Task { task_name, is_done })
            } else {
                None
            }
        })
        .collect()
}

fn display_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks found!");
    } else {
        println!("Your To-Do List:");
        for (index, task) in tasks.iter().enumerate() {
            println!(
                "{}. [{}] {}",
                index + 1,
                if task.is_done == 1 { "x" } else { " " },
                task.task_name
            );
        }
    }
}


fn remove(task_name :String) {
    let  tasks = read_tasks();
    let total_length = tasks.len();
    let filtered_task:Vec<Task> = tasks 
                        .into_iter()
                        .filter(|task| task.task_name != task_name)
                        .collect();
    if filtered_task.len() <total_length{
        println!("Task {} removed Successfully",task_name);
        let mut file = fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open("task.txt")
                        .expect("The file Cannot be opened");
        for task in filtered_task{
            writeln!(file, "{} {}", task.task_name, task.is_done).expect("Unable to write to the file");
        }
    }else{
        println!("Task {} is not found",task_name);
    }
}

fn mark_as_done(task_name:String){
    let mut  tasks = read_tasks();
    let mut found = 0;
    for  task in &mut tasks{
        if task.task_name == task_name{
            found = 1;
            task.is_done = 1;
        }
    }
    if found == 1{
        println!("Task {} marked Successfully",task_name);
        let mut file = fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open("task.txt")
                        .expect("The file Cannot be opened");
        for task in tasks{
            writeln!(file, "{} {}", task.task_name, task.is_done).expect("Unable to write to the file");
        }
    }else{
        println!("There is no task")
    }

}
fn main() {
    println!("Welcome to this To-Do app. The basic operations are:");
    println!("1. add <task>");
    println!("2. list");
    println!("3. remove <task>");
    println!("4. done <task>");
    println!("Just type `cargo run` followed by the operation keyword and task.");

    let args: Vec<String> = env::args().collect();
    let arguments = args.get(1);

    match arguments {
        Some(value) => match value.as_str() {
            "add" => {
                let task = args.get(2);
                match task {
                    Some(task_name) => {
                        println!("Adding the new task: {}", task_name);
                        add(task_name, 0); // Add task with is_done = 0
                    }
                    None => {
                        println!("Please provide a task to add.");
                    }
                }
            }
            "list" => {
                let tasks = read_tasks();
                display_tasks(&tasks);
            }
            "remove" => {
                let task = args.get(2);
                match task {
                    Some(task_name) => {
                        println!("removing the  task: {}", task_name);
                        remove(task_name.to_string());
                    }
                    None => {
                        println!("Please provide a task to add.");
                    }
                }
                
            }
            "done" => {
                let task = args.get(2);
                match task {
                    Some(task_name) => {
                        println!("Marked the  task: {}", task_name);
                        mark_as_done(task_name.to_string());
                    }
                    None => {
                        println!("Please provide a task to add.");
                    }
                }
            }
            _ => {
                println!("Invalid operation. Use `add`, `list`, `remove`, or `done`.");
            }
        },
        None => {
            println!("Please provide an operation (e.g., `add`, `list`, `remove`, `done`).");
        }
    }
}
