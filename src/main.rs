mod variables {
    include!("01_variables.rs");
}
mod types {
    include!("02_types.rs");
}
mod functions {
    include!("03_functions.rs");
}
mod ownership {
    include!("04_ownership.rs");
}
mod structs {
    include!("05_structs.rs");
}
mod enums {
    include!("06_enums.rs");
}
mod error_handling {
    include!("07_error_handling.rs");
}

fn main() {
    println!("🦀 Rust 학습 프로그램에 오신 것을 환영합니다! 🦀");
    println!("=============================================");
    
    // 학습할 주제 선택
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "1" | "variables" => variables::variables_demo(),
            "2" | "types" => types::types_demo(),
            "3" | "functions" => functions::functions_demo(),
            "4" | "ownership" => ownership::ownership_demo(),
            "5" | "structs" => structs::structs_demo(),
            "6" | "enums" => enums::enums_demo(),
            "7" | "error" => error_handling::error_handling_demo(),
            "all" => {
                variables::variables_demo();
                types::types_demo();
                functions::functions_demo();
                ownership::ownership_demo();
                structs::structs_demo();
                enums::enums_demo();
                error_handling::error_handling_demo();
            }
            _ => print_help(),
        }
    } else {
        print_help();
    }
}

fn print_help() {
    println!("\n사용법:");
    println!("  cargo run -- <옵션>");
    println!("\n옵션:");
    println!("  1 또는 variables  - 변수와 가변성");
    println!("  2 또는 types      - 데이터 타입");
    println!("  3 또는 functions  - 함수와 제어문");
    println!("  4 또는 ownership  - 소유권 (중요!)");
    println!("  5 또는 structs    - 구조체");
    println!("  6 또는 enums      - 열거형과 패턴 매칭");
    println!("  7 또는 error      - 에러 처리");
    println!("  all              - 모든 예제 실행");
    println!("\n예시:");
    println!("  cargo run -- 1");
    println!("  cargo run -- ownership");
    println!("  cargo run -- all");
}
