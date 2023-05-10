struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 튜플 구조체

struct AlwaysEqual;
// 단위 유사 구조체

fn build_user(email: String, username: String) -> User {
    // 필드 초기화 축약 구문
    User {
        active: true,
        email, // 필드 이름과 변수 이름이 같을 경우 축약 가능
        username,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    
    // 점 표기법을 통해 접근할 수 있으며, 변경 가능한 인스턴스일 경우 점 표기법을 통해 변경도 가능함.
    let mut user2 = User {
        active: true,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 1,
    };
    
    user2.email = String::from("user2new@example.com");
    
    let user3 = User {
        username: String::from("user3"),
        ..user1 // 명시적으로 설정되지 않은 필드는 user1의 값으로 초기화됨.
        // 이 문법이 항상 마지막에 있기 때문에 다른 여러가지 필드도 명시적으로 지정할 수 있으며, 그렇게 되면 나머지 필드들이 user1의 값으로 초기화됨
    };
    
    // let user4 = User {
    //     ..user1
    // };
    // user3을 선언할 때 한 번 업데이트 구문을 사용했음(업데이트 구문은 `=`처럼 사용함)
    // 따라서 다시 해당 인스턴스를 사용할 수 없음(이동되었기 때문에)
    // 하지만 인스턴스 내부 필드는 접근 가능
    
    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);
    
    // let color_to_pont: Point = color; // 고유한 타입이기 때문에 내부의 타입이 동일하게 구성되어 있더라도 할당할 수 없음
    
    let subject = AlwaysEqual; // 중괄호나 괄호가 필요하지 않음
    // 일부 타입에 메서드를 구현해야 하지만 저장하려는 데이터가 없을 때 주로 사용함.
}
