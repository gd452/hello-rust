// 외부 크레이트 임포트
use chrono::{DateTime, Local};      // 날짜/시간 처리를 위한 chrono 크레이트
use serde::{Deserialize, Serialize}; // JSON 직렬화/역직렬화를 위한 serde
use std::fmt;                        // Display 트레이트 구현을 위한 표준 라이브러리

// Todo 구조체 정의
// derive 매크로로 자동으로 트레이트 구현
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,                     // 고유 식별자
    pub title: String,                 // 할일 제목
    pub completed: bool,               // 완료 여부
    pub created_at: DateTime<Local>,   // 생성 시간 (로컬 시간대)
    pub updated_at: DateTime<Local>,   // 마지막 수정 시간
}

// Todo 구조체의 메서드 구현
impl Todo {
    // 새로운 Todo 인스턴스를 생성하는 연관 함수
    // self 파라미터가 없으므로 Todo::new() 형태로 호출
    pub fn new(id: usize, title: String) -> Self {
        let now = Local::now();  // 현재 로컬 시간 가져오기
        Self {
            id,
            title,
            completed: false,    // 새 할일은 항상 미완료 상태로 시작
            created_at: now,
            updated_at: now,     // 생성시 두 시간 필드는 동일
        }
    }

    // 완료 상태를 토글하는 메서드
    // &mut self: 자기 자신을 가변 참조로 받아 수정
    pub fn toggle(&mut self) {
        self.completed = !self.completed;  // 불린 값 반전
        self.updated_at = Local::now();    // 수정 시간 업데이트
    }

    // 제목을 업데이트하는 메서드
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;         // 새 제목으로 변경
        self.updated_at = Local::now(); // 수정 시간 업데이트
    }
}

// Display 트레이트 구현 - Todo를 문자열로 표시하는 방법 정의
// println!("{}", todo) 형태로 사용 가능
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 완료 상태에 따라 다른 기호 사용
        let status = if self.completed { "✓" } else { "○" };
        
        // 날짜 포맷 지정 (년-월-일 시:분)
        let created = self.created_at.format("%Y-%m-%d %H:%M");
        
        // 포맷터에 출력 형식 작성
        write!(
            f,
            "[{}] {} {} (생성: {})",
            self.id, status, self.title, created
        )
    }
}