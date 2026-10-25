use app_data::AppData;
use serde::{Deserialize, Serialize};

use std::{
    fs,
    time::{self, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task_name: String,
    is_done: bool,
    time: u64,
}

pub fn check_data_dir() {
    AppData::default().ensure_data_dir().unwrap();
}

fn write(tasks: &Vec<Task>) {
    let data = serde_json::to_string(&tasks).unwrap();

    fs::write(get_file_path(), data).unwrap()
}

fn read() -> Vec<Task> {
    let data_buf = fs::read_to_string(get_file_path()).unwrap();

    let data: Vec<Task> = serde_json::from_str(&data_buf).unwrap();

    data
}

fn get_file_path() -> String {
    AppData::default()
        .get_file_path("data.json")
        .unwrap()
        .into_string()
        .unwrap()
}

pub fn reset_data() {
    fs::write(get_file_path(), "[]").unwrap();
}

pub fn print_task_list() {
    let tasks: Vec<Task> = read();

    for task in tasks {
        println!(
            "{} is {}, added at {}",
            task.task_name, task.is_done, task.time
        )
    }
}

pub fn add_task(task_name: &String) {
    let name = task_name.to_string();

    let mut tasks: Vec<Task> = read();
    let time = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    tasks.push(Task {
        task_name: name,
        is_done: false,
        time: time,
    });

    write(&tasks);

    println!("{} task, added!", task_name)
}

pub fn remove_task(task_name: &String) {
    let mut tasks: Vec<Task> = read();

    if let Some(index) = tasks
        .iter()
        .position(|x| x.task_name.to_lowercase() == task_name.to_lowercase())
    {
        tasks.remove(index);
        write(&tasks);
    } else {
        println!("task not found!")
    }
}

pub fn toggle_task(task_name: &String) {
    let mut tasks: Vec<Task> = read();

    if let Some(index) = tasks
        .iter()
        .position(|x| x.task_name.to_lowercase() == task_name.to_lowercase())
    {
        tasks[index].is_done = !tasks[index].is_done;
        write(&tasks);
        println!("task status toggled")
    } else {
        println!("task not found!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
