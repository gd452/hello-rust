use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Self {
        let now = Local::now();
        Self {
            id,
            title,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
        self.updated_at = Local::now();
    }

    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.updated_at = Local::now();
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { "○" };
        let created = self.created_at.format("%Y-%m-%d %H:%M");
        write!(
            f,
            "[{}] {} {} (생성: {})",
            self.id, status, self.title, created
        )
    }
}