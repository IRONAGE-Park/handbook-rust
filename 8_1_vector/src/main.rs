fn create_vector() {
    let v1: Vec<i32> = Vec::new(); // 제네릭으로 구현되어 있기 때문에, 어떠한 값도 넣지 않은 벡터의 경우, 타입 명시가 필요함
    let mut v2 = vec![1, 2, 3];

    // v1.push(5); // mutable이 아니기 때문에 갱신할 수 없음
    v2.push(4);
    v2.push(5);
} // v가 스코프 밖으로 벗어나면, 벡터의 요소들도 해제됨

fn read_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2); // 만약 존재하지 않은 인덱스에 접근할 때 동작을 제어하기 위해서 제공

    // let does_not_exist = &v[100]; // panic 발생
    let does_not_exist = v.get(100); // panic 발생하지 않고, None 반환

    let mut u = vec![1, 2, 3, 4, 5];
    let first = &u[0];

    // u.push(6); // first가 존재하고 있기 때문에 벡터에 요소를 추가할 수 없음
    // 새로운 요소를 벡터의 끝에 추가하는 것은 새로 메모리를 할당하여 예전 요소를 새 공간에 복사하는 일을 필요로 할 수 있는데, 이 경우 에러가 발생할 수 있음
    // u.push(7);

    println!("The first element is: {}", first);
}

fn repeat_vector() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50; // 가변 참조자가 참고하는 값을 바꾸기 위해서, 역참조 연산자(*)를 사용하여 값을 얻어내야 함
    }
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vector() {
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ]; // 열거형을 사용하면, 벡터 내에 다른 타입의 값을 저장하는 것처럼 사용할 수 있음!
}

fn main() {
    create_vector();
    read_vector();
}
