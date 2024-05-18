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
