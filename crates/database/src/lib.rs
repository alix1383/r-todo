use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    time::{self, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task_name: String,
    is_done: bool,
    time: u64,
}

fn get_base_dir() -> PathBuf {
    let base_dir = ProjectDirs::from("ir", "sekai", "r-todo").unwrap();
    let is_dir = base_dir.data_local_dir().is_dir();

    if !is_dir {
        fs::create_dir(&base_dir.data_local_dir()).unwrap();
    }

    base_dir.data_local_dir().join("data.json")
}

pub fn check_data_file_exists() {
    let path = get_base_dir();

    if !path.is_file() {
        fs::write(path, "[]\n\n").unwrap()
    }
}

fn write(tasks: &Vec<Task>) {
    let path = get_base_dir();
    let data = serde_json::to_string(&tasks).unwrap();

    fs::write(path, data).unwrap()
}

fn read() -> Vec<Task> {
    let path = get_base_dir();

    let data_buf = fs::read_to_string(path).unwrap();

    let data: Vec<Task> = serde_json::from_str(&data_buf).unwrap();

    data
}

pub fn reset_data() {
    let path = get_base_dir();
    fs::remove_file(path).unwrap();
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
fn check_duplicate_task(task_name: &str, tasks: &[Task]) -> bool {
    tasks
        .iter()
        .any(|x| x.task_name.eq_ignore_ascii_case(task_name))
}
pub fn add_task(task_name: &String) {
    let name = task_name.to_string();
    let mut tasks: Vec<Task> = read();

    if check_duplicate_task(task_name, &tasks) {
        println!("{} task already exists", task_name);
        return;
    }

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
        .position(|x| x.task_name.eq_ignore_ascii_case(task_name))
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
        .position(|x| x.task_name.eq_ignore_ascii_case(task_name))
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
