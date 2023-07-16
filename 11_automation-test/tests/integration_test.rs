use automation_test;
// package(라이브러리)의 이름에 하이픈(-)이 포함될 경우, 언더바(_)로 치환하여 가져올 수 있음
// `tests` 디렉터리의 각 파일은 별개의 크레이트이므로, 각각의 테스트 크레이트의 스코프로 우리가 만든 라이브러리를 가져올 필요가 있음

mod common;

// 통합 테스트 내 코드는 `#[cfg(test]`가 필요 없음
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, automation_test::add_two(2));
}
