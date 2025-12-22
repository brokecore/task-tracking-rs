use std::io;
use std::fs;

pub fn create_dir(){
    let create_task_dir = fs::create_dir("./tasks");
    
    if create_task_dir.is_ok() {
        println!("Task folder was created")
    } else {
        println!("Directory failed")
    }
}

pub fn create_tasks(){
    let mut task = String::new();

    let path = "./tasks/task1.json";

    let _ = io::stdin()
        .read_line(&mut task)
        .expect("Failed to read task");

    _ = std::fs::write(path, task);

}

fn main(){
    create_dir();
    create_tasks();
}
