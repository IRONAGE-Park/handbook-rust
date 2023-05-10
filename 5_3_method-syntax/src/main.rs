#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 컨텍스트 내에서 함수를 정의하기 위해 구현 블록을 시작함
    // 이 블록 내의 모든 항목은 타입과 연결됨
    fn area(&self) -> u32 { // `&self`는 `self: &Rectangle`의 문법적 설탕(syntactic sugar)
        self.width * self.height
    }
    
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
}

impl Rectangle {
    // 구조체는 여러 블록을 가질 수 있음.
    
    // Associate Function(관련 함수)
    fn square(size: u32) -> Self {
        // 정사각형 생성자를 작성
        // String::from 함수를 떠올려 볼 수 있음
        // `Rust`에는 `new` 키워드가 내장되어 있지 않음.
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    
    if rect.width() { // 필드와 이름이 같더라도 괄호가 포함되어 있기 때문에 메서드라는 것을 알 수 있음
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }
    // Rust 에서는 getters 메서드를 구조체 필드에 자동으로 구현하지는 않음

    // 한 인스턴스와 매개변수의 인스턴스를 비교하여 완전히 들어갈 수 있으면 `true`를 반환하고, 그렇지 않으면 `false`를 반환하는 메서드를 구현해보자.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    let square = Rectangle::square(3);
}
