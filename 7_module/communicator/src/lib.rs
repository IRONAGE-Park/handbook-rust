pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 모듈의 이름이 작성되고 중괄호 안에 코드 블록이 오는데, 이 블록 안의 모든 것은 이름 공간으로 감싸짐
// `network::connect()`, `client::connect()`와 같이 `::` 문법을 사용하여 특정할 수 있음
// mod network {
//     fn connect() {}
//     mod client {
//         // 모듈들은 계층을 구성할 수 있으며, 지속적인 트래킹을 위해 잘 조직화될 필요가 있음
//         // 프로젝트 내에서 "논리적"으로가 의미하는 것은 작성자에 달려있음
//         fn connect() {}
//     }
//     // 이런 식으로 한 파일 내에 모듈이 여러 개 정의되면 될 수록, 스크롤하여 코드를 찾기 까다로워짐
//     mod server {
//         fn connect() {}
//     }
// }
pub mod network;

pub mod client {
    pub fn connect() {}
}

pub mod client_out;
// 이렇게 모듈을 선언하고는 있지만, 코드 블록을 세미 콜론으로 대체하여 `Rust` 컴파일러에게 내부가 정의된 코드를 다른 위치에서 찾으라고 표시함.
// mod client_out {
//   // contents of client_out.rs
// }
pub mod network_out;

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }

    fn test() {
        inside::inner_function(); // 접근 가능
    }
}

fn try_me() {
    outermost::middle_function();
    // 정상적으로 동작함
    // `middle_function`의 부모 모듈인 `outermost`를 통해 접근 가능

    outermost::middle_secret_function();
    // 컴파일 에러 발생
    // `middle_secret_function`은 비공개이므로, 두 번째 규칙이 적용됨
    // `middle_secret_function`의 현재 모듈도 아니고(`outermost`가 현재 모듈)
    // `middle_secret_function`의 현재 모듈의 자식 모듈도 아님

    outermost::inside::inner_function();
    outermost::inside::secret_function();
    // `inside` 모듈은 비공개고 자식 모듈이 없으므로, 현재 모듈인 `outermost`에 의해서만 접근될 수 있음
    // `try_me` 함수는 `outermost::inside`로 호출할 수 없음
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn test() {
    // use a::series::of;
    use a::series::of::nested_modules;
    // `use` 키워드를 통해 스코프 내에서 호출하고 싶어하는 함수의 모듈을 가져올 수 있음
    // `use` 키워드는 명시한 것만 스코프 내로 가져옴

    // of::nested_modules();
    nested_modules();

    // use TrafficLight::{Red, Yellow};
    // 열거형 또한 모듈과 비슷한 일종의 이름 공간을 형성하고 있기 때문에 `use`를 사용할 수 있음

    use TrafficLight::*; // `*`(glob)를 사용하여 모든 열거형을 가져올 수 있음
                         // 하지만 이는 이름 간의 충돌(naming conflict)의 원인이 될 수도 있음

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}

#[cfg(test)]
mod tests {
    use super::*; // 이 키워드가 없으면 아래의 함수를 사용할 수 없음
                  // 매번 `super` 키워드를 타이핑하여 부모 모듈에 접근하고, 그 후 사용할 함수를 가져오는 것보다는 `use` 키워드를 사용하는 편이 용이함

    #[test]
    fn it_works() {
        let result = add(2, 2);
        super::client::connect();
        assert_eq!(result, 4);
    }
}
