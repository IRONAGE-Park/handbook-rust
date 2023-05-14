enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 위와 같은 코드를 아래처럼 작성할 수 있음
    if let Some(max) = config_max {
        // 값이 패턴(`Some(max)`)과 일치하지 않으면 블록의 코드가 실행되지 않음
        // 첫 번째 팔과 같음
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // 위와 같은 코드를 아래처럼 작성할 수 있음
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
