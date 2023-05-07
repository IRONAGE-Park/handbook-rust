fn first_word(s: &String) -> usize {
    // 이 함수는 `s`의 변경 여부와 관계가 없음
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    s.len()
}

fn second_word(s: &String) -> &str { // `&str`은 스트링 슬라이스 타입
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn third_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // first_word 함수는 이 방식으로 작성하면 오류 없이 동작할 수'는' 있음
    let mut s = String::from("hello world");
    
    let word = second_word(&s); // 불변 참조가 생성됨
    
    // s.clear(); 
    
    println!("the first word is: {}", word); // 하지만 이렇게 사용하려고 하면 에러가 발생함
    // 변경 이전에 불변 참조가 할당되었고, 이를 변경하려 했기 때문임

    let my_string_literal = "hello world";  // String slice 타입
    // String slice 불변 타입
    
    let my_string = String::from("Hello world");
    
    // first_word 함수는 `String`의 슬라이스에 대한 참조를 받음
    let word = third_word(&my_string[0..6]);
    let word = third_word(&my_string[..]);
    
    let word = third_word(&my_string);
    
    let word = third_word(&my_string_literal[..]);
    let word = third_word(&my_string_literal);
    
    let word = third_word(my_string_literal);
}
