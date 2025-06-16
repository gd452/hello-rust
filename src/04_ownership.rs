// 4. 소유권 (Ownership) - Rust의 핵심 개념
pub fn ownership_demo() {
    println!("\n=== 소유권 (Ownership) ===");
    
    // 스택과 힙
    stack_heap_demo();
    
    // 소유권 이동
    ownership_move_demo();
    
    // 참조와 차용
    borrowing_demo();
    
    // 슬라이스
    slice_demo();
}

fn stack_heap_demo() {
    println!("\n--- 스택과 힙 ---");
    
    // 스택에 저장되는 고정 크기 데이터
    let x = 5;
    let y = x;  // 복사 (Copy trait)
    println!("x = {}, y = {} (둘 다 사용 가능)", x, y);
    
    // 힙에 저장되는 가변 크기 데이터
    let s1 = String::from("Hello");
    let s2 = s1;  // 소유권 이동 (Move)
    // println!("{}", s1);  // 에러! s1은 더 이상 유효하지 않음
    println!("s2 = {}", s2);
    
    // 명시적 복사
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {} (clone 사용)", s2, s3);
}

fn ownership_move_demo() {
    println!("\n--- 소유권 이동 ---");
    
    let s = String::from("소유권 예제");
    takes_ownership(s);  // s의 소유권이 함수로 이동
    // println!("{}", s);  // 에러! s는 더 이상 유효하지 않음
    
    let x = 5;
    makes_copy(x);  // x는 복사됨
    println!("x는 여전히 사용 가능: {}", x);
    
    // 소유권 반환
    let s1 = gives_ownership();
    println!("반환받은 문자열: {}", s1);
    
    let s2 = String::from("안녕");
    let s3 = takes_and_gives_back(s2);
    println!("받아서 다시 반환: {}", s3);
}

fn takes_ownership(some_string: String) {
    println!("함수가 소유권을 받음: {}", some_string);
}  // some_string이 스코프를 벗어나고 drop 호출됨

fn makes_copy(some_integer: i32) {
    println!("함수가 복사본을 받음: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("함수가 생성한 문자열");
    some_string  // 소유권 반환
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 받은 것을 그대로 반환
}

fn borrowing_demo() {
    println!("\n--- 참조와 차용 ---");
    
    let s1 = String::from("참조 예제");
    let len = calculate_length(&s1);  // 참조 전달
    println!("'{}' 의 길이는 {} 입니다", s1, len);  // s1 여전히 사용 가능
    
    // 가변 참조
    let mut s = String::from("Hello");
    change(&mut s);
    println!("변경된 문자열: {}", s);
    
    // 참조 규칙 데모
    reference_rules_demo();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s는 참조이므로 drop되지 않음

fn change(some_string: &mut String) {
    some_string.push_str(", Rust!");
}

fn reference_rules_demo() {
    println!("\n--- 참조 규칙 ---");
    
    let mut s = String::from("참조 규칙");
    
    // 여러 개의 불변 참조는 가능
    let r1 = &s;
    let r2 = &s;
    println!("불변 참조들: {} and {}", r1, r2);
    // r1과 r2는 더 이상 사용되지 않음
    
    // 가변 참조는 하나만 가능
    let r3 = &mut s;
    r3.push_str(" 테스트");
    println!("가변 참조: {}", r3);
    
    // 댕글링 참조는 컴파일 에러
    // let reference_to_nothing = dangle();  // 컴파일 에러!
}

// fn dangle() -> &String {  // 컴파일 에러!
//     let s = String::from("hello");
//     &s  // s는 함수가 끝나면 drop되므로 댕글링 참조
// }

fn slice_demo() {
    println!("\n--- 슬라이스 ---");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("슬라이스: '{}' '{}'", hello, world);
    
    // 더 간단한 문법
    let hello = &s[..5];   // 0부터 시작
    let world = &s[6..];   // 끝까지
    let whole = &s[..];    // 전체
    
    println!("간단한 문법: '{}' '{}' '{}'", hello, world, whole);
    
    // 문자열 슬라이스 활용
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("첫 번째 단어: {}", word);
    // s.clear();  // 에러! word가 s를 차용 중
    
    // 배열 슬라이스
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("배열 슬라이스: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}