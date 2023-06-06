use std::collections::HashMap;

// Q: 정수 리스트가 주어졌을 때, 벡터를 이용하여 이 리스트의 평균값, 중간값, 최빈값을 반환하는 함수를 작성하라.
pub fn quiz_1(array: &[i32]) -> (i32, i32, i32) {
    let vec = Vec::from(array);

    let sum = vec.iter().sum::<i32>();
    let medium = vec[vec.len() / 2];
    let mut map = HashMap::new();

    vec.iter().for_each(|&x| {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    });

    let mut mode = (-1, -1);
    for (key, value) in &map {
        if *value > mode.1 {
            mode = (*key, *value);
        }
    }

    (sum, medium, mode.0)
}

// 스트링을 피그 라틴(pig Latin)으로 변경하라.
pub fn quiz_2(str: &str) -> String {
    let map = HashMap::from([
        ("a", false),
        ("e", false),
        ("i", false),
        ("o", false),
        ("u", false),
        ("b", true),
        ("c", true),
        ("d", true),
        ("f", true),
        ("g", true),
        ("h", true),
        ("j", true),
        ("k", true),
        ("l", true),
        ("m", true),
        ("n", true),
        ("p", true),
        ("q", true),
        ("r", true),
        ("s", true),
        ("t", true),
        ("v", true),
        ("w", true),
        ("x", true),
        ("y", true),
        ("z", true),
    ]);

    let first_char = &str[0..1];
    let is_consonant = map.get(first_char).expect("invalid alphabet");
    if *is_consonant {
        format!("{}-{}ay", &str[1..str.len()], first_char)
    } else {
        format!("{}-hay", str)
    }
}

// 해쉬맵과 벡터를 이용하여 회사 내 부서 피고용인 이름을 관리하는 텍스트 인터페이스(CLI)를 작성하라.
pub fn quiz_3() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Add, Read, Remove 중 하나를 입력하세요.");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let commands = input.split_whitespace().collect::<Vec<&str>>();

        let command_type = *commands.get(0).expect("invalid command type");

        if command_type.eq("Add") {
            let name = *commands.get(1).expect("invalid name");
            let department = *commands.get(3).expect("invalid department");

            let employees = map.entry(department.to_string()).or_insert(Vec::new());
            employees.push(name.to_string());
        } else if command_type.eq("Read") {
            let is_sorted = *commands.get(1).map(|&x| x.eq("Sort")).get_or_insert(false);
            for (key, value) in &map {
                let list: Vec<String> = if is_sorted {
                    let mut sorted = value.clone();
                    sorted.sort();
                    sorted
                } else {
                    value.clone()
                };
                println!("{}: {:?}", key, list);
            }
            println!("{}", is_sorted);
        } else if command_type.eq("Remove") {
            let name = *commands.get(1).expect("invalid name");
            let department = *commands.get(3).expect("invalid department");

            let employees = map.entry(department.to_string()).or_insert(Vec::new());
            let index = employees
                .iter()
                .position(|x| x.eq(&name))
                .expect("invalid name");
            employees.remove(index);
        } else {
            println!("This input is invalid!");
            println!("So Re-try!");
        }
    }
}
