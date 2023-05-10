# Structure

## Description

- Structure(`struct`)는 사용자 정의 데이터 유형입니다.
- 객체 지향 언어의 클래스 어트리뷰트와 유사합니다.
- 구조체와 열거형은 컴파일 시간 유형 검사를 최대한 활용하기 위해 프로그램 도메인에서 새 유형을 생성하기 위한 빌딩 블록입니다.

## 구조체 데이터의 소유권

- 구조체의 인스턴스가 모든 데이터를 소우하고 전체 구조체가 유효한 한 해당 데이터가 유효하기를 원하기 때문에 문자열 슬라이스 유형이 아닌 `String` 유형으로 명시함.
- 구조체가 다른 것이 소유한 데이터에 대한 참조를 저장하는 것도 가능하지만(예: `String` 대신 `&str`), 이러기 위해서는 `lifetimes`라는 기능을 사용해야 함.
- 수명(`lifetimes`)는 구조체가 참조하는 데이터가 유효한지 확인함.

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someuser", // error[E0106]: missing lifetime specifier
        email: "someuser@eaxmple.com", // error[E0106]: missing lifetime specifier
        sign_in_count: 1,
    };
}
```