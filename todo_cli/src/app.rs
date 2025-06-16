use crate::storage::Storage;
use crate::todo::Todo;
use colored::*;
use std::error::Error;

pub struct TodoApp {
    todos: Vec<Todo>,
    storage: Storage,
    next_id: usize,
}

impl TodoApp {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let storage = Storage::new();
        let todos = storage.load()?;
        let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        Ok(Self {
            todos,
            storage,
            next_id,
        })
    }

    pub fn add(&mut self, title: String) -> Result<(), Box<dyn Error>> {
        let todo = Todo::new(self.next_id, title);
        println!("{}", format!("✨ 새 할 일 추가: {}", todo.title).green());
        self.todos.push(todo);
        self.next_id += 1;
        self.storage.save(&self.todos)?;
        Ok(())
    }

    pub fn list(&self) {
        if self.todos.is_empty() {
            println!("{}", "📋 할 일이 없습니다.".yellow());
            return;
        }

        println!("{}", "\n📋 할 일 목록:".bold());
        println!("{}", "─".repeat(50));
        
        for todo in &self.todos {
            let display = if todo.completed {
                format!("{}", todo).strikethrough().dimmed()
            } else {
                format!("{}", todo).normal()
            };
            println!("{}", display);
        }
        
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        println!("{}", "─".repeat(50));
        println!(
            "{}",
            format!("전체: {} | 완료: {} | 미완료: {}", total, completed, total - completed).cyan()
        );
    }

    pub fn toggle(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        match self.todos.iter_mut().find(|t| t.id == id) {
            Some(todo) => {
                todo.toggle();
                let msg = if todo.completed {
                    format!("✅ 완료: {}", todo.title).green()
                } else {
                    format!("⏸️  미완료로 변경: {}", todo.title).yellow()
                };
                println!("{}", msg);
                self.storage.save(&self.todos)?;
                Ok(())
            }
            None => {
                println!("{}", format!("❌ ID {} 를 찾을 수 없습니다.", id).red());
                Ok(())
            }
        }
    }

    pub fn delete(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            let removed = self.todos.remove(pos);
            println!("{}", format!("🗑️  삭제됨: {}", removed.title).red());
            self.storage.save(&self.todos)?;
        } else {
            println!("{}", format!("❌ ID {} 를 찾을 수 없습니다.", id).red());
        }
        Ok(())
    }

    pub fn clear_completed(&mut self) -> Result<(), Box<dyn Error>> {
        let before_count = self.todos.len();
        self.todos.retain(|todo| !todo.completed);
        let removed_count = before_count - self.todos.len();
        
        if removed_count > 0 {
            println!("{}", format!("🗑️  {} 개의 완료된 할 일이 삭제되었습니다.", removed_count).red());
            self.storage.save(&self.todos)?;
        } else {
            println!("{}", "완료된 할 일이 없습니다.".yellow());
        }
        Ok(())
    }

    pub fn stats(&self) {
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        println!("{}", "\n📊 통계:".bold());
        println!("{}", "─".repeat(30));
        println!("전체 할 일: {}", total);
        println!("완료됨: {} {}", completed, "✅".repeat(completed.min(10)));
        println!("미완료: {} {}", pending, "⏳".repeat(pending.min(10)));
        
        if total > 0 {
            let percentage = (completed as f64 / total as f64 * 100.0) as u32;
            println!("완료율: {}%", percentage);
            
            let bar_length = 20usize;
            let filled = (bar_length * percentage as usize / 100);
            let bar = format!(
                "[{}{}]",
                "█".repeat(filled).green(),
                "░".repeat(bar_length - filled).dimmed()
            );
            println!("{}", bar);
        }
    }
}