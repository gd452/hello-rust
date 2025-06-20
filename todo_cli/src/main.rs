// 모듈 선언 - Rust의 모듈 시스템을 사용하여 코드를 구조화
mod app;      // 애플리케이션 로직을 포함하는 모듈
mod storage;  // 파일 저장/불러오기를 담당하는 모듈
mod todo;     // Todo 데이터 구조를 정의하는 모듈

// 외부 크레이트와 모듈 임포트
use app::TodoApp;
use clap::{Parser, Subcommand};  // clap: 커맨드라인 인자 파싱을 위한 크레이트
use std::error::Error;           // 표준 에러 트레이트

// CLI 구조체 정의 - clap의 derive 매크로를 사용하여 자동으로 파서 생성
#[derive(Parser)]
#[command(name = "todo")]
#[command(author = "Rust Learner")]
#[command(version = "1.0")]
#[command(about = "간단한 CLI Todo 애플리케이션", long_about = None)]
struct Cli {
    // subcommand 속성을 통해 여러 명령어를 지원
    #[command(subcommand)]
    command: Commands,
}

// 지원하는 명령어들을 열거형(enum)으로 정의
#[derive(Subcommand)]
enum Commands {
    /// 새로운 할 일 추가
    Add {
        /// 할 일 내용 - 공백을 포함할 수 있도록 Vec<String>으로 받음
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

// 메인 함수 - Result를 반환하여 에러 처리를 간편하게 함
fn main() -> Result<(), Box<dyn Error>> {
    // 커맨드라인 인자를 파싱하여 Cli 구조체 생성
    let cli = Cli::parse();
    
    // TodoApp 인스턴스 생성 - ?를 사용하여 에러 전파
    let mut app = TodoApp::new()?;

    // 패턴 매칭을 사용하여 각 명령어에 따른 처리
    match cli.command {
        Commands::Add { title } => {
            // Vec<String>을 하나의 문자열로 합침 (공백 포함 제목 지원)
            let title = title.join(" ");
            app.add(title)?;
        }
        Commands::List => {
            // 할일 목록 출력 (에러가 발생하지 않는 작업)
            app.list();
        }
        Commands::Toggle { id } => {
            // 특정 ID의 할일 상태 토글
            app.toggle(id)?;
        }
        Commands::Delete { id } => {
            // 특정 ID의 할일 삭제
            app.delete(id)?;
        }
        Commands::Clear => {
            // 완료된 할일 모두 삭제
            app.clear_completed()?;
        }
        Commands::Stats => {
            // 통계 정보 출력 (에러가 발생하지 않는 작업)
            app.stats();
        }
    }

    // 정상 종료
    Ok(())
}

// 테스트 모듈 - cfg(test) 속성으로 테스트 실행시에만 컴파일됨
#[cfg(test)]
mod tests {
    use super::*;

    // Todo 생성 테스트
    #[test]
    fn test_todo_creation() {
        // 새로운 Todo 인스턴스 생성
        let todo = todo::Todo::new(1, "테스트 할 일".to_string());
        
        // 생성된 Todo의 필드값 검증
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "테스트 할 일");
        assert!(!todo.completed);  // 새로 생성된 할일은 미완료 상태여야 함
    }

    // Todo 토글 기능 테스트
    #[test]
    fn test_todo_toggle() {
        // 가변 Todo 인스턴스 생성 (mut 키워드 필요)
        let mut todo = todo::Todo::new(1, "테스트".to_string());
        
        // 초기 상태는 미완료
        assert!(!todo.completed);
        
        // 토글 후 완료 상태로 변경 확인
        todo.toggle();
        assert!(todo.completed);
        
        // 다시 토글하면 미완료 상태로 변경 확인
        todo.toggle();
        assert!(!todo.completed);
    }
}