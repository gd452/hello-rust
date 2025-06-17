# Hello Rust - 따라하기 실습

Rust 프로그래밍 언어를 처음 시작하는 분들을 위한 따라하기 실습 프로젝트입니다.

## 📚 실습 목표

이 프로젝트를 통해 다음을 학습할 수 있습니다:
- Rust 개발 환경 설정
- Cargo를 사용한 프로젝트 관리
- 기본적인 Rust 문법 이해
- 간단한 프로그램 작성 및 실행

## 🚀 시작하기

### 1. Rust 설치

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# https://rustup.rs 에서 rustup-init.exe 다운로드 후 실행
```

### 2. 프로젝트 클론

```bash
git clone https://github.com/your-username/hello-rust.git
cd hello-rust
```

### 3. 프로젝트 빌드 및 실행

```bash
# 프로젝트 빌드
cargo build

# 프로젝트 실행
cargo run
```

## 📝 실습 내용

### 실습 1: Hello World 출력하기

`src/main.rs` 파일을 열어 다음 코드를 확인하세요:

```rust
fn main() {
    println!("Hello, world!");
}
```

### 실습 2: 변수와 타입

다음 코드를 추가해보세요:

```rust
fn main() {
    // 불변 변수
    let x = 5;
    println!("x의 값: {}", x);
    
    // 가변 변수
    let mut y = 10;
    println!("y의 초기값: {}", y);
    y = 20;
    println!("y의 변경된 값: {}", y);
}
```

### 실습 3: 함수 만들기

간단한 함수를 추가해보세요:

```rust
fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 실습 4: 조건문과 반복문

```rust
fn main() {
    // 조건문
    let number = 7;
    if number % 2 == 0 {
        println!("{}는 짝수입니다", number);
    } else {
        println!("{}는 홀수입니다", number);
    }
    
    // 반복문
    for i in 1..=5 {
        println!("카운트: {}", i);
    }
}
```

## 🛠 유용한 명령어

```bash
# 코드 포맷팅
cargo fmt

# 린트 검사
cargo clippy

# 테스트 실행
cargo test

# 릴리즈 빌드
cargo build --release
```

## 📚 추가 학습 자료

- [The Rust Programming Language (한국어)](https://doc.rust-kr.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - 실습 문제 모음

## 🤝 기여하기

이 프로젝트는 학습 목적으로 만들어졌습니다. 개선 사항이나 추가 실습 내용이 있다면 PR을 보내주세요!

## 📄 라이선스

MIT License