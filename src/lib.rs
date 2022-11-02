use derive_more::Display;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Display)]
pub enum BackendError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl actix_web::error::ResponseError for BackendError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInput {
    pub name: String,
    pub owner: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub owner: String,
}

impl Task {
    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let s = serde_json::to_string(&self)?;
        Ok(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: vec![] }
    }

    pub fn add(&mut self, task: TaskInput) {
        let t = Task {
            name: task.name,
            owner: task.owner,
            id: self.next_id(),
        };
        self.tasks.push(t)
    }

    pub fn remove_by_id(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id)
    }

    pub fn get_by_id(&self, id: u32) -> Option<Task> {
        let task = match self.tasks.iter().find(|t| t.id == id) {
            Some(t) => t,
            None => return None,
        };
        return Some(task.to_owned());
    }

    pub fn to_string(&self) -> Result<String, BackendError> {
        let s = match serde_json::to_string(&self) {
            Ok(t) => t,
            Err(_) => return Err(BackendError::BadClientData),
        };
        Ok(s)
    }

    pub fn next_id(&self) -> u32 {
        match self.tasks.iter().map(|t| t.id).max() {
            Some(n) => n + 1,
            None => 1,
        }
    }
}

pub trait DataHandler {
    fn load(&self) -> Result<TaskList, Box<dyn Error>>;
    fn save(&self, task_list: &TaskList) -> Result<(), Box<dyn Error>>;
}

pub struct JSONHandler {
    pub path: String,
}

impl JSONHandler {
    pub fn new(filepath: &str) -> Self {
        JSONHandler {
            path: filepath.to_owned(),
        }
    }
}

impl DataHandler for JSONHandler {
    fn load(&self) -> Result<TaskList, Box<dyn Error>> {
        let task_list = {
            let text = std::fs::read_to_string(&self.path);
            if text.is_err() {
                println!("unable to read file at {}", &self.path);
                return Ok(TaskList::new());
            }
            serde_json::from_str::<TaskList>(&text?)?
        };
        Ok(task_list)
    }

    fn save(&self, task_list: &TaskList) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(&self.path)?;
        let json = task_list.to_string().expect("invalid data");
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

// pub struct SQLiteLoader {
// pub path: String,
// }
//
// pub struct PostgresLoader {
// pub host: String,
// pub port: String,
// pub user: String,
// pub password: String,
// }
