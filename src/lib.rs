use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub owner: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: vec![] }
    }

    pub fn add(&mut self, task: Task) {
        self.tasks.push(task)
    }

    pub fn remove_by_id(&mut self, id: i32) {
        self.tasks.retain(|t| t.id != id)
    }
}

pub trait Loader {
    fn load(&self) -> Result<TaskList, Box<dyn Error>>;
}

pub struct JSONLoader {
    pub path: String,
}

impl JSONLoader {
    pub fn new(filepath: &str) -> Self {
        JSONLoader {
            path: filepath.to_owned(),
        }
    }
}

impl Loader for JSONLoader {
    fn load(&self) -> Result<TaskList, Box<dyn Error>> {
        let task_list = {
            let text = std::fs::read_to_string(&self.path);
            if text.is_err() {
                println!("unable to read file at {}", &self.path);
                return Ok(TaskList::new())
            }
            serde_json::from_str::<TaskList>(&text?)?
        };
        Ok(task_list)
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
