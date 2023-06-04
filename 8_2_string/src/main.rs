fn new_string() {
    let s = String::new();

    let data = "initial contents";

    let s = data.to_string(); // Display Trait이 고현된 어떤 타입이든 사용 가능
    let s = String::from("initial contents"); // 위와 동일한 동작을 수행

    // s.push('a');
    // s.push_str("bar");

    let mut s = String::from("foo");
    s.push_str("bar"); // 파라미터로 스트링 슬라이스를 갖는데, 파라미터의 소유권을 가져올 필요가 없기 때문

    let mut s1 = String::from("foo");
    let s2 = String::from("bar"); // "bar";
    s1.push_str(&s2); // s2는 소유권을 갖지 않기 때문에, 여전히 유효함
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
}

fn comb_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 더이상 사용할 수 없음
                       // s1은 &를 사용하지 않음
                       // s2에서 &를 사용하는 이유는 함수 정의 시그니처와 맞춰야 하기 때문
                       // &String의 타입은 &str로 강제될 수 있음
                       // 이 구문은 s1의 소유권을 가져다가 s2의 내용물의 복사본을 추가한 다음 결과물의 소유권을 반환함
                       // 복사보다 효율적임!

    // println!("s1 is {}", s1); // error
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); // format! 매크로를 사용하면 소유권을 가져가지 않음
    let s = s1 + "-" + &s2 + "-" + &s3; // 코드를 이해하기 어려움
    println!("s is {}", s);
}

fn indexing_string() {
    let s1 = String::from("hello");
    // let h = s1[0]; // error
    // Rust 언어에서 스트링은 인덱싱을 지원하지 않음
    // 언어 자체에서 스트링의 길이가 어떤 글자가 들어가 있느냐에 따라 달라지기 때문
    // 인덱스 연산이 언제나 상수 시간 내에 실행될 것으로 기대하지만, 실제로 그렇게 하기 어렵기도 함
    // Rust 언어는 인덱싱 대신 스트링 슬라이스를 사용함
    let h = &s1[0..1];
    println!("h is {}", h);
    let hello = "Здравствуйте";

    // 슬라이싱의 단위는 바이트
    let s = &hello[0..4];
    let end = &hello[4..]; // 4부터 끝까지

    // let error = &hello[0..1]; // error
    // 2바이트를 차지하는 스트링을 1바이트로 슬라이싱해버렸기 때문
}

fn repeat_string() {
    for c in "नमस्ते".chars() {
        // 6개의 값으로 나누어 반환
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        // 6개의 값으로 나누어 반환
        println!("{}", b);
    }
}

fn main() {
    new_string();
    comb_string();
    indexing_string();
    repeat_string();
}
