# Todo CLI - Rust 할일 관리 애플리케이션

Rust로 구현한 커맨드라인 할일 관리 애플리케이션입니다. 이 프로젝트를 통해 Rust의 기본 문법부터 외부 크레이트 활용까지 단계별로 학습할 수 있습니다.

## 목차

1. [프로젝트 개요](#프로젝트-개요)
2. [사전 준비사항](#사전-준비사항)
3. [Step-by-Step 튜토리얼](#step-by-step-튜토리얼)
4. [프로젝트 구조](#프로젝트-구조)
5. [핵심 개념 설명](#핵심-개념-설명)
6. [사용법](#사용법)
7. [확장 아이디어](#확장-아이디어)

## 프로젝트 개요

이 프로젝트는 다음과 같은 기능을 제공합니다:
- ✅ 할일 추가
- 📋 할일 목록 보기 (컬러 출력)
- 🔄 할일 완료/미완료 토글
- 🗑️ 할일 삭제
- 🧹 완료된 할일 모두 삭제
- 📊 통계 보기 (진행률 표시)

## 사전 준비사항

1. **Rust 설치**: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. **IDE 추천**: VS Code + rust-analyzer 확장

## Step-by-Step 튜토리얼

### Step 1: 프로젝트 초기화

```bash
# 새 프로젝트 생성
cargo new todo_cli
cd todo_cli

# 프로젝트 실행 테스트
cargo run
```

### Step 2: 의존성 추가 (Cargo.toml)

`Cargo.toml` 파일을 열고 다음 의존성을 추가합니다:

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }  # CLI 파싱
serde = { version = "1.0", features = ["derive"] } # 직렬화
serde_json = "1.0"                                  # JSON 지원
chrono = { version = "0.4", features = ["serde"] } # 시간 처리
colored = "2.1"                                     # 컬러 출력
```

### Step 3: 모듈 구조 설계

`src` 폴더에 다음 파일들을 생성합니다:

```
src/
├── main.rs      # 진입점 및 CLI 처리
├── todo.rs      # Todo 데이터 구조
├── app.rs       # 애플리케이션 로직
└── storage.rs   # 파일 저장/불러오기
```

### Step 4: Todo 데이터 구조 구현 (todo.rs)

```rust
// src/todo.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Self {
        let now = Utc::now();
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
        self.updated_at = Utc::now();
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { " " };
        let date = self.created_at.format("%Y-%m-%d %H:%M");
        write!(f, "[{}] {} - {} (created: {})", status, self.id, self.title, date)
    }
}
```

### Step 5: 저장소 구현 (storage.rs)

```rust
// src/storage.rs
use crate::todo::Todo;
use std::fs;
use std::path::Path;

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn load(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }
        
        let contents = fs::read_to_string(&self.file_path)?;
        let todos = serde_json::from_str(&contents)?;
        Ok(todos)
    }

    pub fn save(&self, todos: &[Todo]) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(todos)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }
}
```

### Step 6: 애플리케이션 로직 구현 (app.rs)

```rust
// src/app.rs
use crate::storage::Storage;
use crate::todo::Todo;
use colored::*;

pub struct TodoApp {
    todos: Vec<Todo>,
    storage: Storage,
}

impl TodoApp {
    pub fn new(storage_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let storage = Storage::new(storage_path);
        let todos = storage.load()?;
        Ok(Self { todos, storage })
    }

    pub fn add(&mut self, title: String) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.get_next_id();
        let todo = Todo::new(id, title);
        println!("Added: {}", todo.title.green());
        self.todos.push(todo);
        self.storage.save(&self.todos)?;
        Ok(())
    }

    pub fn list(&self) {
        if self.todos.is_empty() {
            println!("{}", "No todos found!".yellow());
            return;
        }

        println!("\n{}", "=== Todo List ===".bold());
        for todo in &self.todos {
            if todo.completed {
                println!("{}", todo.to_string().strikethrough().dimmed());
            } else {
                println!("{}", todo.to_string().bright_white());
            }
        }
        println!();
    }

    pub fn toggle(&mut self, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        match self.todos.iter_mut().find(|t| t.id == id) {
            Some(todo) => {
                todo.toggle();
                let status = if todo.completed { "completed" } else { "uncompleted" };
                println!("Todo {} marked as {}", id, status.green());
                self.storage.save(&self.todos)?;
                Ok(())
            }
            None => {
                eprintln!("{}", format!("Todo with id {} not found", id).red());
                Ok(())
            }
        }
    }

    pub fn delete(&mut self, id: usize) -> Result<(), Box<dyn std::error::Error>> {
        let initial_len = self.todos.len();
        self.todos.retain(|t| t.id != id);
        
        if self.todos.len() < initial_len {
            println!("Deleted todo {}", id);
            self.storage.save(&self.todos)?;
        } else {
            eprintln!("{}", format!("Todo with id {} not found", id).red());
        }
        Ok(())
    }

    pub fn clear_completed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let completed_count = self.todos.iter().filter(|t| t.completed).count();
        self.todos.retain(|t| !t.completed);
        println!("Cleared {} completed todo(s)", completed_count);
        self.storage.save(&self.todos)?;
        Ok(())
    }

    pub fn stats(&self) {
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let pending = total - completed;
        
        println!("\n{}", "=== Statistics ===".bold());
        println!("Total: {}", total.to_string().bright_white());
        println!("Completed: {}", completed.to_string().green());
        println!("Pending: {}", pending.to_string().yellow());
        
        if total > 0 {
            let percentage = (completed as f32 / total as f32) * 100.0;
            let progress_bar = self.create_progress_bar(percentage);
            println!("\nProgress: {} {:.1}%", progress_bar, percentage);
        }
    }

    fn get_next_id(&self) -> usize {
        self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    fn create_progress_bar(&self, percentage: f32) -> String {
        let filled = (percentage / 10.0) as usize;
        let empty = 10 - filled;
        format!("[{}{}]", "█".repeat(filled).green(), "░".repeat(empty))
    }
}
```

### Step 7: CLI 인터페이스 구현 (main.rs)

```rust
// src/main.rs
mod app;
mod storage;
mod todo;

use app::TodoApp;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add { title: String },
    /// List all todos
    List,
    /// Toggle todo completion status
    Toggle { id: usize },
    /// Delete a todo
    Delete { id: usize },
    /// Clear all completed todos
    Clear,
    /// Show statistics
    Stats,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut app = TodoApp::new("todos.json")?;

    match cli.command {
        Commands::Add { title } => app.add(title)?,
        Commands::List => app.list(),
        Commands::Toggle { id } => app.toggle(id)?,
        Commands::Delete { id } => app.delete(id)?,
        Commands::Clear => app.clear_completed()?,
        Commands::Stats => app.stats(),
    }

    Ok(())
}
```

### Step 8: 빌드 및 실행

```bash
# 프로젝트 빌드
cargo build

# 개발 모드로 실행
cargo run -- add "Rust 공부하기"
cargo run -- list
cargo run -- toggle 1
cargo run -- stats

# 릴리즈 빌드
cargo build --release

# 바이너리 직접 실행
./target/release/todo_cli add "새로운 할일"
```

## 프로젝트 구조

```
todo_cli/
├── Cargo.toml      # 프로젝트 설정 및 의존성
├── Cargo.lock      # 의존성 버전 잠금
├── README.md       # 프로젝트 문서
├── todos.json      # 데이터 저장 파일
└── src/
    ├── main.rs     # CLI 진입점
    ├── app.rs      # 비즈니스 로직
    ├── todo.rs     # 데이터 모델
    └── storage.rs  # 영속성 계층
```

## 핵심 개념 설명

### 1. 소유권 (Ownership)
- `String` vs `&str`: 소유권을 가진 문자열과 빌린 문자열 참조
- `mut` 키워드: 가변 참조와 불변 참조

### 2. 에러 처리
- `Result<T, E>` 타입: 성공과 실패를 표현
- `?` 연산자: 에러 전파를 간단하게 처리

### 3. 트레이트 (Traits)
- `Display`: 사용자 정의 출력 형식
- `Serialize/Deserialize`: JSON 변환

### 4. 외부 크레이트 활용
- `clap`: 명령줄 인자 파싱
- `serde`: 데이터 직렬화
- `chrono`: 날짜/시간 처리
- `colored`: 터미널 컬러 출력

## 사용법

### 할일 추가
```bash
cargo run -- add "새로운 할일"
```

### 할일 목록 보기
```bash
cargo run -- list
```

### 할일 완료/미완료 토글
```bash
cargo run -- toggle 1
```

### 할일 삭제
```bash
cargo run -- delete 1
```

### 완료된 할일 모두 삭제
```bash
cargo run -- clear
```

### 통계 보기
```bash
cargo run -- stats
```

## 확장 아이디어

1. **우선순위 기능**: High, Medium, Low 우선순위 추가
2. **카테고리/태그**: 할일을 카테고리별로 분류
3. **검색 기능**: 제목으로 할일 검색
4. **기한 설정**: 마감일 추가 및 알림
5. **설정 파일**: 사용자 설정 저장
6. **데이터베이스**: SQLite 등으로 저장소 변경
7. **웹 API**: REST API로 확장
8. **TUI**: ratatui를 사용한 터미널 UI

## 학습 포인트

1. **Rust 기본 문법**: 변수, 함수, 제어문
2. **타입 시스템**: 강타입, 타입 추론
3. **모듈 시스템**: mod, use, pub
4. **에러 처리**: Result, Option, 패닉
5. **외부 크레이트**: cargo와 crates.io
6. **테스트**: 단위 테스트, 통합 테스트
7. **문서화**: rustdoc, 문서 주석

이 프로젝트를 완성하면 Rust의 핵심 개념들을 실전에서 활용할 수 있게 됩니다!