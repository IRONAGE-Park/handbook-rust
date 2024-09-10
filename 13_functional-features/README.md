# 13. 함수형 언어의 특성: 반복자와 클로저

- `Rust`의 디자인은 함수형 프로그래밍의 영향을 크게 받음
- 함수형 스타일
  - 함수를 값처럼 파라미터로 넘김
  - 함수 자체를 리턴
  - 나중에 실행하기 위해 함수를 변수에 할당
- 패턴 매칭이나 열거형 또한 함수형 스타일의 영향을 받음

## 13.1 클로저: 자신의 환경을 캡처하는 익명 함수

- `Rust`의 클로저는 변수에 저장하거나 다른 함수에 인수로 전달할 수 있는 익명함수
- 한 곳에서 클로저를 만들고 다른 컨텍스트의 다른 곳에서 호출하여 평가 가능
- 함수와 다르게 클로저는 정의된 스코프의 값을 캡처 가능

### 13.1.1 클로저로 환경 캡처하기

- 시나리오

  1. 티셔츠 회사의 메일링 리스트에 있는 사람에게 Color를 선택할 수 있는 티셔츠를 공급함
  2. 메일링 리스트에 등록된 사람은 좋아하는 Color를 사전에 등록해둘 수 있음
  3. 좋아하는 Color가 등록된 사람은 그 Color의 티셔츠를 공급
  4. 좋아하는 Color가 등록되지 않은 사람은 회사에 가장 많이 남은 티셔츠를 공급

- 소스 코드

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShiftColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShiftColor>,
}

impl Inventory {
  fn giveaway(&self, user_preference: Option<ShiftColor>) -> ShiftColor {
    // `unwrap_or_else` 함수의 인자가 클로저
    // 아무런 인수도 없고 `T` 값을 반환하는 클로저
    // 클로저가 정의되어 있고, `unwrap_or_else`의 구현부가 이 클로저를 나중에 평가할 것
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShiftColor {
    let mut red_count = 0;
    let mut blue_count = 0;

    for shirt in &self.shirts {
      match shirt {
        ShiftColor::Red => red_count += 1,
        ShiftColor::Blue => blue_count += 1,
      }
    }

    if red_count > blue_count {
      ShiftColor::Red
    } else {
      ShiftColor::Blue
    }
  }
}

fn main() {
  let store = Inventory {
    shirts: vec![ShiftColor::Red, ShiftColor::Blue, ShiftColor::Red],
  };

  let user_pref1 = Some(ShiftColor::Red);
  let giveaway1 = store.giveaway(user_pref1);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref1, giveaway1
  );

  let user_pref2 = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref2, giveaway2
  );
}
```

- 실행 결과

```shell
$ cargo run
  Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
   Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

- `Inventory` 인스턴스에서 `self.most_stocked()`를 호출하는 클로저를 넘김
- 표준 라이브러리는 우리가 정의한 타입이나 로직에 대해 전혀 알 필요가 없음
- 이 클로저는 불변 참조자를 캡처하여 우리가 지정한 로직과 함께 메서드에 넘겨줄 수 있으나, 함수는 이런 방식으로 자신의 환경을 캡처할 수 없음

### 13.1.2 클로저 타입 추론과 명시