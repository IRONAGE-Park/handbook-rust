pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    // format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            )
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // test 모듈 또한 평범한 모듈이므로 내부 모듈인 tests 모듈에서 외부 모듈의 코드를 테스트하려면 먼저 내부 스코프로 가져와야 함

    // 해당 함수가 테스트 함수임을 표시
    // 테스트 실행기는 이 표시를 보고 해당 함수를 테스트로 다룰 수 있게 됨
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // 이 매크로를 사용하여 `result`에 대한 단언
    }
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // 각각의 테스트는 새로운 스레드에서 실행되며, 메인 스레드에서 테스트 스레드가 죽은 것을 알게 되면 해당 테스트는 실패한 것으로 처리
    #[test]
    fn another() {
        // 패닉이 발생하면 테스트는 실패함
        // panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
        // 커스텀 실패 메시지 작성 가능
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_2() {
        Guess::new(200);
    }

    #[test]
    fn it_works_by_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
