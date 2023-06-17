use std::fs::File;
use std::io::ErrorKind;

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

pub fn file_open_unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
    // hello.txt 파일이 없는 상태에서 이 코드를 실행시키면, unwrap 메서드에 의해 호출된 panic! 으로부터의 에러 메시지를 볼 수 있음

    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\example\result.rs:52:49
}

pub fn file_open_expect() {
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
    // unwrap과 똑같이 사용하지만 에러 메시지도 선택할 수 있도록 해줌

    // thread 'main' panicked at 'hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }', src\example\result.rs:60:33
}
