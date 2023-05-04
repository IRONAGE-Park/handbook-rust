fn normal_scope1() {
    let s ="hello";
}

fn normal_scope2() {
    let s = String::from("hello");
    
    // s.push_str(", world!")
}

fn example_move() {
    let x = 5;
    let y = x;
    
    let s1 = String::from("hello");
    let s2 = s1;
    
    // println!("{s1}, world!"); // error
}

fn example_params() {
    let s = String::from("hello");
    takes_ownership(s);
    
    let x = 5;
    makes_copy(x);
}

fn example_return() {
    let s1 = gives_ownership(); // 반환값을 s1으로 이동시킵니다.

    let s2 = String::from("hello"); // s2가 스코프 안으로 들어왔습니다.

    let s3 = takes_and_gives_back(s2); // s2는 takes_and_gives_back 안으로 이동되었고,
    // 반환값은 s3으로 이동되었습니다.
} // 여기서 s3은 스코프 밖으로 벗어났고, `drop`이 호출됩니다. s2도 스코프 밖으로 벗어났지만
// 이동되었으므로 아무 일도 일어나지 않습니다. s1도 스코프 밖으로 벗어나 drop이 호출됩니다.

fn example_return2() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn main() {
    println!("Hello, world!");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 메소드는 스트링의 길이를 반환합니다.

    (s, length)
}
