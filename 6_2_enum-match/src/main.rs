#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // 이처럼 값이 존재하는 enum 에서는 매개변수가 존재할(바인딩 될) 수 있음
            println!("State quarter from {:?}!", state);
            25
        } // 각각의 분기 식을 팔(arm)이라고 부릅니다.
          // 각 팔은 쉼표(,)로 다음 팔과 구분됩니다.
          // match 표현식이 실행 되면 각 팔의 패턴과 값을 순서대로 비교하고, 값이 일치하면 해당 패턴과 연결된 코드가 실행됩니다.
          // 각 팔과 관련된 코드는 표현식이며, 일치하는 팔에 있는 표현식의 결과 값은 전체 표현식(match)에 반환되는 값
    }
}

fn plus_in(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Rust 컴파일러는 철저하기 때문에, 만약 패턴 중 하나라도 처리할 수 있는 경우를 다루지 않았다면 에러를 발생합니다.
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // 마지막 패턴이 3과 7을 제외하고 나열되지 않은 모든 패턴을 의미함
        // catch-all arm 이라고도 하며, catch-all arm 이후에 다른 arm 을 추가하면 실행되지 않음
        // 만약 이 포괄적인 패턴에 값을 사용하고 싶지 않으면 _ 를 사용하면 됨
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(other: i32) {}
