use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackupStatus {
    Running,
    Completed,
    Failed(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Backup {
    pub id: Uuid,
    pub name: String,
    pub status: BackupStatus,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
}

impl Backup {
    pub fn new(name: String) -> Self {
        Backup {
            id: Uuid::new_v4(),
            name,
            status: BackupStatus::Running,
            started_at: Utc::now(),
            finished_at: None,
        }
    }

    pub fn complete(mut self) -> Self {
        self.status = BackupStatus::Completed;
        self.finished_at = Some(Utc::now());
        self
    }

    pub fn fail(mut self, error: String) -> Self {
        self.status = BackupStatus::Failed(error);
        self.finished_at = Some(Utc::now());
        self
    }
}


// use crate::tasks::{Task, TaskStatus};
// use std::fs::{File, OpenOptions};
// use std::io::{Read, Write};
//
// const DB_FILE: &str = "tasks.json";
//
// pub fn save_task(task: &Task) {
//     let mut tasks = load_all_tasks();
//     tasks.push(task.clone());
//     write_all_tasks(&tasks);
// }
//
// pub fn update_task_status(id: &str, new_status: TaskStatus) {
//     let mut tasks = load_all_tasks();
//     for task in &mut tasks {
//         if task.id.to_string() == id {
//             task.status = new_status.clone();
//             task.finished_at = Some(chrono::Utc::now());
//         }
//     }
//     write_all_tasks(&tasks);
// }
//
// pub fn load_all_tasks() -> Vec<Task> {
//     let mut file = match OpenOptions::new().read(true).open(DB_FILE) {
//         Ok(f) => f,
//         Err(_) => return vec![],
//     };
//
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     serde_json::from_str(&contents).unwrap_or_default()
// }
//
// fn write_all_tasks(tasks: &[Task]) {
//     let data = serde_json::to_string_pretty(tasks).unwrap();
//     let mut file = File::create(DB_FILE).unwrap();
//     file.write_all(data.as_bytes()).unwrap();
// }
// https://chatgpt.com/c/68775387-3940-8008-b3fd-e12ccb82a4a3