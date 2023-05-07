fn main() {
    let mut s = String::from("hello");
    
    let len = calculate_length(&s);// &s1을 통해 s1의 참조를 넘겨줌
    // 해당 키워드를 사용하면 참조는 하지만 소유하지 않는 것을 만들 수 있음
    // 소유하지 않기 때문에 참조가 사용을 중지해도 할당이 해제되지 않음
    
    println!("The length of '{}' is {}.", s, len);
    
    let mut s1 = &mut s;
    println!("{s1}");
    let mut s2 = &mut s;
    println!("{s2}");
    
    change( &mut s);
}

fn calculate_length(s: &String) -> usize { // & 키워드를 통해 참조 타입 매개변수를 선언할 수 있음
    // 포인터와 유사하지만, 포인터와는 달리 해당 참조의 수명 동안 특정 유형의 유효한 값을 가리키도록 보장됨
    s.len()
} // 소유권이 없기 때문에 s 변수는 할당이 해제되지 않음
// 참조는 기본적으로 변경할 수 없음(소유하고 있지 않기 때문에)


fn change(some_string: &mut String) {
    some_string.push_str(", world");
    // 참조는 변경할 수 없기 때문에 컴파일 에러 발생
}

fn reference_check() {
    let mut s = String::from("hello");
    
    let r1 = &s; // 불변 참조
    let r2 = &s; // 불변 참조
    println!("{r1} and {r2}");
    
    // 불변 참조를 사용하는 부분이 모두 지났기 때문에 가변 참조를 생성할 수 있음
    let r3 = &mut s; // 가변 참조
    println!("{r3}");
}

fn dangling() -> String {
    let s = String::from("hello"); // 해당 변수는 이 함수 내에서 생성되기 때문에, 함수가 종료되면 할당이 해제됨
    
    // &s // s의 참조를 반환
    s // s를 반환
} // s는 메모리에 반환되기 때문에 dangling 참조가 됨
