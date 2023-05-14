enum IpAddrKind {
    V4,
    V6,
}
// 열거형의 각 타입에는 네임스페이스가 지정되어 이중 콜론을 통해 각 변수를 구분합니다.

// 이렇게 입력 받은 매개변수는 어떤 종류(열거형) 인지만 알고 있기 때문에, 실제로 조작하기에는 어려움이 있음
fn route(ip_kind: IpAddrKind) {}

// 선언 방법 1
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 열거형만 사용하여 위의 구조체와 동일한 개념을 나타내는 것이 더 간결함
enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum IpAddrOther {
    V4(u8, u8, u8, u8),
    V6(String),
}
// 이렇게 선언하게 되면 V4와 V6의 구조가 달라지기 때문에 구조체를 사용할 수 없게 됨.(사용하려고 하더라도 구조체를 두 개 선언해야 함)

enum Message {
    Quit,                       // 관련 데이터가 없음
    Move { x: i32, y: i32 },    // 구조체처럼 명명된 필드가 있음
    Write(String),              // 단일 String
    ChangeColor(i32, i32, i32), // 세 가지 i32 값을 포함함
} // 이렇게 그룹화된 타입을 구조체로 선언하기는 곤란함

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// Rust 언어 에서의 열거형은 구조체와 비슷하게 `impl` 키워드를 사용, 메서드도 정의할 수 있음

impl Message {
    fn call(&self) {
        // 메서드 본문은 여기에 구현
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let home_enum = IpAddrEnum::V4(String::from("127.0.0.1"));

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let loopback_enum = IpAddrEnum::V6(String::from("::1"));
    // 열거형의 각 타입에 직접 데이터를 첨부하기 때문에 추가 구조체가 필요하지 않음
    // 우리가 정의하는 각 열거형 변형의 이름도 열거형의 인스턴스를 구성하는 함수가 됨
    // 인수를 사용하고 해당 유형의 인스턴스를 반환하는 `IpAddr::V4()` 함수의 호출
    // 열거형을 정의한 결과로 정의된 이 생성자 함수를 자동으로 얻음

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let m = Message::Write(String::from("hello"));
    m.call();
}
