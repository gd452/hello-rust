// 필요한 모듈과 타입 임포트
use crate::todo::Todo;    // Todo 데이터 구조
use std::fs;              // 파일 시스템 작업
use std::io;              // I/O 에러 타입
use std::path::Path;      // 파일 경로 처리

// 상수 정의 - 데이터를 저장할 파일명
const STORAGE_FILE: &str = "todos.json";

// Storage 구조체 - 파일 기반 영속성을 담당
pub struct Storage {
    file_path: String,    // 저장 파일의 경로
}

// Storage의 메서드 구현
impl Storage {
    // 새로운 Storage 인스턴스 생성
    pub fn new() -> Self {
        Self {
            // &str을 String으로 변환
            file_path: STORAGE_FILE.to_string(),
        }
    }

    // 파일에서 Todo 목록을 불러오는 메서드
    // Result<Vec<Todo>, io::Error>: 성공시 Todo 벡터, 실패시 IO 에러 반환
    pub fn load(&self) -> Result<Vec<Todo>, io::Error> {
        // 파일이 존재하지 않으면 빈 벡터 반환
        // Path::new()를 사용하여 파일 존재 여부 확인
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }

        // 파일 내용을 문자열로 읽기
        // ?를 사용하여 에러 발생시 즉시 반환
        let contents = fs::read_to_string(&self.file_path)?;
        
        // JSON 문자열을 Todo 벡터로 역직렬화
        // serde_json의 에러를 io::Error로 변환
        let todos: Vec<Todo> = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        Ok(todos)
    }

    // Todo 목록을 파일에 저장하는 메서드
    // &[Todo]: Todo 슬라이스 참조 (읽기 전용)
    pub fn save(&self, todos: &[Todo]) -> Result<(), io::Error> {
        // Todo 벡터를 보기 좋은 JSON 문자열로 직렬화
        // to_string_pretty()는 들여쓰기가 있는 읽기 쉬운 형식 생성
        let json = serde_json::to_string_pretty(todos)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        // JSON 문자열을 파일에 쓰기
        // fs::write()는 파일이 없으면 생성, 있으면 덮어쓰기
        fs::write(&self.file_path, json)?;
        
        Ok(())
    }
}