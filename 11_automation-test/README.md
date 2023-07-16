# 11. 자동화 테스트 작성하기

테스트의 매커니즘 - 어노테이션, 매크로

## 11.1 테스트 작성 방법

> 테스트란, 테스트할 코드가 의도대로 기능하는지 검증하는 함수
>
> 테스트 함수의 동작
>
> 1. 필요한 데이터나 상태 설정
> 2. 테스트할 코드 실행
> 3. 의도한 결과가 나오는지 확인

### 테스트 함수 파헤치기

- `Rust`에서 테스트: 어노테이션 된 함수
  - 속성: `Rust` 코드 조각에 대한 메타데이터
    - `derive`도 속성 중 하나
- 함수의 `fn` 이전 줄에 `#[test]` 어노테이션을 추가하면 테스트 함수로 변경
- 테스트는 `cargo test` 명령어로 실행되며, 이 명령을 실행하면 `Rust`는 `#[test]` 속성이 표시된 함수를 실행하고 결과를 보고하는 테스트 실행 바이너리를 빌드
  - 인수를 통해 문자열과 이름이 일치하는 테스트만 실행하도록 필터링 가능
- `Cargo`로 새 라이브러리 프로젝트를 생성할 때마다 테스트 함수가 포함된 테스트 모듈이 자동 생성
- `Rust`는 `API` 문서에 작성해놓은 예제 코드도 컴파일 가능

### `assert!` 매크로로 결과 검사하기

- 어떤 조건이 `true`임을 보장하는 테스트를 작성할 땐 표준 라이브러리가 제공하는 `assert!` 매크로가 유용
  - `assert!` 매크로는 `bool` 값으로 평가되는 인수를 전달받음
  - `true`일 경우 통과하지만, `false`일 경우 `panic!`을 호출하여 테스트를 실패하도록 만듬

### `assert_eq!`, `assert_ne!` 매크로를 이용한 동등 테스트

- `assert!` 매크로에 `==` 연산자를 사용한 표현식을 전달하는 방법 외에 `assert_eq!`, `assert_ne!` 매크로를 사용할 수 있음
- 두 매크로는 각각 두 인수를 비교하고 동등(equality)한지, 그렇지 않은(inequality)지 판단
- 실패 시 두 값을 출력하여 테스트의 실패 사유를 더 알기 쉽게 보여줌
  - `assert!` 매크로는 실패 사유를 디테일하게 알기 어려움
- 몇몇 프로그래밍 언어, 프레임워크에서는 동등 단언 함수의 매개변수를 `expected`, `actual`라고 지칭하며, 코드를 작성할 때 인수의 순서를 지켜야 함
- `Rust`에서는 `left`, `right`라고만 지칠할 뿐 순서는 상관없음
- `assert_eq!`, `assert_ne!` 매크로는 실패 시 인수를 디버그할 수 있어야 하기 때문에 `PartialEq`, `Debug` 트레이트를 구현해야 함
  - 모든 기본 타입 및 표준 라이브러리 타입은 이 두 트레이트를 구현함
  - 이 두 트레이트 모두 파생 가능한 트레이트이기 때문에 구조체, 열거형 정의에 `#[derive(PartialEq, Debug)]` 어노테이션 하는 것이 일반적

### 커스텀 실패 메시지 추가하기

- `assert!` 매크로의 인자를 통해 커스텀 실패 메시지를 작성 가능

### `should_panic` 매크로로 패닉 발생 검사하기

- 코드의 반환 값을 검사하는 것에 더하여, 예상한대로 에러 조건을 잘 처리하는지 검사하는 것도 중요함
- `should_panic` 속성을 추가하면 테스트 함수 내부에서 패닉이 발생해야 통과하고, 패닉이 발생하지 않으면 실패함
- `should_panic` 속성만으로는 정확하게 어떠한 이유로 패닉이 발생한건지 파악하기 어려움
  - `expected` 매개변수를 추가하면 실패 메시지를 지정할 수 있음
  - 이 매개변수에 명시할 내용은 패닉 메시지가 얼마나 고유한지 혹은 동적인지, 테스트에 요구되는 정확성에 따라 달라짐

### `Result<T, E>`를 이용한 테스트

- 테스트할 함수가 `Result<T, E>`를 반환하는 방식으로도 테스트를 작성할 수 있음
- 패닉을 발생시키는 대신 `Err`을 반환하여 실패 확인
- `?` 연산자를 사용할 수 있기 때문에 작성하기 편리
- `#[should_panic]` 어노테이션을 사용할 수 없음
  - 연산이 `Err` 배리언트를 반환하는 것을 단언하기 위해서는 물음표 연산자가 아니라 `assert!(value.is_err())`을 사용할 수 있음

---

## 11.2 테스트 실행 방법 제어하기

- `cargo run` 명령어가 코드를 컴파일하고 생성된 바이너리를 실행하는 것과 같이, `cargo test` 명령어는 코드를 테스트 모드에서 컴파일하고 생성된 바이너리를 실행
  - `cargo test`에 의해 생성된 바이너리의 기본 동작은 모든 테스트를 병렬로 실행하고 테스트가 수행되는 동안 발생된 출력을 캡처하는 것
  - 출력이 표시되는 것을 막고 테스트 결과와 관련된 출력을 읽기 편하게 해줌
  - 커맨드 라인 옵션을 통해 기본 동작 변경 가능
- 명령어 옵션은 `cargo test`에 전달되는 것도 있고, 테스트 바이너리에 전달되는 것도 있음
  - 이를 구분하기 위해 `cargo test`에 전달할 인수를 먼저 나열
  - `--` 구분자(separator)를 씀
  - 그 뒤에 테스트 바이너리에게 전달할 인수 나열

### 테스트를 병렬 혹은 순차적으로 실행하기

- 여러 테스트를 실행할 때는 기본적으로 스레드를 사용해 병렬 실행
- 동시에 실행되므로, 각 테스트가 공유 상태(공유 자원, 현재 작업 디렉터리, 환경 변수 등)을 갖거나 다른 테스트에 의존해서는 안됨
  - 동시에 같은 곳에 참조할 일이 있을 경우, 참조할 위치를 피하거나 순차적으로 실행하는 해결책이 있음
- `--test-threads` 플래그와 함께 테스트 바이너리에서 사용할 스레드 개수를 지정할 수 있음

```shell
# 스레드 개수를 1로 설정 => 병렬 처리를 사용하지 않음
$ cargo test -- --test-threads=1
```

### 함수 출력 표시하기

- `Rust` 테스트 라이브러리는 기본적으로 성공한 테스트의 모든 표준 출력(standard output)을 캡처
- 테스트가 성공하면 출력을 찾아볼 수 없음
- 테스트가 실패하면 표준 출력으로 출력됐던 모든 내용이 실패 메시지 아래 출력
- 성공한 테스트에서 출력한 내용도 보고싶다면 `--show-output` 플래그 사용

```shell
$ cargo test -- --show-output
```

```shell
$ cargo test -- --show-output
   Compiling silly-function v0.1.0 (file:///projects/silly-function)
    Finished test [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests src/lib.rs (target/debug/deps/silly_function-160869f38cff9166)

running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

### 이름을 지정해 일부 테스트만 실행하기

- 테스트 모음을 전부 실행하는 데 시간이 오래 걸리기도 하므로, 코드의 특정한 부분에 대한 작업 중이라면 해당 부분의 코드에 관련된 테스트만 실행하고 싶을 수도 있음
- 테스트의 이름을 인수로 넘겨 어떤 테스트를 실행할지 선택 가능
- `cargo test` 명령어에 테스트 함수 이름을 전달하여 해당 테스트만 실행 가능
- 실행 시 요약 라인에서 실행한 테스트 이외에도 다른 테스트가 존재함을 알려줌
- 이 명령어는 첫 번째 값만 사용하므로 여러 개 이름을 지정할 수 없음

```shell
$ cargo test <test-function-name>
```

```shell
$ cargo test one_hundred
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

### 테스트를 필터링하여 여러 테스트 실행하기

- 테스트 이름의 일부만 지정하면 해당 값이 맞는 모든 테스트가 실행됨
- `add_function1`이라는 테스트와 `add_function2`라는 테스트가 있을 때, `cargo test add` 명령어로 두 테스트를 모두 실행 가능
- 테스트가 위치한 모듈도 테스트 이름의 일부로 나타나므로, 모듈 이름으로 필터링하면 해당 모듈 내 모든 테스트를 실행 가능

```shell
$ cargo test add
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.61s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

### 특별 요청이 없다면 일부 테스트 무시하기

- 간혹 몇몇 테스트는 실행하는 데 굉장히 오랜 시간이 걸려 제외하고 싶을 수 있음
- `ignore` 속성을 어노테이션하여 해결할 수 있음
- `ignore` 표시된 테스트는 `cargo test -- --ignored` 명령어를 사용하여 실행 가능

```shell
$ cargo test -- --ignored

# 모든 테스트를 실행하는 명령어
$ cargo test -- --include-ignored
```

```shell
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.60s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 11.3 테스트 조직화

- `Rust`는 테스트를 두 종류로 나눔
  1. 유닛 테스트(Unit Test, 단위 테스트)

  - 작고 집중적
  - 한 번에 하나의 모듈만 테스트
  - 모듈의 비공개 인터페이스도 테스트 가능

  2. 통합 테스트(Integration Test)

  - 라이브러리 외부에 위치하여, 외부에서 코드를 사용할 때와 똑같은 방식
  - 하나의 테스트에서 여러 모듈이 사용되기도 함

### 유닛 테스트

- 유닛 테스트의 목적
  - 각 코드 단위를 나머지 코드와 분리하여, 제대로 작동하지 않는 코드가 어느 부분인지 빠르게 파악하는 것
  - `src` 디렉터리 내 각 파일에 테스트 대상이 될 코드와 함께 작성
  - 각 파일에 `tests` 모듈을 만들고 `cfg(test)` 어노테이션하는 게 일반적인 관례

#### 테스트 모듈과 `#[cfg(test)]`

- `#[cfg(test)]` 어노테이션은 이 코드가 `cargo build` 명령어가 아닌 `cargo test` 명령어 실행 시에만 컴파일 및 실행될 것을 `Rust`에 전달
- 라이브러리 빌드 시 테스트 코드는 제외되므로, 컴파일 소요 시간, 결과물 크기도 줄어듬
- 유닛 테스트는 일반 코드와 같은 파일에 위치하기 때문에, `#[cfg(test)]` 어노테이션을 작성해 컴파일 결과물에 포함되지 않도록 명시해야 함
- `cfg` 속성은 설정(Configuration)을 의미하며, `Rust`는 이 아이템을 특정 설정 옵션 적용 시에만 포함
  - 옵션 값이 `test`이기 때문에, `cargo test` 명령어를 실행할 때만 테스트 코드를 컴파일
  - `#[test]` 어노테이션 뿐만 아니라, 모듈 내 도우미 함수도 포함

#### 비공개 함수 테스트하기

> 비공개 함수 테스트에 대해서는 많은 논쟁이 있어 다른 언어에서는 테스트하기 어렵거나 불가능한 경우도 있음

- `pub`으로 표시되지 않은 함수도 참조하여 테스트할 수 있음

### 통합 테스트

- 통합 테스트는 라이브러리와 완전히 분리되어 있음
- 통합 테스트는 외부 코드와 마찬자기로, `public` `API`만 호출 가능
- 통합 테스트의 목적
  - 라이브러이의 여러 부분을 함께 사용했을 때 제대로 작동하는지 확인하는 것
- 각각 따로 사용했을 때 잘 작동하는 코드도 함께 사용할 때는 문제가 발생할 수 있기 때문에 통합 테스트도 중요
- 통합 테스트를 작성하기 위해서는 `tests` 디렉터리를 만들어야 함

#### `tests` 디렉터리

- 프로젝트 디렉터리 최상위, `src` 옆에 `tests` 디렉터리를 생성
- `cargo`는 디렉터리 내 통합 테스트 파일을 자동으로 인식
  - 원하는 만큼 통합 테스트 파일을 만들 수 있고, `cargo`는 각 파일을 개별 크레이트로 컴파일

```shell
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished test [unoptimized + debuginfo] target(s) in 1.31s
     Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

- 어떤 절 안에 어떠한 테스트라도 실패하면, 그다음 절(유닛 - 통합 - 문서)은 실행되지 않음
- 각각 통합 테스트 파일은 별도의 출력 절을 생성하므로, `tests` 디렉터리에 파일을 추가하면 통합 테스트 절이 더 만들어짐
- 통합 테스트도 `cargo test` 명령어에 테스트 함수명을 인수로 전달해 특정 통합 테스트 함수를 실행할 수 있음

```shell
# integration_test.rs 파일 내의 테스트만 실행
$ cargo test --test integration_test
```

#### 통합 태스트 내 서브 모듈

- 통합 테스트를 추가하다 보면, 조직화를 위해 `tests` 디렉터리에 더 많은 파일이 필요할 수 있음
- `tests` 내 각 파일은 각각의 크레이트로 컴파일되는데, 각 통합 테스트 파일이 각각의 크레이트로 취급된다는 점 대문에 만든 크레이트를 사용할 실제 사용자처럼 분리된 스코프를 만들어 내는 데에 유용
- 하지만, `src` 디렉터리 내에서 코드를 모듈과 파일로 분리하여 동일한 동작을 공유하는 것을 `tests` 디렉터리에서는 불가능
- 테스트 파일에서 유용하게 사용할 도우미 함수 묶음을 공통 모듈로 분리하려 할 때, `tests` 디렉터리의 동작 방식은 걸림돌
- 이 경우 `common/mod.rs`와 같이 파일을 구성하면 공통 함수를 테스트에 포함시키지 않고 작성할 수 있음

#### 바이너리 크레이트에서의 통합 테스트

- `src/lib.rs` 파일이 없고 `src/main.rs` 파일만 있는 바이너리 크레이트라면, `tests` 디렉터리에 통합 테스트를 만들어서 `src/main.rs` 파일에 정의된 함수를 `use` 구문으로 가져올 수 없음
- 다른 크레이트에서 사용할 수 있도록 함수를 노출하는 건 라이브러리 크레이트 뿐
  - 바이너리 크레이트는 자체적으로 실행되게 되어 있음
- 바이너리를 제공하는 `Rust` 프로젝트들이 `src/main.rs` 파일은 간단하게 작성하고, 로직은 `src/lib.rs` 파일에 위치시키려는 이유가 바로 이것
  - 이런 구조로 작성하면 중요 기능을 통합 테스트에서 `use` 구문으로 가져와 테스트 가능

# 정리

- `Rust`의 테스트 기능을 사용하면 코드가 어떻게 작동해야 하는지 명시하여, 코드를 변경하더라도 계속하여 의도대로 작동함을 보장할 수 있음
- 유닛 테스트
  - 비공개 세부 구현을 포함한 라이브러리의 각 부분이 별도로 잘 작동하는지 확인
- 통합 테스트
  - 외부 코드가 라이브러리를 사용하는 것과 동일한 방식으로 라이브러리 공개 `API`를 이용하여 라이브러리의 여러 부분이 함께 사용될 때 제대로 작동하는지 확인
- `Rust`의 타입 시스템과 소유권 규칙이 일부 버그를 방지해주긴 하지만, 논리 버그를 제거하기 위해선 테스트도 마찬가지로 중요