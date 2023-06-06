use std::collections::HashMap;

mod quiz;

fn new_hashmap() {
    let mut scores = HashMap::new();
    // 해쉬맵은 빌트인 매크로가 따로 존재하지 않음

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 모든 키와 값은 같은 타입이어야 함

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // zip 함수로 튜플의 벡터를 생성
    // 튜플의 벡터를 collect 함수를 통해 해쉬맵으로 변경
    // 이 경우 타입 명시가 필요한데, Rust 언어는 특정하지 않으면 어떤 것을 원하는지 알 수 없기 때문
}

fn ownership_hashmap() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value는 이 시점에서 더이상 유효하지 않음
    // 참조자를 삽입하면 이 값들은 해쉬맵으로 이동되지 않음
}

fn read_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 해쉬맵의 모든 키와 값을 순회
}

fn update_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // 값 덮어쓰기

    scores.entry(String::from("Yellow")).or_insert(50); // 값이 있을 경우에만 삽입하기
    scores.entry(String::from("Blue")).or_insert(50); // 값이 이미 있으므로 무시됨

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 값이 있을 경우에만 삽입하기
                                                  // 가변 참조자를 반환
        *count += 1; // 참조자를 통해 값을 변경
    } // 단어의 등장 횟수 세기
}

fn main() {
    new_hashmap();
    ownership_hashmap();
    let quiz1_input = [
        1, 2, 3, 4, 5, 2, 3, 4, 1, 2, 3, 1, 2, 3, 4, 5, 5, 1, 2, 1, 0,
    ];
    let quiz1_result = quiz::quiz_1(&quiz1_input);
    println!("quiz 1 input: {:?}", quiz1_input);
    println!("quiz 1 result: {:?}", quiz1_result);

    let quiz2_input = String::from("apple");

    let quiz2_result = quiz::quiz_2(&quiz2_input);
    println!("quiz 2 input: {:?}", quiz2_input);
    println!("quiz 2 result: {:?}", quiz2_result);

    quiz::quiz_3()
}
