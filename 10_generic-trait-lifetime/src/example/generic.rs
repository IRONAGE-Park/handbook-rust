// 10.1 - 제네릭 함수 정의
pub fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
pub fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// 새 단일 함수의 시그니처 내 타입을 매개변수화하려면 타입 매개변수의 이름을 지어주어야 함
// `Rust`에서 타입 이름을 지어줄 때는 대문자로 시작하는 `UpperCamelCase`를 사용하는 것이 관례
// `<>` 꺾쇠 괄호 사이에 타입 매개변수의 이름을 선언해야 함
pub fn largest<T>(list: &[T]) -> &T {
    // `largest` 함수는 어떤 타입 `T`에 대한 제네릭 함수
    let mut largest = &list[0];
    for item in list {
        // if item > largest {
        // 지금 바로 컴파일하면 여기서 에러가 발생함
        // 에러의 도움말에서 언급되는 `std::cmp::PartialOrd`는 우선
        // 이 에러가 `largest`의 본문이 `T`가 될 수 있는 모든 타입에 대해 동작할 수 없음을 뜻한다고 이해하자
        // `std::cmp::PartialOrd` 트레잇을 구현하면 비교가 가능하게 됨
        largest = item;
        // }
    }
    largest
}
pub fn test_main_1() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// 10.1 - 제네릭 구조체 정의
struct Point<T> {
    x: T,
    y: T,
}
pub fn definition_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // let mix = Point { x: 5, y: 4.0 };
    // 위의 예제에서는 `Point<T>`선언에 하나의 제네릭만 사용했기 때문에 `x`, `y` 둘다 같은 타입이라는 뜻
    // 만약 다른 타입을 갖는 `x`, `y`를 선언하기 위해서는 여러 개의 타입 매개변수를 사용해야 함
    struct PointDual<T, U> {
        x: T,
        y: U,
    }
}

// 10.1 - 제네릭 메서드 정의
impl<T> Point<T> {
    // `Point<T>` 타입에 메서드를 구현한다고 명시
    // 이렇게 하면 `Rust`는 `Point`의 꺾쇠 괄호 내 타입이 구체적인 타입이 아닌 제네릭 타입임을 인지
    // 구조체 정의에 선언된 제네릭 매개변수의 이름과 같은 이름을 사용하는 것이 관례
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    // 메서드를 정의할 때 제네릭 타입에 대한 제약을 지정할 수도 있음
    // T가 `f32` 타입이 아닌 `Point<T>` 인스턴스는 이 메서드가 정의되지 않음
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
pub fn test_main_2() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
struct PointOther<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> PointOther<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointOther<X2, Y2>) -> PointOther<X1, Y2> {
        PointOther {
            x: self.x,
            y: other.y,
        }
    }
}
pub fn test_main_3() {
    let p1 = PointOther { x: 5, y: 10.4 };
    let p2 = PointOther { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
