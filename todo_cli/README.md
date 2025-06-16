# Todo CLI 애플리케이션

Rust로 만든 간단한 명령줄 할 일 관리 도구입니다.

## 기능

- ✨ 할 일 추가
- 📋 목록 보기
- ✅ 완료/미완료 토글
- 🗑️ 삭제
- 📊 통계 보기

## 설치

```bash
cargo build --release
```

## 사용법

### 할 일 추가
```bash
cargo run -- add "Rust 공부하기"
cargo run -- add "Todo 앱 개선하기"
```

### 목록 보기
```bash
cargo run -- list
```

### 할 일 완료 처리
```bash
cargo run -- toggle 1
```

### 할 일 삭제
```bash
cargo run -- delete 1
```

### 완료된 할 일 모두 삭제
```bash
cargo run -- clear
```

### 통계 보기
```bash
cargo run -- stats
```

## 학습 포인트

이 프로젝트를 통해 학습한 Rust 개념들:

1. **구조체와 메서드**: `Todo`, `TodoApp`, `Storage` 구조체
2. **열거형**: `Commands` 열거형으로 명령어 정의
3. **에러 처리**: `Result<T, E>` 타입과 `?` 연산자
4. **트레이트**: `Display`, `Serialize/Deserialize`
5. **소유권**: 문자열 처리와 참조
6. **외부 크레이트**: clap (CLI), serde (직렬화), colored (색상)
7. **파일 I/O**: JSON 파일로 데이터 저장/불러오기
8. **패턴 매칭**: `match` 표현식 활용

## 개선 아이디어

- [ ] 우선순위 기능 추가
- [ ] 마감일 설정
- [ ] 카테고리/태그 기능
- [ ] 검색 기능
- [ ] 설정 파일 지원