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