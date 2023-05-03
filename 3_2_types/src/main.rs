use std::io;

fn integer() {
    // 정수 유형은 부호가 있는 것과 없는 것으로 구분됨
    // 부호가 있는 정수는 i로 시작하고, 부호가 없는 정수는 u로 시작함
    // i와 u 모두 크기에 따라 8, 16, 32, 64, 128비트 정수를 저장할 수 있음
    // isize와 usize는 컴퓨터의 아키텍처에 따라 달라지며, 32비트 또는 64비트 정수를 저장할 수 있음
}

fn float() {
    // 부동 소수점 유형은 f32와 f64 두 가지가 있음
    // 기본적으로 f64가 사용되며, f32는 명시적으로 사용해야 함
    // f32는 f64보다 더 빠르게 동작하며(최신 CPU 에서는 거의 차이가 없음), f64는 더 정확함
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

fn number_calculate() {
    let sum = 5 + 10; // 덧셈
    let difference = 95.5 - 4.3; // 뺄셈
    let product = 4 * 30; // 곱셈
    let quotient = 56.7 / 32.2; // 나눗셈
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5; // 나머지
}

fn bool() {
    let t = true;
    let f: bool = false; // 명시적으로 타입을 지정해 줄 수 있음
}

fn character() {
    let c = 'z';
    let z: char = 'Z'; // 명시적으로 타입을 지정해 줄 수 있음
    let heart_eyed_cat = '😻';
    // Rust는 char가 4바이트로 기본적으로 유니코드를 지원합니다.
}

fn tuple() {
   let tup: (i32, f64, u8) = (500, 6.4, 1); // 튜플
    let (x, y, z) = tup; // 튜플의 각 요소를 변수에 할당
    // 튜플의 요소에 접근
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    
    // 값이 없는 튜플은 특별한 이름인 unit 를 가짐
    let unit = ();
    // 빈 값 또는 빈 반환 유형을 나타냄
}

fn array() {
    let a = [1, 2, 3, 4, 5]; // 배열의 길이가 고정되어 있음
    let a1: [i32; 5] = [1, 2, 3, 4, 5]; // 명시적으로 타입과 길이를 지정해 줄 수 있음
    let b = [3; 5]; // [3, 3, 3, 3, 3] 과 같음
    
    let first = a[0]; // 배열의 요소에 접근
    let second = a[1];
    
    println!("Please enter an array index.");
    
    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    // 잘못된 메모리에 참조하려고 시도할 경우, 런타임 오류가 발생함
    let element = a[index];
    
    println!("The value of the element at index {index} is: {element}");
}

fn main() {
    // Rust 언어는 정적으로 타입이 지정되는 언어로, 컴파일 시간에 모든 변수의 타입을 알아야 함.
    // 아래의 `guess` 변수는 `parse()` 함수에서 타입을 추론할 수 없기 때문에 에러가 발생하므로, 타입을 명시해 주어야 함
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    
    array();
}
