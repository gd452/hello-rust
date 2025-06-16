// 2. 데이터 타입
pub fn types_demo() {
    println!("\n=== 데이터 타입 ===");
    
    // 정수형
    let a: i32 = -42;  // 부호 있는 32비트
    let b: u32 = 42;   // 부호 없는 32비트
    let c = 100_000;   // 가독성을 위한 _ 사용
    println!("정수: i32={}, u32={}, 100_000={}", a, b, c);
    
    // 부동소수점
    let pi: f64 = 3.14159;
    let e = 2.71828f32;  // f32 타입 명시
    println!("실수: pi={}, e={}", pi, e);
    
    // 불리언
    let is_rust_awesome = true;
    let is_difficult: bool = false;
    println!("Rust는 멋진가? {}, 어려운가? {}", is_rust_awesome, is_difficult);
    
    // 문자와 문자열
    let ch = '한';
    let emoji = '🦀';
    let string = "Rust 문자열";
    println!("문자: {}, 이모지: {}, 문자열: {}", ch, emoji, string);
    
    // 튜플
    let tuple: (i32, f64, char) = (500, 6.4, 'z');
    let (x, y, z) = tuple;  // 구조 분해
    println!("튜플: ({}, {}, {})", x, y, z);
    println!("튜플 접근: tuple.0 = {}", tuple.0);
    
    // 배열
    let arr = [1, 2, 3, 4, 5];
    let months = ["1월", "2월", "3월"];
    let zeros = [0; 5];  // [0, 0, 0, 0, 0]
    println!("배열: {:?}, 첫 번째 월: {}", arr, months[0]);
    println!("0으로 초기화된 배열: {:?}", zeros);
}