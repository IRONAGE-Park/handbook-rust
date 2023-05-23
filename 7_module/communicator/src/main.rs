// 바이너리 크레이트 작성
extern crate communicator;
// communicator 라이브러리 크레이트를 사용하겠다고 선언
// main.rs는 communicator 라이브러리 크레이트의 루트 모듈이 아니기 때문에, `use communicator;` 문법을 사용할 수 없음
// communicator 라이브러리 밖의(main.rs) 크레이트가 안을 들여다 보는 시점에서, communicator 라는 이름을 갖는 모듈이 크레이트의 최상위 모듈(루트 모듈)이 됨

fn main() {
    // communicator 라이브러리의 client 모듈이 가지고 있는 connect 함수를 호출하면, client` 모듈이 비공개라고 에러를 표시함
    // Rust 언어의 모든 코드의 기본 상태는 비공개로, 다른 사람은 이 코드를 사용할 수 없음
    // 비공개 상태의 코드는 선언 후 파일 내에서 사용하지 않으면, 사용하지 않았다는 에러가 발생하지만, 만약 공개로 변경하면 이 에러가 발생하지 않음
    // 함수를 공개로 변경하기 위해서는 `pub` 키워드를 사용할 수 있음
    communicator::client::connect();
}
