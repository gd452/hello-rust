// 7. 에러 처리 (Error Handling)
use std::fs::File;
use std::io::{self, Read, Write, ErrorKind};
use std::error::Error;
use std::fmt;

pub fn error_handling_demo() {
    println!("\n=== 에러 처리 (Error Handling) ===");
    
    // panic! 매크로
    panic_demo();
    
    // Result<T, E> 기본
    result_basics();
    
    // 에러 전파
    error_propagation();
    
    // 커스텀 에러 타입
    custom_error_demo();
}

fn panic_demo() {
    println!("\n--- panic! 매크로 ---");
    println!("panic!은 복구 불가능한 에러에 사용됩니다.");
    println!("실제로 실행하면 프로그램이 종료되므로 주석 처리했습니다.");
    
    // panic!("크래시 발생!");
    
    // 배열 인덱스 초과도 panic 발생
    // let v = vec![1, 2, 3];
    // v[99];  // panic: index out of bounds
}

fn result_basics() {
    println!("\n--- Result<T, E> 기본 ---");
    
    // 파일 열기 시도
    let file_result = File::open("hello.txt");
    
    // match로 처리
    let _file = match file_result {
        Ok(file) => {
            println!("파일을 성공적으로 열었습니다!");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("파일이 없습니다. 새로 생성합니다.");
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        println!("파일 생성 실패: {:?}", e);
                        return;
                    }
                }
            }
            other_error => {
                println!("파일 열기 실패: {:?}", other_error);
                return;
            }
        }
    };
    
    // unwrap과 expect
    println!("\n--- unwrap과 expect ---");
    
    // unwrap(): Ok면 값 반환, Err면 panic
    // let _file = File::open("존재하지_않는_파일.txt").unwrap();
    
    // expect(): unwrap과 같지만 에러 메시지 지정 가능
    // let _file = File::open("존재하지_않는_파일.txt")
    //     .expect("파일을 열 수 없습니다!");
    
    // unwrap_or_else(): 에러 시 대체 동작
    let _file = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("파일 생성 실패: {:?}", error);
            })
        } else {
            panic!("파일 열기 실패: {:?}", error);
        }
    });
    
    println!("파일 처리 완료");
}

// 에러를 반환하는 함수
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? 연산자를 사용한 간단한 버전
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 더 간단한 버전 (메서드 체이닝)
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 가장 간단한 버전
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

fn error_propagation() {
    println!("\n--- 에러 전파 ---");
    
    // username.txt 파일 생성
    let mut file = File::create("username.txt").expect("파일 생성 실패");
    file.write_all(b"rust_user").expect("파일 쓰기 실패");
    
    // 다양한 방법으로 읽기
    match read_username_from_file() {
        Ok(username) => println!("사용자명 (긴 버전): {}", username),
        Err(e) => println!("에러 발생: {}", e),
    }
    
    match read_username_from_file_short() {
        Ok(username) => println!("사용자명 (? 연산자): {}", username),
        Err(e) => println!("에러 발생: {}", e),
    }
    
    // Option에서도 ? 사용 가능
    let result = last_char_of_first_line("Hello\nWorld");
    println!("첫 줄의 마지막 문자: {:?}", result);
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 커스텀 에러 타입
#[derive(Debug)]
enum ParseError {
    InvalidInput,
    TooLarge,
    NegativeNumber,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidInput => write!(f, "입력이 유효하지 않습니다"),
            ParseError::TooLarge => write!(f, "숫자가 너무 큽니다"),
            ParseError::NegativeNumber => write!(f, "음수는 허용되지 않습니다"),
        }
    }
}

impl Error for ParseError {}

fn parse_positive_number(s: &str) -> Result<u32, ParseError> {
    let num: i32 = s.parse()
        .map_err(|_| ParseError::InvalidInput)?;
    
    if num < 0 {
        Err(ParseError::NegativeNumber)
    } else if num > 100 {
        Err(ParseError::TooLarge)
    } else {
        Ok(num as u32)
    }
}

fn custom_error_demo() {
    println!("\n--- 커스텀 에러 타입 ---");
    
    let test_cases = vec!["42", "-5", "200", "abc"];
    
    for input in test_cases {
        match parse_positive_number(input) {
            Ok(num) => println!("'{}' 파싱 성공: {}", input, num),
            Err(e) => println!("'{}' 파싱 실패: {}", input, e),
        }
    }
    
    // Result를 Option으로 변환
    let number: Option<u32> = parse_positive_number("50").ok();
    println!("\nResult를 Option으로: {:?}", number);
    
    // 에러 처리 모범 사례
    error_handling_best_practices();
}

fn error_handling_best_practices() {
    println!("\n--- 에러 처리 모범 사례 ---");
    
    // 1. 적절한 에러 타입 선택
    println!("1. panic! - 복구 불가능한 에러");
    println!("2. Result - 복구 가능한 에러");
    
    // 2. 에러 메시지는 명확하게
    let result = divide_safe(10.0, 0.0);
    if let Err(msg) = result {
        println!("\n명확한 에러 메시지: {}", msg);
    }
    
    // 3. 에러 체이닝
    let result = complex_operation();
    match result {
        Ok(value) => println!("\n복잡한 연산 성공: {}", value),
        Err(e) => println!("\n복잡한 연산 실패: {}", e),
    }
}

fn divide_safe(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("0으로 나눌 수 없습니다"))
    } else {
        Ok(a / b)
    }
}

fn complex_operation() -> Result<f64, Box<dyn Error>> {
    let num_str = "42";
    let num: f64 = num_str.parse()?;
    let result = divide_safe(num, 2.0)?;
    Ok(result)
}