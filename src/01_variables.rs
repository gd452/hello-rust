// 1. 변수와 가변성
pub fn variables_demo() {
    println!("\n=== 변수와 가변성 ===");
    
    // 불변 변수 (기본값)
    let x = 5;
    println!("x = {}", x);
    
    // x = 10; // 에러! 불변 변수는 재할당 불가
    
    // 가변 변수
    let mut y = 10;
    println!("y 초기값 = {}", y);
    y = 20;
    println!("y 변경 후 = {}", y);
    
    // Shadowing (같은 이름으로 재선언)
    let x = x + 1;
    println!("x (shadowing) = {}", x);
    
    // 타입도 변경 가능
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces 길이 = {}", spaces);
}