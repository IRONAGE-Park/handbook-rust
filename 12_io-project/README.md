# 12. I/O 프로젝트: 커맨드 라인 프로그램 만들기

- `grep`(globally search a regular expression and print) 만들기

## 12.1 커맨드 라인 인수 받기

- 아래와 같은 인수를 처리하기 위한 기능을 구현
  1. 인수가 나올 것임을 알려주는 두 개의 하이픈
  2. 검색을 위한 문자열
  3. 검색하길 원하는 파일명

```shell
$ cargo run -- searchstring example-filename.txt
```

### 12.1.1 인수 값 읽기

- 커맨드 라인 인수로 전달된 값들을 읽을 수 있도록, `Rust` 표준 라이브러리가 제공하는 `std::env::args` 함수를 사용함.
  - 이 함수는 커맨드 라인 인수의 반복자(`Iterator`)를 반환
  - 반복자는 일련의 값들을 생성하고, 반복자의 `collect` 메서드를 호출하여 반복자가 생성하는 모든 요소를 담고 있는 벡터 같은 컬렉션으로 치환 가능
  - `collect` 함수를 사용하여 다양한 종류의 컬렉션을 만들 수 있으므로, 문자열의 벡터가 필요하다는 것을 명시하기 위해 `args`의 타입을 명시적으로 표기
  - `collect`는 타입 표기가 자주 필요한 함수 중 하나

```rust
use std::env;
// `use std::env::args`를 가져와 `args`로 호출하는 것보다 명확하게(모호하지 않게) 사용하도록 함
// 또한, 다른 `std::env` 내의 함수도 쉽게 사용할 수 있도록 하기 위함

fn main() {
  let args: Vec<String> = env::args().collect();
  dbg!(args):
}
```

> `args` 함수와 유효하지 않은 유니코드
>
> `std::env::args` 함수는 유효하지 않는 유니코드가 들어있을 경우 패닉을 일으킴
> `std::env::args_os` 함수를 통해 유효하지 않는 유니코드를 받아들이는 `OsString`을 사용할 수 있으나, 운영체제 별로 다루는 방법이 다르고 복잡할 수 있음

- 실행 결과

  ```shell
  $ cargo run
  Compiling io-project v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\12_io-project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target\debug\io-project.exe`
  [src\main.rs:5:5] args = [
      "target\\debug\\io-project.exe",
  ]
  ```

  ```shell
  $ cargo run -- neddle haystack
  Compiling io-project v0.1.0 (C:\Users\ghooz\source\Rust\handbook-rust\12_io-project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `target\debug\io-project.exe`
  [src\main.rs:5:5] args = [
    "target\\debug\\io-project.exe",
    "neddle",
    "haystack",
  ]
  ```

- 벡터의 첫 번째 값이 `"target\\debug\\io-project"`, 즉 바이너리 파일의 이름으로, 이는 `C`에서의 인수 리스트 동작과 일치함.
  - 프로그램의 이름은 종종 편리하게 이용될 수 있음

### 12.1.2 인수 값들을 변수에 저장하기

- 첫 번째(`args[0]`)는 프로그램(바이너리 파일)의 이름이므로 그 이후 인수부터 시작함

```rust
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  let query = &args[1];
  let file_path = &args[2];

  println!("Searching for {}", query);
  println!("In file {}", file_path);
}
```

- 실행 결과

  ```shell
  $ cargo run -- test sample.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\io-project.exe test sample.txt`
  Searching for test
  In file sample.txt
  ```

## 12.2 파일 읽기

```rust
// 파일 읽는 코드 작성
use std::fs;

fn main() {
  // --생략--
  let contents = fs.read_to_string(file_path).expect("Should have been abel to read the file");

  println!("With text:\n{contents}");
}
```

- 실행 결과

  ```shell
  $ cargo run -- the poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\io-project.exe the poem.txt`
  Searching for the
  In file poem.txt
  With text:
  I'm nobody! Who are you?
  Are you nobody, too?
  Then there's a pair of us - don't tell!
  They'd banish us, you know.

  How dreary to be somebody!
  How public, like a frog
  To tell your name the livelong day
  To an admiring bog!
  ```

- 현재 코드의 문제점
  - 현재 `main` 함수에는 여러가지 기능이 있음
    - 일반적으로 함수 하나당 단 하나의 아이디어에 대한 기능을 구현할 때 함수가 더 명료해지고 관리하기 쉬워짐
  - 처리 가능한 수준의 에러 처리를 하고 있지 않음
    - 작을 땐 문제가 되지 않으나, 커지면 이 문제들을 깔끔하게 고치리 어려워질 것임
- 작은 양의 코드를 리팩터링하는 것이 훨씬 쉽기 때문에 일찍 리팩터링 하는 것은 좋은 관행

## 12.3 모듈성과 에러 처리 향상을 위한 리팩터링

- 해결해야 할 네 가지 문제점

1. `main` 함수에서 목적 추출 및 분리

- 하나의 함수에 여러 목적이 포함되어 있으면, 함수의 본래 목적 추론, 테스트, 리팩터링이 어려워짐

2. 로직을 위한 변수 구조체로 분리

- 변수가 스코프 내에 여러 개 선언되어 있을 경우, 해당 변수의 역할이 혼란스러울 수 있음
- 설정 변수들을 하나의 구조체로 묶어서 목적을 분명히 하는 것이 좋음

3. 에러 메시지의 정보 품질 향상
4. 에러 처리 로직 한 곳에 통합

### 12.3.1 바이너리 프로젝트에 대한 관심사 분리

- `main`이 커지기 시작할 때 바이너리 프로그램의 별도 관심사를 나누기 위한 가이드라인
  - 프로그램을 `main.rs`와 `lib.rs`로 분리하고 프로그램 로직을 `lib.rs`에 옮기세요.
  - 커맨드 라인 파싱 로직이 작은 동안에는 `main.rs`에 남아있을 수 있습니다.
  - 커맨들 라인 파싱 로직이 복잡해지기 시작하면, `main.rs`로부터 추출하여 `lib.rs`로 옮기세요.
- 이 과정을 거친 후 `main` 함수에 남아있는 책임소재는 다음으로 한정되어야 함
  - 인수 값을 가지고 커맨드 라인 파싱 로직 호출하기
  - 그 밖의 설정
  - `lib.rs`의 `run` 함수 호출
  - `run`이 에러를 반환할 때 에러 처리하기

#### 인수 파서 추출

```rust
fn parse_config(args: &[String]) -> (&str, &str) {
  let query = &args[1];
  let file_path = &args[2];

  (query, file_path)
}

```

- `main`은 더이상 커맨드 라인 인수와 변수들이 어떻게 대응되는지를 결정할 책임이 없음
- 이러한 리팩터링은 조금씩 점진적으로 변해가며, 한 단계를 거칠 때마다 기능이 여전히 동작하는지 검증하면 좋음

#### 설정 값 묶기

```rust
struct Config {
  query: String,
  file_path: String,
}

fn parse_config(args: &[String]) -> Config {
  let query = args[1].clone();
  let file_path = args[2].clone();

  Config { query, file_path }
}
```

- 기존에 반환하고 있던 튜플을 바로 개별 부분으로 즉시 쪼개는 것은, 적절한 추상화가 이루어지지 않았다는 신호
- 하나의 구조체에 넣어 필드에 각각 의미있는 이름을 부여
- 이 과정에서 필요한 `String` 데이터 관리 방법은 다양하며, 가장 쉬운 방법은 복사본을 만드는 것
  - 참조자를 지정하는 것에 비해 더 많은 비용을 지불하나, 라이프타임 관리가 필요없어 직관적임
  - 이러한 환경에서 약간의 성능을 포기하고 단순함을 얻는 것은 가치 있는 절충안

> `clone`을 사용한 절충안
>
> 많은 러스트 개발자들이 런타임 비용의 이유로 `clone`을 사용한 소유권 문제 해결을 회피하는 경향  
> 그러나 처음부터 최적화된 코드 작성을 시도하기보다는 다소 비효율적이더라도 동작하는 프로그램을 만드는 편이 좋음

#### `Config`를 위한 생성자 만들기

- `parse_config` 함수의 목적이 `Config`를 생성하는 것이므로 자연스럽게 `Config`와 연관된 `new`라는 이름의 함수로 바꿀 수 있음

```rust
impl Config {
  pub fn new(args: &[String]) -> Self {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
  }
}
```

### 12.3.2 에러 처리 수정
