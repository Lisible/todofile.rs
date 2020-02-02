use std::fs::File;
use dirs;
use std::path::PathBuf;
use std::io::{BufReader, BufRead};
use std::collections::BTreeMap;
use regex;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
enum TodoFileError {
    FilePathNotFound,
    FileOpenFailed
}

fn open_todo_file() -> Result<File, TodoFileError> {
    let path = match todo_file_path() {
        Some(path) => path,
        None => return Err(TodoFileError::FilePathNotFound)
    };

    match File::open(path) {
        Ok(file) => Ok(file),
        Err(_) => return Err(TodoFileError::FileOpenFailed)
    }
}

fn main() -> Result<(), TodoFileError>{
    let todo_file = open_todo_file()?;
    let reader = BufReader::new(todo_file);
    let mut tasks = BTreeMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        let (priority, task) = parse_task_infos(&line);
        if !tasks.contains_key(&priority) {
            tasks.insert(priority, Vec::new());
        }

        tasks.get_mut(&priority).unwrap().push(task);
    }

    for (_, task_list) in tasks.iter().rev() {
        for task in task_list {
            println!("{}", task);
        }
    }

    Ok(())
}

fn parse_task_infos(task_line: &str) -> (u32, String) {
    let split_line: Vec<&str> = task_line.split_whitespace().collect();

    let priority = parse_priority(&split_line.last().unwrap());
    let task = parse_task(&split_line, priority != 0);
    (priority, task)
}

fn parse_priority(priority_str: &str) -> u32 {
    lazy_static! {
        static ref PRIORITY_REGEX: Regex = Regex::new(r"^\++$").unwrap();
    }

    if PRIORITY_REGEX.is_match(priority_str) {
        priority_str.len() as u32
    } else {
        0
    }
}

fn parse_task(split_line: &Vec<&str>, priority_specified: bool) -> String {
    let last_index = if priority_specified {
        split_line.len() - 1
    }  else {
        split_line.len()
    };

    split_line[0..last_index].join(" ")
}

fn todo_file_path() -> Option<PathBuf> {
    const TODO_FILE: &'static str = "todo";
    let mut file_path = dirs::home_dir()?.clone();
    file_path.push(TODO_FILE);
    Some(file_path)
}