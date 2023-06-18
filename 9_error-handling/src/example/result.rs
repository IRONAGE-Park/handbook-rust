use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

// 9.2 - `Result`로 복구 가능한 에러 처리하기
pub fn file_open() {
    let greeting_file_result = File::open("hello.txt");
    // File::open의 반환 타입은 Result<T, E>
    // T = File::open의 구현부에 성공 값인 파일 핸들 std::fs::File
    // E = std::io::Error -> 파일이 존재하지 않거나 접근 권한이 없음

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
        // 파일이 없으면 panic!이 발생하기 때문에 아래와 같은 에러 발생

        // thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\example\result.rs:11:23
    };
    // Option 열거형과 같이 Result 열거형과 배리언트들은 프렐루드로부터 가져와짐
    // 따라서 match 갈래의 Ok와 Err 앞에 Result:: 라고 지정하지 않아도 됨
}

// 9.2 - 서로 다른 에러에 대해 매칭하기
pub fn file_open_match() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 파일이 없어서 실패했다
            ErrorKind::NotFound => match File::create("hello.txt") {
                // 새로운 파일을 만든다
                // File::create도 실패할 수 있으니 내부 match 표현식의 두 번째 갈래 또한 작성해야 함
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    // File::open이 반환하는 Err 배리언트 값의 타입은 io::Error 인데, 이는 표준 라이브러리에서 제공하는 구조체
    // 이 구조체가 제공하는 kind 메서드를 호출하여 io::ErrorKind 값을 얻을 수 있음
}

// 9.2 - `Result<T, E>`와 `match` 사용에 대한 대안
pub fn file_open_match_by_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });
}

// 9.2 - `unwrap` 메서드
pub fn file_open_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
    // hello.txt 파일이 없는 상태에서 이 코드를 실행시키면, unwrap 메서드에 의해 호출된 panic! 으로부터의 에러 메시지를 볼 수 있음

    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\example\result.rs:52:49
}

// 9.2 - `expect` 메서드
pub fn file_open_expect() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    // unwrap과 똑같이 사용하지만 에러 메시지도 선택할 수 있도록 해줌

    // thread 'main' panicked at 'hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\example\result.rs:60:33
}

// 9.2 - 에러 전파하기
// Result<T, E>에서 제네릭 매개변수 T는 구체 타입(concrete type)이고, E는 에러 타입(error type)
// 에러 타입으로 io::Error를 선택한 이유는 File::open 함수와 read_to_string 메서드 두 가지가 모두 io::Error 타입을 반환하기 때문
pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // 함수 전체를 여기서 끝낼 수 있음
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 9.2 - 에러를 전파하기 위한 숏컷: `?`
pub fn read_username_from_file_with_question() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
pub fn read_username_from_file_with_question_shortcut() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    // 이렇게 더 줄일 수도 있음
    Ok(username)
}
pub fn read_username_from_file_with_fs() -> Result<String, io::Error> {
    // 파일에서 문자열을 읽는 코드는 굉장히 흔하게 사용되기 때문에,
    // 표준 라이브러리에서는 파일을 열고, 새 `String`을 생성하고, 파일 내용을 읽고, 내용을 `String`에 집어넣고 반환하는 함수를 제공
    fs::read_to_string("hello.txt")
}

// 9.2 - `?` 연산자가 사용될 수 있는 곳
pub fn greeting_file() -> () {
    // `()`를 반환하는 `main`에서의 `?` 사용 시도는 컴파일 되지 않음
    // let greeting_file = File::open("hello.txt")?;
    //                                            ^ cannot use the `?` operator in a function that returns `()`

    // 이 에러를 고치기 위한 두 가지 선택지
    // 1. `?` 연산자가 사용되는 곳의 값과 호환되게 함수의 반환 타입을 수정하는 것
    //     - 이러한 수정을 막는 제약 사항이 없는 한에서 가능함
    // 2. `Result<T, E>`를 적절한 식으로 처리하기 위해 `match` 혹은 `Result<T, E>`의 메서드 중 하나를 사용하는 것
}
// 이 함수는 문자가 있을 수도, 없을 수도 있기 때문에 `Option<char>`를 반환
pub fn last_char_of_first_line(text: &str) -> Option<char> {
    // `Option<T>` 값일 때 예제
    // `None` 값인 경우 `None`을 일찍 반환
    // `Some` 값이라면 `Some` 안에 있는 값이 이 표현식의 결괏값이 되면서 함수가 지속
    text.lines().next()?.chars().last()
}
pub fn test_main() -> Result<(), Box<dyn Error>> {
    // `Box<dyn Error> 타입은 트레잇 객체(trait object)인데, 지금은 '어떠한 종류의 에러를 의미'한다고 읽으면 됨
    // main 함수의 구현 내용이 `std::io::Error` 타입의 에러만 반환하겠지만,
    // 이 함수 시그니처에 `Box<dyn Error>`라고 명시하면 이후 main 구현체에 다른 에러들을 반환하는 코드가 추가되더라도 계속 올바르게 동작함
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
