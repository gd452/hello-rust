// 6. 열거형 (Enums)과 패턴 매칭
pub fn enums_demo() {
    println!("\n=== 열거형 (Enums) ===");
    
    // 기본 열거형
    basic_enum_demo();
    
    // Option<T> 사용
    option_demo();
    
    // Result<T, E> 미리보기
    result_preview();
    
    // 고급 패턴 매칭
    advanced_matching();
}

// 기본 열거형
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 데이터를 포함하는 열거형
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 다양한 타입을 포함하는 열거형
#[derive(Debug)]
enum Message {
    Quit,                       // 데이터 없음
    Move { x: i32, y: i32 },   // 구조체 형태
    Write(String),             // String 포함
    ChangeColor(i32, i32, i32), // 튜플 형태
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("프로그램을 종료합니다."),
            Message::Move { x, y } => println!("좌표 ({}, {})로 이동", x, y),
            Message::Write(text) => println!("메시지: {}", text),
            Message::ChangeColor(r, g, b) => println!("색상 변경: RGB({}, {}, {})", r, g, b),
        }
    }
}

fn basic_enum_demo() {
    println!("\n--- 기본 열거형 ---");
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IP 종류: {:?}, {:?}", four, six);
    
    // 데이터를 포함하는 열거형
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("홈 주소: {:?}", home);
    println!("루프백: {:?}", loopback);
    
    // 패턴 매칭
    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 주소: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 주소: {}", addr);
        }
    }
    
    // 메서드가 있는 열거형
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("안녕하세요!")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    println!("\n메시지 처리:");
    for msg in messages {
        msg.process();
    }
}

fn option_demo() {
    println!("\n--- Option<T> ---");
    
    // Option<T>는 Rust의 내장 열거형
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    let some_number = Some(5);
    let some_string = Some("문자열");
    let absent_number: Option<i32> = None;
    
    println!("Some 값들: {:?}, {:?}", some_number, some_string);
    println!("None 값: {:?}", absent_number);
    
    // Option<T> 사용 예
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    // let sum = x + y;  // 에러! Option<i8>과 i8은 더할 수 없음
    
    // 올바른 방법 1: match
    let sum = match y {
        Some(value) => x + value,
        None => x,
    };
    println!("합계 (match): {}", sum);
    
    // 올바른 방법 2: if let
    if let Some(value) = y {
        println!("합계 (if let): {}", x + value);
    }
    
    // 올바른 방법 3: unwrap_or
    let sum = x + y.unwrap_or(0);
    println!("합계 (unwrap_or): {}", sum);
    
    // 실용적인 예: 배열에서 값 찾기
    let numbers = vec![1, 2, 3, 4, 5];
    let first = numbers.get(0);
    let tenth = numbers.get(10);
    
    println!("\n첫 번째 요소: {:?}", first);
    println!("열 번째 요소: {:?}", tenth);
    
    // 안전한 처리
    match numbers.get(2) {
        Some(value) => println!("세 번째 요소: {}", value),
        None => println!("세 번째 요소가 없습니다"),
    }
}

// 커스텀 에러 타입
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn result_preview() {
    println!("\n--- Result<T, E> 미리보기 ---");
    
    // Result<T, E>도 Rust의 내장 열거형
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);
    
    println!("10 / 2 = {:?}", result1);
    println!("10 / 0 = {:?}", result2);
    
    // Result 처리
    match divide(20.0, 4.0) {
        Ok(value) => println!("20 / 4 = {}", value),
        Err(e) => println!("에러 발생: {:?}", e),
    }
    
    // 연쇄 처리
    let calculation = divide(16.0, 4.0)
        .and_then(|x| sqrt(x))
        .and_then(|x| divide(x, 2.0));
        
    match calculation {
        Ok(result) => println!("sqrt(16/4) / 2 = {}", result),
        Err(e) => println!("계산 중 에러: {:?}", e),
    }
}

// 고급 패턴 매칭
#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn advanced_matching() {
    println!("\n--- 고급 패턴 매칭 ---");
    
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Paste(String::from("내용 붙여넣기")),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageUnload,
    ];
    
    for event in events {
        inspect_event(event);
    }
    
    // 매치 가드
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("{} 는 5보다 작음", x),
        Some(x) => println!("{} 는 5 이상", x),
        None => println!("값이 없음"),
    }
    
    // @ 바인딩
    let msg = Message::Move { x: 5, y: 10 };
    match msg {
        Message::Move { x, y } if x > 0 && y > 0 => {
            println!("첫 번째 사분면으로 이동: ({}, {})", x, y);
        }
        Message::Move { x: x_val @ 0..=10, y } => {
            println!("x가 0-10 범위: x={}, y={}", x_val, y);
        }
        _ => {}
    }
    
    // if let과 while let
    let config_max = Some(100);
    if let Some(max) = config_max {
        println!("\n최대값 설정: {}", max);
    }
    
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("\n스택 팝:");
    while let Some(top) = stack.pop() {
        println!("  {}", top);
    }
}

fn inspect_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("페이지 로드됨"),
        WebEvent::PageUnload => println!("페이지 언로드됨"),
        WebEvent::KeyPress(c) => println!("'{}' 키 눌림", c),
        WebEvent::Paste(s) => println!("\"{}\" 붙여넣기됨", s),
        WebEvent::Click { x, y } => {
            println!("({}, {}) 위치 클릭됨", x, y);
        }
    }
}