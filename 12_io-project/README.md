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

#### 에러 메시지 개선

- 기존의 에러 메시지는 아래와 같음
  ```shell
  thread 'main' panicked at src\main.rs:23:21:
  index out of bounds: the len is 1 but the index is 1
  ```
- 개발자에게 행동을 유도할 수 있는 다른 메시지를 부여함
  ```rust
  impl Config {
    pub fn new(args: &[String]) -> Self {
      if args.len() < 3 {
        panic!("not enough arguments");
      }
    }
  }
  ```
- 아까보다는 나은 코드가 되었지만, 여전이 개발자에게 필요 없는, `panic!`에 의한 추가적인 정보가 제공
- `panic!`을 호출하는 것은 사용 방법의 문제가 아니라, 프로그램의 문제에 적합하므로 `Result`를 사용하도록 함

#### `panic!` 호출 대신 `Result` 반환하기

- 일반적으로 `new`라는 생성자는 실패하지 않으리라 예상하기 때문에, 함수 이름을 `build`로 수정
- `Result` 타입에 에러 메시지를 담아 `panic!`의 호출로 인한 `thread 'main' ... RUST_BACKTRACE`에 대한 텍스트를 제거할 수 있음

```rust
impl Config {
  pub fn build(args: &[String]) -> Result<Self, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(Config { query, file_path })
  }
}
```

#### `Config::build` 호출과 에러 처리

- `Config::build`를 사용하는 함수 내에서 직접 `Result` 타입에 대한 에러를 처리하고, 에러 발생 시 직접 `0`이 아닌 에러 코드로 종료함
- `0`이 아닌 종료 상태 값은 프로그램을 호출한 프로세스에게 에러 상태 값과 함께 종료되었음을 알려주는 관례

```rust
fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::build(&args).unwrap_or_else(|err| {
    // `Result`가 `Err`를 반환했을 경우 호출하는 클로저
    // `panic!`이 아닌 에러 처리를 정의할 수 있음
    println!("Problem parsing arguments: {err}");
    std::process::exit(1); // `panic!`과 유사하지만 추가 출력문이 사라짐
  });
}
```

```shell
Problem parsing arguments: not enough arguments
```

### 12.3.3 `main`으로부터 로직 추출하기

- 프로그램 로직에서, `main` 함수에 있는 로직 중 설정 값이나 에러 처리와는 관련되지 않은 모든 로직을 `run`이라는 함수로 추출
  - `main`은 간결하고 검사하기 쉬워짐

```rust
fn run(config: Config) {
  let contents =
    fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

  println!("With text:\n{contents}")
}
```

#### `run` 함수로부터 에러 반환하기

- 현재 `run` 함수는 뭔가 잘못되면 `expect`를 호출하여 프로그램이 패닉이 되도록 함
  - 대신 `Result<T, E>`를 반환하도록 할 수 있음

```rust
// 트레이트 객체 `Box<dyn std::error::Error>`를 반환
// -> 이 함수가 `Error` 트레이트를 구현한 어떤 타입을 반환하는데, 그 반환 값이 구체적으로 어떤 타입인지 특정하지 않아도 됨
// 서로 다른 에러의 경우에서 서로 다른 타입이 될지도 모르는 에러 값을 반환하는 유연성을 제공
// `dyn` 키워드는 dynamic의 줄임말
fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  println!("With text:\n{contents}");

  // 원래 `()`를 반환하여 생략 가능했음
  // `run` 함수의 타입 시그니처는 `()`로 선언되었는데, 유닛 타입 값을 `Ok` 값으로 감쌀 필요가 있음을 의미
  // `Ok(())` 문법은 이상해보일 수 있지만, 부작용에 대해서만 처리하겠다는 것을 가리키는 자연스러운 방식
  Ok(())
}
```

- 컴파일러가 `main`에서 처리하지 않은 `run` 함수의 반환 `Result`에 대해 경고를 줌
- `Result` 값이 무시되고 있으며, 어떤 에러 처리가 필요하지 않은지 상기시켜주는 역할

#### `main`에서 `run`으로부터 반환된 에러 처리하기

```rust
fn main() {
  if let Err(e) = run(config) {
    println!("Application error: {e}");
    std::process::exit(1);
  }
}
```

- `Cargo::build`와 유사한 기술을 통해 에러를 검사하고 처리하는데, `unwrap_or_else` 대신 `if let`을 사용
- `run` 함수가 반환한 값은 `()`이기 때문에, 실패한 경우만 신경쓰면 되므로 `unwrap_or_else`가 필요 없어짐
- `if let`과 `unwrap_or_else`는 모두 에러를 출력하고 종료하는 역할

### 12.3.4 라이브러리 크레이트로 코드 쪼개기

- 코드를 쪼개면 테스트하기 용이하고, `main.rs` 파일의 책임 소재를 더 적게할 수 있음
- 옮길 부분
  - `run` 함수 정의 부분
  - 이와 관련된 `use` 구문들
  - `Config` 정의 부분
  - `Config::build` 함수 정의 부분
- `pub` 키워드를 사용하여 외부에서 라이브러리 크레이트에 접근할 수 있도록 함
- `lib.rs`

  ```rust
  pub struct Config {
    pub query: String,
    pub file_path: String,
  }

  impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
      // --생략--
    }
  }

  pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --생략--
  }
  ```

- `main.rs`

  ```rust
  use io_project::Config;

  fn main() {
    // --생략--
    if let Err(e) = minigrep::run(config) {
      // --생략--
    }
  }
  ```

- 이후 테스트를 작성하여 모듈성의 이점을 활용할 수 있음

## 12.4 테스트 주도 개발로 라이브러리 기능 개발하기
