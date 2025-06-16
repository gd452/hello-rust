// 3. 함수와 제어문
pub fn functions_demo() {
    println!("\n=== 함수와 제어문 ===");
    
    // 함수 호출
    greet("Rust 개발자");
    
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);
    
    // 표현식과 구문
    let y = {
        let x = 3;
        x + 1  // 세미콜론이 없으면 표현식 (반환값)
    };
    println!("블록 표현식 결과: {}", y);
    
    // 조건문
    let number = 7;
    if number < 5 {
        println!("{} 는 5보다 작습니다", number);
    } else if number == 5 {
        println!("{} 는 5입니다", number);
    } else {
        println!("{} 는 5보다 큽니다", number);
    }
    
    // if는 표현식
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("조건에 따른 값: {}", number);
    
    // 반복문
    loops_demo();
}

fn greet(name: &str) {
    println!("안녕하세요, {}님!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // return 키워드 없이 마지막 표현식이 반환값
}

fn loops_demo() {
    println!("\n--- 반복문 ---");
    
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;  // loop에서 값 반환
        }
    };
    println!("loop 결과: {}", result);
    
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("발사!");
    
    // for
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("배열 요소: {}", element);
    }
    
    // 범위를 사용한 for
    for number in 1..4 {  // 1, 2, 3
        println!("범위: {}", number);
    }
    
    // 역순
    for number in (1..4).rev() {
        println!("역순: {}", number);
    }
}