fn main() {
    // let mut x = 5;
    // let y = (x = 6);
    // println!("The value of y is: {y}");
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("The value of x is: {x}");
}

// Rust 언어는 스테이크 케이스로 함수 및 변수의 이름을 지정하는 것을 일반적으로 권장
// 모든 문자는 소문자
// 함수의 정의 위치는 사용하는 곳 앞 뒤 아무 곳이나 상관 없음
fn another_function(value: i32, unit_label: char) {
    println!("The measuremenet is: {value}{unit_label}");
}

// Rust 언어는 표현식 기반 언어
fn statement() {
    let y = 6; // statement
               // let x = (let y = 6); // statement, 명령문은 값을 반환하지 않음
    let y = {
        let x = 3;
        x + 1 // 표현식
              // 끝에 세미 콜론이 없기 때문에 표현식으로 볼 수 있으며, 세미콜론을 추가하면 문으로 변경되고 값을 반환하지 않음
    };
    // 함수 호출과 매크로 호출은 표현식이고, 중괄호로 만든 새 범위 블록은 식
}

// 반환 값이 있는 함수는 화살표를 통해 유형을 선언할 수 있음
// return 키워드를 사용하고 값을 지정하여 함수에서 일찍 반환할 수는 있지만 대부분의 함수는 암시적으로 마지막 식을 반환함
fn five() -> i32 {
    5 // 표현식
      // 'car' <- 타입 에러가 발생함
      // 5; <- 세미 콜론이 있어서 에러가 발생함
}
