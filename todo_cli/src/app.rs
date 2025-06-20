// crate 루트로부터 모듈 임포트
use crate::storage::Storage;  // 저장소 모듈
use crate::todo::Todo;        // Todo 데이터 구조
use colored::*;               // 터미널 컬러 출력을 위한 크레이트
use std::error::Error;        // 에러 처리를 위한 표준 트레이트

// TodoApp 구조체 - 애플리케이션의 상태를 관리
pub struct TodoApp {
    todos: Vec<Todo>,     // Todo 항목들을 저장하는 벡터
    storage: Storage,     // 파일 시스템과의 상호작용을 담당
    next_id: usize,       // 다음 할일에 할당할 ID
}

// TodoApp의 메서드 구현
impl TodoApp {
    // 새로운 TodoApp 인스턴스 생성
    // Result 타입을 반환하여 파일 로드 실패 등의 에러 처리
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let storage = Storage::new();
        let todos = storage.load()?;  // ? 연산자로 에러 전파
        
        // 기존 할일들 중 가장 큰 ID를 찾아 다음 ID 설정
        let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        Ok(Self {
            todos,
            storage,
            next_id,
        })
    }

    // 새로운 할일 추가
    pub fn add(&mut self, title: String) -> Result<(), Box<dyn Error>> {
        // 새 Todo 인스턴스 생성
        let todo = Todo::new(self.next_id, title);
        
        // 성공 메시지 출력 (녹색)
        println!("{}", format!("✨ 새 할 일 추가: {}", todo.title).green());
        
        // 벡터에 추가하고 ID 증가
        self.todos.push(todo);
        self.next_id += 1;
        
        // 파일에 저장
        self.storage.save(&self.todos)?;
        Ok(())
    }

    // 모든 할일 목록 출력
    pub fn list(&self) {
        // 할일이 없는 경우 처리
        if self.todos.is_empty() {
            println!("{}", "📋 할 일이 없습니다.".yellow());
            return;
        }

        // 헤더 출력
        println!("{}", "\n📋 할 일 목록:".bold());
        println!("{}", "─".repeat(50));
        
        // 각 할일을 상태에 따라 다른 스타일로 출력
        for todo in &self.todos {
            let display = if todo.completed {
                // 완료된 항목: 취소선 + 흐림 효과
                format!("{}", todo).strikethrough().dimmed()
            } else {
                // 미완료 항목: 일반 스타일
                format!("{}", todo).normal()
            };
            println!("{}", display);
        }
        
        // 요약 정보 출력
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        println!("{}", "─".repeat(50));
        println!(
            "{}",
            format!("전체: {} | 완료: {} | 미완료: {}", total, completed, total - completed).cyan()
        );
    }

    // 특정 ID의 할일 완료 상태 토글
    pub fn toggle(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        // iter_mut()로 가변 반복자를 얻어 직접 수정
        match self.todos.iter_mut().find(|t| t.id == id) {
            Some(todo) => {
                todo.toggle();
                
                // 상태에 따른 메시지 출력
                let msg = if todo.completed {
                    format!("✅ 완료: {}", todo.title).green()
                } else {
                    format!("⏸️  미완료로 변경: {}", todo.title).yellow()
                };
                println!("{}", msg);
                
                // 변경사항 저장
                self.storage.save(&self.todos)?;
                Ok(())
            }
            None => {
                // ID를 찾을 수 없는 경우
                println!("{}", format!("❌ ID {} 를 찾을 수 없습니다.", id).red());
                Ok(())
            }
        }
    }

    // 특정 ID의 할일 삭제
    pub fn delete(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        // position()으로 인덱스를 찾고 remove()로 삭제
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            let removed = self.todos.remove(pos);
            println!("{}", format!("🗑️  삭제됨: {}", removed.title).red());
            self.storage.save(&self.todos)?;
        } else {
            println!("{}", format!("❌ ID {} 를 찾을 수 없습니다.", id).red());
        }
        Ok(())
    }

    // 완료된 모든 할일 삭제
    pub fn clear_completed(&mut self) -> Result<(), Box<dyn Error>> {
        let before_count = self.todos.len();
        
        // retain()으로 미완료 항목만 유지
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

    // 통계 정보 출력
    pub fn stats(&self) {
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|t| t.completed).count();
        let pending = total - completed;

        // 통계 헤더
        println!("{}", "\n📊 통계:".bold());
        println!("{}", "─".repeat(30));
        
        // 기본 통계
        println!("전체 할 일: {}", total);
        println!("완료됨: {} {}", completed, "✅".repeat(completed.min(10)));
        println!("미완료: {} {}", pending, "⏳".repeat(pending.min(10)));
        
        // 진행률 표시
        if total > 0 {
            let percentage = (completed as f64 / total as f64 * 100.0) as u32;
            println!("완료율: {}%", percentage);
            
            // 프로그레스 바 생성
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