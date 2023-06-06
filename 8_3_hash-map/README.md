# 해쉬맵(hash map)

[해쉬맵 Crate](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

- `HashMap<K, V>` 타입은 `K` 타입의 키에 `V` 타입의 값을 매핑한 것을 저장함
- 이 매핑은 해쉬 함수를 통해 동작하는데, 해쉬 함수는 이 키와 값을 메모리 어디에 저장할 지 결정

## 해쉬 함수

- `HashMap`은 기본적으로 서비스 거부 공격(Denial of Service(DOS) attack)에 저항 기능을 제공할 수 있는, 암호학적으로 보안되는 해쉬 함수를 사용
- 가장 빠르지는 않지만 더 나은 보안 제공
- 기본 해쉬 함수가 느리다면 다른 해쉬어를 특정하여 바꿀 수 있음
  - 해쉬어는 `BuildHasher` 트레잇을 구현한 타입