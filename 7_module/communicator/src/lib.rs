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
mod network;

mod client {
    fn connect() {}
}

mod client_out;
// 이렇게 모듈을 선언하고는 있지만, 코드 블록을 세미 콜론으로 대체하여 `Rust` 컴파일러에게 내부가 정의된 코드를 다른 위치에서 찾으라고 표시함.
// mod client_out {
//   // contents of client_out.rs
// }
mod network_out;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
