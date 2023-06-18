// 10 - 함수를 추출하여 중복 없애기
// 리스트에서 가장 큰 수 찾기
pub fn find_biggest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
// 두 개의 다른 숫자 리스트에서 가장 큰 수 찾기
pub fn find_biggest_addition_requirement() {
    // 중복된 코드를 생성하는 일은 지루하고 에러가 발생할 가능성도 커짐
    // 로직을 바꾸고 싶을 때 수정해야 할 부분이 여러 군데임을 기억해야 한다는 의미
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
pub fn find_biggest_remove_overlap() {
    // 여기까지 거쳐온 과정
    // 1. 중복된 코드 식별
    // 2. 중복된 코드를 함수의 본문으로 분리하고, 함수의 시그니처 내에 해당 코드의 입력값 및 반환 값을 명시
    // 3. 중복됐었던 두 지점의 코드를 함수 호출로 변경
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
