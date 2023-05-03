fn main() {
    condition();
    loop_example();
    loop_label();
    while_loop();
    collection_loop();
}

fn condition() {
    let number = 3;
    // 조건과 관련된 코드 블록은 arm 이라고 표현함
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // bool 타입이 아니기 때문에 에러가 발생함
    // 다른 언어와는 달리 bool 타입이 아니면 자동으로 타입을 변환하지 않음
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    // 명령문 위에서도 결과를 반환하고 변수에 할당하는데 사용 가능함.
    let condition = true;
    // 삼항 연산자와 유사한 형태
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // 각 arm 에서의 결과가 될 가능성이 있는 값이 동일한 유형이 아니기 때문에 에러가 발생함
}

fn loop_example() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            // break 키워드와 함께 값을 전달할 수 있음
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
}

fn loop_label() {
    let mut count = 0;
    // 루프 레이블을 사용하고 break 키워드를 통해 종료할 루프를 지정할 수 있음
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // 레이블을 사용하여 바깥쪽 루프를 종료함
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn while_loop() {
    let mut number = 3;
    
    // loop 키워드와 if 키워드의 조합으로 구현하는 것은 매우 일반적이기 때문에 while 키워드를 제공함
    while number != 0 {
        println!("{number}!");
        
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
}

fn collection_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    
    // 이렇게 직접 인덱스를 참조하는 것은 오류가 발생하기 쉽기 때문에 패닉 상태가 될 수 있음
    while index < 5 {
        // println!("the value is: {a[index]}"); // 이렇게 표현하는 것은 안 되는 듯
        println!("the value is: {}", a[index]);
        
        index += 1;
    }
    
    // 이렇게 for 루프를 사용하는 것이 더 좋음 -> 버그 가능성을 제거함
    for element in a {
        println!("the value is: {}", element);
    }
    
    // 1 ~ 4 까지의 숫자를 역순으로 출력함
    for number in (1..4).rev() {
        println!("{number}!");
    }
}