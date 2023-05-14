# Enums and Pattern Matching

- 열겨형을 사용하면 가능한 값의 목록을 정의할 수 있습니다.
- `Option`을 사용하여 값이 있거나 없음을 나타내는 데 유용합니다.
- 그리고 `match`를 사용하여 다음 식의 패턴 일치를 통해 열거형의 다른 값에 대해 다른 코드를 쉽게 실행할 수 있습니다.
  - `if let` 구문을 사용하여 간결한 흐름 제어가 가능합니다.

## `Option` 타입과 `Rust` 에서의 `null`

- `Option` 타입은 값이 무언가가 될 수도 있고 아무 것도 아닐 수도 있는 매우 일반적인 시나리오를 인코딩합니다.
  - 비어 있지 않은 목록의 첫 번째 항목을 요청하면 값을 받음
  - 비어 있는 목록의 첫 번째 항목을 요청하면 값을 받지 못함
- `Rust`에는 다른 많은 언어에 있는 `null`이 없습니다.
- 기존 언어에서 `null` 값으로 인해 발생한 문제를 해결하기 위해, `Rust`에서는 `Option` 열거형을 통해 `null`을 대체합니다.

  ```rust
  enum Option<T> {
      None, // `null`과 유사합니다.
      Some(T), // 값이 있음을 뜻합니다.
  }
  ```

- `Option` 타입은 제네릭 매개변수 `T` 데이터 한 조각을 보유할 수 있습니다.

  ```rust
  fn main () {
      let some_number = Some(5);
      let some_char = Some('e');
      let absent_number: Option<i32> = None;
  }
  ```
  
- `Option<T>`와 `T`는 다른 타입이므로 `Option<T>`를 `T`와 혼동할 수 없습니다.
- 즉, `null` 타입과 유사한 `None`을 일반 연산에 사용할 일 없다는 뜻입니다.

  ```rust
  fn main () {
      let x: i8 = 5;
      let y: Option<i8> = Some(5);
      
      let sum = x + y; // 컴파일 에러
  }
  ```
  
- 아래 처럼 일반 타입이 값이 있을 지도, 혹은 없을 지도 모르는 타입과 연산을 할 수는 없기 때문에, 두 타입을 같은 `Option` 타입으로 만들거나, `Option`을 벗겨내어 값이 존재할 때만 사용할 만한 로직을 부여해야 합니다.
- 그렇게 하기 위한 방법으로 다양한 메서드들이 제공되고 있습니다.
- [`Option - Method`](https://doc.rust-lang.org/std/option/enum.Option.html)