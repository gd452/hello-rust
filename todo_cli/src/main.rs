mod app;
mod storage;
mod todo;

use app::TodoApp;
use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(name = "todo")]
#[command(author = "Rust Learner")]
#[command(version = "1.0")]
#[command(about = "간단한 CLI Todo 애플리케이션", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 새로운 할 일 추가
    Add {
        /// 할 일 내용
        #[arg(required = true)]
        title: Vec<String>,
    },
    /// 모든 할 일 목록 보기
    List,
    /// 할 일 완료/미완료 토글
    Toggle {
        /// 할 일 ID
        id: usize,
    },
    /// 할 일 삭제
    Delete {
        /// 할 일 ID
        id: usize,
    },
    /// 완료된 모든 할 일 삭제
    Clear,
    /// 통계 보기
    Stats,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let mut app = TodoApp::new()?;

    match cli.command {
        Commands::Add { title } => {
            let title = title.join(" ");
            app.add(title)?;
        }
        Commands::List => {
            app.list();
        }
        Commands::Toggle { id } => {
            app.toggle(id)?;
        }
        Commands::Delete { id } => {
            app.delete(id)?;
        }
        Commands::Clear => {
            app.clear_completed()?;
        }
        Commands::Stats => {
            app.stats();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_creation() {
        let todo = todo::Todo::new(1, "테스트 할 일".to_string());
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "테스트 할 일");
        assert!(!todo.completed);
    }

    #[test]
    fn test_todo_toggle() {
        let mut todo = todo::Todo::new(1, "테스트".to_string());
        assert!(!todo.completed);
        
        todo.toggle();
        assert!(todo.completed);
        
        todo.toggle();
        assert!(!todo.completed);
    }
}