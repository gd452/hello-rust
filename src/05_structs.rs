// 5. 구조체 (Structs)
pub fn structs_demo() {
    println!("\n=== 구조체 (Structs) ===");
    
    // 기본 구조체
    basic_struct_demo();
    
    // 메서드와 연관 함수
    methods_demo();
    
    // 다양한 구조체 타입
    struct_types_demo();
}

// 구조체 정의
#[derive(Debug)] // Debug 트레이트 자동 구현
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn basic_struct_demo() {
    println!("\n--- 기본 구조체 ---");
    
    // 구조체 인스턴스 생성
    let user1 = User {
        active: true,
        username: String::from("rust_learner"),
        email: String::from("rust@example.com"),
        sign_in_count: 1,
    };
    
    println!("사용자: {}", user1.username);
    println!("이메일: {}", user1.email);
    
    // 가변 구조체
    let mut user2 = User {
        active: true,
        username: String::from("another_user"),
        email: String::from("another@example.com"),
        sign_in_count: 0,
    };
    
    user2.sign_in_count += 1;
    println!("로그인 횟수: {}", user2.sign_in_count);
    
    // 구조체 갱신 문법
    let user3 = User {
        email: String::from("new@example.com"),
        ..user1  // 나머지는 user1에서 가져옴
    };
    
    // Debug 출력
    println!("\n전체 구조체 정보:");
    println!("{:?}", user3);  // Debug 포맷
    println!("{:#?}", user2); // Pretty Debug 포맷
    
    // 필드 초기화 축약 문법
    let email = String::from("short@example.com");
    let username = String::from("shorthand");
    
    let user4 = build_user(email, username);
    println!("\n축약 문법으로 생성: {:?}", user4);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,  // username: username 의 축약
        email,     // email: email 의 축약
        sign_in_count: 1,
    }
}

// 메서드가 있는 구조체
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 메서드 (&self 를 첫 번째 매개변수로)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // self 를 가변으로 받는 메서드
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // 다른 Rectangle 를 매개변수로 받는 메서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 연관 함수 (self 매개변수 없음)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn methods_demo() {
    println!("\n--- 메서드와 연관 함수 ---");
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("사각형: {:?}", rect1);
    println!("넓이: {}", rect1.area());
    println!("둘레: {}", rect1.perimeter());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("\nrect1이 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1이 rect3를 포함하는가? {}", rect1.can_hold(&rect3));
    
    // 연관 함수 호출 (:: 문법)
    let square = Rectangle::square(20);
    println!("\n정사각형: {:?}", square);
    
    // 가변 메서드 호출
    let mut rect4 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("\n원래 크기: {:?}", rect4);
    rect4.double_size();
    println!("두 배 크기: {:?}", rect4);
}

// 다양한 구조체 타입
fn struct_types_demo() {
    println!("\n--- 다양한 구조체 타입 ---");
    
    // 튜플 구조체
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("검은색 RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("원점 좌표: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // 유닛 구조체 (필드가 없음)
    struct AlwaysEqual;
    
    let subject = AlwaysEqual;
    // 주로 트레이트 구현용으로 사용
    
    // 실용적인 예: 3D 점과 벡터
    #[derive(Debug)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }
    
    impl Point3D {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }
        
        fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
            self.x += dx;
            self.y += dy;
            self.z += dz;
        }
    }
    
    let mut point = Point3D { x: 3.0, y: 4.0, z: 0.0 };
    println!("\n3D 점: {:?}", point);
    println!("원점으로부터 거리: {}", point.distance_from_origin());
    
    point.translate(1.0, 1.0, 1.0);
    println!("이동 후: {:?}", point);
}