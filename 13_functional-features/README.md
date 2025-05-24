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

- 클로저는 `fn`으로 선언한 함수와 다르게 매개변수와 반환 타입 명시가 필수적이지 않음
  - 함수의 타입 명시는 그 타입이 사용자들에게 노출되는 명시적인 인터페이스의 일부분이므로 요구
- 클로저는 통상적으로 짧고 임의의 시나리오가 아니라 짧은 컨텍스트 내에서만 관련됨
  - 이러한 한정된 컨텍스트 내에서, 컴파일러는 변수의 타입 추론과 비슷한 방식으로 클로저의 매개변수와 반환 타입을 추론함
  - 컴파일러가 클로저 타입을 명시하도록 요구하는 경우는 드물게 있음(`Vec`와 유사)

```rust
let expensive_closure = |num: u32| -> u32 {
  println!("calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  num
}

fn  add_one_v1   (x: u32) -> u32 { x + 1 };
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 모든 것이 명시된 클로저
let add_one_v3 = |x|      ->     { x + 1 }; // 타입 명시를 제거한 클로저
let add_one_v4 = |x|      ->       x + 1  ; // 하나의 표현식이므로 중괄호를 제거한 클로저
```

- `add_one_v3`와 `add_one_v4`를 컴파일하기 위해서는 이 클로저들이 평가되는 곳이 필요함
  - 클로저들이 사용된 곳에서 타입이 추론될 것이기 때문
  - `let v = Vec::new();`가 타입이 추론되기 위해 `Vec` 안에 집어 넣을 어떤 타입의 값이 필요한 것과 유사
- 클로저의 정의에 대하여 컴파일러는 각각 매개변수와 반환 값마다 '하나의 고정 타입'을 추론

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

- 위 코드에 대해 컴파일러는 아래의 에러 메시지를 보임

```shell
$ cargo run
   Compiling functional-features v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\13_functional-features)
error[E0308]: mismatched types
  --> src/main.rs:57:29
   |
57 |     let n = example_closure(5);
   |             --------------- ^- help: try using a conversion method: `.to_string()`
   |             |               |
   |             |               expected `String`, found integer
   |             arguments to this function are incorrect
   |
note: expected because the closure was earlier called with an argument of type `String`
  --> src/main.rs:56:29
   |
56 |     let s = example_closure(String::from("hello"));
   |             --------------- ^^^^^^^^^^^^^^^^^^^^^ expected because this argument is of type `String`
   |             |
   |             in this closure call
note: closure parameter defined here
  --> src/main.rs:54:28
   |
54 |     let example_closure = |x| x;
   |                            ^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functional-features` (bin "functional-features") due to 1 previous error
```

- 처음 `String`을 가지고 클로저를 호출해 컴파일러는 클로저의 매개 변수와 리턴 타입이 `String`이라고 추론했기 때문임
- 첫 코드로 `example_closure` 클로저의 타입이 고정되었기 때문에 그 다음 동일한 클로저에 대해서 타입 에러가 발생함

### 13.1.3 참조자를 캡처하거나 소유권 이동하기

- 클로저는 세 가지 방식으로 자신의 환경으로부터 값을 캡처할 수 있음
- 이는 함수가 매개변수를 취하는 세 가지 방식과 직접적으로 대응
  1. 불변으로 빌려오기(참조자)
  2. 가변으로 빌려오기(참조자)
  3. 소유권 이동

#### 불변으로 빌려오기

```rust
fn main() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let only_borrows = || println!("From closure: {:?}", list); // 값을 출력하기 위한 불변 참조자

  println!("Before calling closure: {:?}", list);
  only_borrows();
  println!("After calling closure: {:?}", list);
}
```

- `list`에 대한 불변 참조자를 동시에 가질 수 있기 때문에 `list`에는 클로저 정의 전이나 후 뿐만 아니라 클로저의 호출 전과 후에도 여전히 접근 가능함
- 클로저가 불변 참조자만을 사용하여 `list`를 캡처하는 이유는 `list`를 출력하기 위해 필요한 최소한의 접근 수준이기 때문

```shell
$ cargo run
   Compiling functional-features v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\13_functional-features)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target\debug\functional-features.exe`
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
```

#### 가변으로 빌려오기

```rust
fn main() {
  let mut list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let mut borrows_mutably = || list.push(7); // 벡터 값에 요소를 추가하기 위한 가변 참조자
  // 클로저에도 `mut` 키워드를 선언해야 함

  borrows_mutably();
  println!("After calling closure: {:?}", list);
}
```

- 함수가 정의된 시점에 클로저가 `list`에 대한 가변 참조자를 캡처
- 클로저가 호출된 이후로 클로저를 사용하고 있지 않으므로 가변 대여가 그 시점에서 끝남
- 클로저의 정의와 호출 사이에는 출력을 위한 불변 대여가 허용되지 않음
  - 가변 대여가 있을 때는 다른 대여가 허용되지 않기 때문
- `move` 키워드를 사용해서 클로저가 캡처한 변수의 소유권을 가질 수 있도록 할 수 있음

```shell
$ cargo run
   Compiling functional-features v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\13_functional-features)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target\debug\functional-features.exe`
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]
```

#### 소유권 이동

- 이 기법은 대체로 클로저를 새 스레드에 넘길 때 데이터를 이동시켜서 새로운 스레드가 이 데이터를 소유하게 하는 경우에 유용

```rust
use std::thread;

fn main() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();
}
```

- 이 예제는 스레드의 실행 순서가 보장되지 않으므로 `move` 키워드를 집어 넣어 `list`가 이동되어야 함을 명시할 필요가 있음
  - 만일 메인 스레드가 `list`의 소유권을 유지하고 있는데 새 스레드가 끝나기 전에 끝나버려서 `list`를 제거한다면, 새 스레드의 불변 참조자는 유효하지 않게 됨
- 따라서 컴파일러는 `list`를 새 스레드에 제공될 클로저로 이동시켜 참조자가 유효하도록 요구

### 13.1.4 캡처된 값을 클로저 밖으로 이동하기와 `Fn` 트레이트

- 클로저가 자신이 정의된 환경으로부터 값의 참조자 혹은 소유권을 캡처하면, 클로저 본문의 코드는 이 클로저가 나중에 평가될 때 그 참조자나 값에 어떤 일이 발생하는지 정의
  - 캡처된 값을 클로저 밖으로 이동시키기
  - 캡처된 값을 변형하기
  - 캡처만 하기
  - 캡처하지 않기
- 위와 같이 캡처한 값을 다루는 방식은 클로저가 구현하는 `Fn` 트레이트에 영향을 주고, 어떤 명시의 클로저를 사용하는지 파악할 수 있음
- 클로저는 클로저의 본문이 값을 처리하는 방식에 따라서 이 `Fn` 트레이트들 중 하나, 혹은 모두를 추가하는 방식으로 자동으로 구현됨

1. `FnOnce`: 한 번만 호출될 수 있는 클로저. 모든 클로저는 이 트레이트를 구현함(캡처된 값을 본문 밖으로 이동시키는 경우도 포함)
2. `FnMut`: 캡처된 값을 본문 밖으로 이동시키지는 않지만 변경할 수 있는 클로저.
3. `Fn`: 캡처된 값을 본문 밖으로 이동시키지 않고 변경하지도 않는, 환경으로부터 아무런 값도 캡처하지 않는 클로저.

- `Option<T>::unwrap_or_else`의 정의

```rust
impl<T> Opton<T> {
  pub fn unwrap_or_else<F>(self, f: F) -> T
  where
    F: FnOnce() -> T // `f`가 한 번만 호출될 수 있어야 하고, 인수가 없고, `T`를 반환함
    // `FnOnce`를 사용하는 것은 정의한 함수가 `f`를 아무리 많아야 한 번만 호출할 것이라는 제약 사항을 표현
    // 모든 클로저가 `FnOnce`를 구현하므로 `unwrap_or_else`는 가장 다양한 종류의 클로저를 허용하며 될 수 있는 한 유연하게 동작
  {
    match self {
      Some(x) => x,
      None => f()
    }
  }
}
```

> 함수도 이 세 종류의 `Fn` 트레이트를 모두 구현할 수 있음. 따라서 클로저 대신 함수를 사용할 수도 있음.  
> Example: `unwrap_or_else(Vec::new)`

- `sort_by_key`와 `unwrap_or_else`의 비교를 통해 `FnOnce`와 `FnMut`의 차이점 구분
- `[T].sort_by_key`의 정의

```rust
impl<T> [T] {
  pub fn sort_by_key<K, F>(&mut self, mut f: F)
  where
    F: FnMut(&T) -> K,
    K: Ord
  {
    stable_sort(self, |a, b| f(a).lt(&f(b)));
  }
}
```

- `sort_by_key`는 각 아이템의 특정 속성을 이용하여 슬라이스를 정렬하고 싶을 때 유용
- 아래 코드는 `width` 속성을 낮은 것부터 높은 순으로 정렬

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  let mut list = [
    Rectangle {
      width: 10,
      height: 1,
    },
    Rectangle {
      width: 3,
      height: 5,
    },
    Rectangle {
      width: 7,
      height: 12,
    },
  ];
  list.sort_by_key(|r| r.width);
  println!("{:#?}", list);
}
```

```shell
$ cargo run
   Compiling functional-features v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\13_functional-features)
warning: field `height` is never read
  --> src/main.rs:98:5
   |
96 | struct Rectangle {
   |        --------- field in this struct
97 |     width: u32,
98 |     height: u32,
   |     ^^^^^^
   |
   = note: `Rectangle` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: `functional-features` (bin "functional-features") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.25s
     Running `target\debug\functional-features.exe`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

- `sort_by_key`가 `FnMut` 클로저를 갖도록 정의된 이유는 이 함수가 클로저를 여러 번 호출하기 때문
- 클로저는 자신의 환경으로부터 어떤 것도 캡처나 변형, 혹은 이동시키지 않으므로 `FnMut` 트레이트를 구현하여 트레이트 바운드 요건을 충족함
- `FnOnce` 트레이트만 구현한 클로저의 경우 `sort_by_key`에 사용할 수 없음

```rust
let mut sort_operations = vec![];
let value = String::from("by key called");
list.sort_by_key(|r| {
  // 이 클로저는 `FnMut`를 구현하지 않음, 캡처된 값을 외부로 내보내고 있기 때문
  // let value = String::from("by key called"); 처럼 `value`를 외부가 아닌 내부에 선언해도 사용 가능
  sort_operations.push(value); // value.clone()을 통해 소유권을 밖으로 내보내지 않으면 사용 가능
  r.width
});
```

- 클로저는 `value`를 캡처한 다음 `value`의 소유권을 `sort_operations` 벡터로 보내서 `value`를 클로저 밖으로 이동시킴
- 이 클로저는 한 번만 호출될 수 있는데, 두 번째 호출 시도가 발생하고 이때 `value`가 더이상 환경에 남아있지 않아 동작하지 않음
- 이 클로저는 `FnOnce` 만을 구현하고 있으며, 클로저가 `FnMut`를 구현해야 사용 가능

```shell
$ cargo run
   Compiling functional-features v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\13_functional-features)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:95:30
   |
93 |     let value = String::from("by key called");
   |         ----- captured outer variable
94 |     list.sort_by_key(|r| {
   |                      --- captured by this `FnMut` closure
95 |         sort_operations.push(value);
   |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
   |
help: consider cloning the value if the performance cost is acceptable
   |
95 |         sort_operations.push(value.clone());
   |                                   ++++++++

For more information about this error, try `rustc --explain E0507`.
error: could not compile `functional-features` (bin "functional-features") due to 1 previous error
```

- 아래와 같이 가변 참조자를 캡처하는 방식으로 `FnMut`를 구현하게 하면 해결할 수 있음

```rust
let mut sort_operations = vec![];
let mut num_sort_operations = 0;
let value = String::from("by key called");
list.sort_by_key(|r| {
    sort_operations.push(value.clone());
    num_sort_operations += 1;
    r.width
});
println!("{:#?}, sorted in {num_sort_operations} operations", list);
```

```shell
$ cargo run
   Compiling functional-features v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\13_functional-features)
warning: field `height` is never read
   --> src/main.rs:106:5
    |
104 | struct Rectangle {
    |        --------- field in this struct
105 |     width: u32,
106 |     height: u32,
    |     ^^^^^^
    |
    = note: `Rectangle` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
    = note: `#[warn(dead_code)]` on by default

warning: `functional-features` (bin "functional-features") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `target\debug\functional-features.exe`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
], sorted in 6 operations
```

- `Fn` 트레이트는 클로저를 사용하는 함수 혹은 타입을 정의하고 사용할 때 중요

## 13.2 반복자로 일련의 아이템들 처리하기

- 반복자 패턴은 일련의 아이템들에 대해 순서대로 어떤 작업을 수행할 수 있게 함
- 각 아이템을 순회하고 시퀀스 종료 시점을 결정하는 로직을 담당
- `Rust`에서는 단순히 반복자를 생성하는 것으로는 아무 일이 일어나지 않음
- 반복자는 다양한 방법으로 활용될 수 있으며, `Rust`에서는 `for` 루프를 사용할 수 있도록 제공
- Example Code

  ```rust
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();

  for val in v1_iter {
    println!("Got: {}", val);
  }
  ```

- Output
  ```bash
  $ cargo run
    Compiling functional-features v0.1.0 (C:\Users\ghooz\sources\Rust\handbook-rust\13_functional-features)
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
      Running `target\debug\functional-features.exe`
  Got: 1
  Got: 2
  Got: 3
  ```
- 벡터처럼 인덱스를 사용하는 자료구조 외에 많은 다른 케이스에도 사용 가능함

### `Iterator` 트레이트와 `next` 메서드

```rust
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;

  // ...
}
```

- `Rust`에서 모든 반복자는 표준 라이브러리에 정의된 `Iterator` 라는 이름의 트레이트를 구현
- `type Item`, `Self::Item`은 연관 타입(associated type)이라고 부름
  - `Iterator`를 구현하려면 연관 타입도 함께 정의해야 하며, 이 타입이 `next` 메서드의 반환 타입
  - `Item` 타입은 반복자로부터 반환되는 타입 -> `Vec<T>`에서 `T` 타입
- `Iterator`는 딱 하나의 메서드, `next`의 정의를 요구
  - `next` 메서드는 `Option` 타입으로 `Some`으로 감싼 아이템으로 반복 시도, `None` 반환 시 반복 종료를 나타냄

```rust
#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();

  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}
```

- `next`를 호출할 경우, 반복자 내부 상태를 변경하기 때문에 `mut` 변수로 선언해두어야 함
- 즉, `next` 메서드는 반복자를 소비(consume), `for` 루프의 경우에는 루프가 `v1_iter`의 소유권을 갖고 내부에서 가변으로 만들기 때문에 가변일 필요는 없음

> `iter` 메서드는 불변 참조자에 대한 반복자를 반환하며, 소유권을 얻어야 하는 경우 `into_iter`, 가변 참조자에 대한 반복자가 필요하다면 `iter_mut`를 호출

### 반복자를 소비하는 메서드

- `next`를 호출하는 메서드들을 소비 어댑터(consuming adaptor)라고 함
- 호출하면 반복자를 소비하기 때문에, 호출 후 `iter`를 사용할 수 없음

```rust
#[test]
fn iterator_sum() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();

  let total: i32 = v1_iter.sum();
  // `sum`을 호출한 이후에는 `v1_iter`가 소모되므로 더이상 사용할 수 없음
  assert_eq!(total, 6);
}

```

### 다른 반복자를 생성하는 메서드

- 반복자 어댑터(iterator adaptor): `Iterator`에 정의된 메서드로, 반복자를 소비하지 않고 원본 반복자를 다른 반복자로 바꿔서 제공
- Example Code
  ```rust
  let v1 = vec![1, 2, 3];
  v1.iter().map(|x| x + 1); // 각 벡터의 아이템에서 1이 증가한 새로운 반복자 반환
  ```
- Output

  ```bash
  $ cargo run
   Compiling functional-features v0.1.0 (C:\Users\ghooz\sources\Rust\handbook-rust\13_functional-features)
  warning: unused `Map` that must be used
  --> src\main.rs:3:5
    |
  3 |     v1.iter().map(|x| x + 1);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: iterators are lazy and do nothing unless consumed
    = note: `#[warn(unused_must_use)]` on by default
  help: use `let _ = ...` to ignore the resulting value
    |
  3 |     let _ = v1.iter().map(|x| x + 1);
    |     +++++++

  warning: `functional-features` (bin "functional-features") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
      Running `target\debug\functional-features.exe`
  ```

- 에러가 발생하는 이유는 반복자를 소비하지 않았기 때문이며, 반복자를 소비할 필요가 있다는 것을 나타냄

```rust
let v1 = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

- `collect` 메서드는 반복자를 소비하고 결괏값을 모아서 컬렉션 데이터 타입으로 만듬
- `map`은 클로저를 인수로 받기 때문에, 연산의 정의에 대해 자유로워 그 어떤 연산이라도 지정할 수 있음
- 반복자 어댑터의 호출을 연결시키면 복잡한 동작을 읽기 쉬운 방식으로 수행 가능
- **모든 반복자는 게으르므로, 반복자 어댑터를 호출한 결과를 얻기 위해서는 소비 어댑터 중 하나를 호출해야만 함**

> `Iterator` 트레이트가 제공하는 반복 동작을 재사용하면서 클로저로 동작의 일부를 커스터마이징할 수 있게 해주는 대표적인 예시

### 환경을 캡처하는 클로저 사용하기
