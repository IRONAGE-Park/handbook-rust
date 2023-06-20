// 10.3 - 라이프타임으로 댕글링 참조자 방지하기
pub fn test_main_1() {
    // let r;
    //
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //
    // println!("r: {}", r); // 이미 참조 값이 스코프를 벗어나 사용할 수 없음
}
// error[E0597]: `x` does not live long enough
// --> src\example\lifetime.rs:7:13
//    |
// 6  |         let x = 5;
//    |             - binding `x` declared here
// 7  |         r = &x;
//    |             ^^ borrowed value does not live long enough
// 8  |     }
//    |     - `x` dropped here while still borrowed
// 9  |
// 10 |     println!("r: {}", r);
//    |                       - borrow later used here

// 10.3 -  함수에서의 제네릭 라이프타임
pub fn test_main_2() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// fn longest(x: &str, y: &str) -> &str { // 여기서 발생하는 에러는 반환 타입에 제네릭 라이프타임 매개변수가 필요하다는 내용
//                                        // 반환할 참조자가 `x`인지 `y`인지 `Rust`가 알 수 없기 때문
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// error[E0106]: missing lifetime specifier
// --> src\example\lifetime.rs:33:33
//    |
// 33 | fn longest(x: &str, y: &str) -> &str {
//    |               ----     ----     ^ expected named lifetime parameter
//    | = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
//    |
// 33 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//    |           ++++     ++          ++          ++

// 10.3 - 함수 시그니처에서 라이프타임 명시하기
// 시그니처에서 다음과 같은 제약사항 표현
// - 두 매개변수의 참조자가 모두 유효한 동안에는 반환된 참조자도 유효할 것
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
    // 이 함수 시그니처는 두 매개변수가 적어도 라이프타임 `'a`만큼 살아있는 문자열 슬라이스이며,
    // 반환하는 문자열 슬라이스도 라이프타임 `'a`만큼 살아있다는 정보를 알려줌
}
pub fn test_main_3() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        // 정상 실행
        println!("The longest string is {}", result);
    }
}
pub fn test_main_4() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // 에러 발생
    // println!("The longest string is {}", result);
}

// 10.3 - 라이프타임의 측면에서 생각하기
pub fn longest_1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// pub fn longest_2<'a>(x: &str, y: &'a str) -> &'a str {
//     // 반환 타입에 `'a`를 지정했지만, 그 어떤 매개변수와도 관련이 없으므로 컴파일 불가
//     let result = String::from("really long string");
//     result.as_str()
// }

// 10.3 - 구조체 정의에서 라이프타임 명시하기
struct ImportantExcerpt<'a> {
    // `ImportantExcerpt` 인스턴스는 `part` 필드가 보관하는 참조자의 라이프타임보다 오래 살 수 없다
    part: &'a str,
}
pub fn test_main_5() {
    let novel = String::from("Call me Ishmeal. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
pub fn test_other() {
    let novel = String::from("Call me Ishmeal. Some years ago...");
    // novel이 참조자의 본체이기 때문에 novel의 스코프에 영향을 받게 됨
    let i;
    {
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{}", i.part);
}

// 10.3 - 메서드 정의에서 라이프타임 명시하기
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    // `ImportantExcerpt` 인스턴스는 `part` 필드가 보관하는 참조자의 라이프타임보다 오래 살 수 없다
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        // `announcement`를 반환할 수 없음
        self.part
    }
}
