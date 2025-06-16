# Rust 학습 로드맵

## 📚 1단계: Rust 기초 (1-2주)

### 1.1 환경 설정 및 Hello World
- Rust 설치 (rustup)
- Cargo 기본 사용법
- 첫 프로그램 작성

### 1.2 기본 문법
- 변수와 가변성 (mut)
- 데이터 타입 (정수, 부동소수점, 불리언, 문자)
- 함수와 제어문
- 주석과 문서화

### 1.3 소유권(Ownership) 이해 ⭐
- 소유권 규칙
- 참조와 차용 (References & Borrowing)
- 슬라이스 (Slices)

### 1.4 구조체와 열거형
- 구조체 정의와 사용
- 메서드 구문
- 열거형과 패턴 매칭
- Option<T>와 Result<T, E>

## 📚 2단계: Rust 중급 (2-3주)

### 2.1 컬렉션
- 벡터 (Vec<T>)
- 문자열 (String)
- 해시맵 (HashMap<K, V>)

### 2.2 에러 처리
- panic!과 복구 불가능한 에러
- Result<T, E> 활용
- ? 연산자

### 2.3 제네릭과 트레이트
- 제네릭 데이터 타입
- 트레이트 정의와 구현
- 트레이트 바운드

### 2.4 라이프타임
- 라이프타임 문법
- 함수에서의 라이프타임
- 구조체에서의 라이프타임

## 📚 3단계: Rust 고급 (3-4주)

### 3.1 스마트 포인터
- Box<T>
- Rc<T>와 Arc<T>
- RefCell<T>와 내부 가변성

### 3.2 동시성
- 스레드 생성과 관리
- 메시지 패싱 (채널)
- 공유 상태 동시성 (Mutex, RwLock)
- Send와 Sync 트레이트

### 3.3 비동기 프로그래밍
- async/await
- Future 트레이트
- tokio 런타임

### 3.4 매크로
- 선언적 매크로 (macro_rules!)
- 절차적 매크로 기초

## 🛠️ 실습 프로젝트

### 프로젝트 1: CLI Todo 앱 (기초)
- 명령줄 인자 파싱 (clap)
- 파일 I/O
- 직렬화/역직렬화 (serde)

### 프로젝트 2: 웹 스크레이퍼 (중급)
- HTTP 요청 (reqwest)
- HTML 파싱 (scraper)
- 비동기 처리
- 에러 처리 패턴

### 프로젝트 3: 미니 웹 서버 (고급)
- TCP 소켓 프로그래밍
- HTTP 프로토콜 구현
- 멀티스레드 처리
- 라우팅 시스템

### 최종 프로젝트: 분산 키-값 저장소
- 네트워크 통신
- 동시성 제어
- 데이터 영속성
- 성능 최적화

## 🎯 AI/시스템 개발을 위한 추가 학습

### AI 관련
- ndarray: NumPy 스타일 배열
- candle: Rust 네이티브 딥러닝 프레임워크
- ort: ONNX Runtime 바인딩

### 시스템 프로그래밍
- unsafe Rust
- FFI (Foreign Function Interface)
- 메모리 레이아웃 제어
- 운영체제 인터페이스

## 📖 추천 리소스
- The Rust Programming Language (공식 문서)
- Rust by Example
- Rustlings (연습 문제)
- Jon Gjengset의 YouTube 채널