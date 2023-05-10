// 직사각형의 면적을 계산하는 프로그램

#[derive(Debug)] // `Debug` 트레이트를 구현하기 위해 필요
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area_by_struct(&rect)
    );
    
    // 구조체를 디버깅하기 쉽게 `println!` 매크로를 사용하여 출력할 수 있다면 유용할 것
    // println!("rect is {}", rect);  // 하지만 이 방법은 `Display` 트레이트를 구현하지 않기 때문에 출력할 수 없음
    
    // `:?` 플래그를 사용하여 `println!` 매크로를 사용할 수 있음
    // println!("rect is {:?}", rect); // `Debug`를 구현하고 있지 않기 때문에 마찬가지로 사용할 수 없음.

    println!("rect is {:?}", rect);
    println!("rect is {:#?}", rect); // `:#?` 플래그를 사용하면 좀 더 보기 좋게 출력할 수 있음
    
    // `dbg!` 매크로를 사용하여 값을 출력할 수도 있음
    // 이 매크로는 표현식의 소유권을 갖고, 매크로 호출이 발생하는 파일 및 줄 번호를 함께 인쇄함.
    // ** `dbg!` 매크로는 표준 출력 스트림(`stdout`)에 인쇄되지 않고 표준 오류 콘솔 스트림(`stderr`)에 인쇄됨. **
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // 30 * scale = 60
        height: 50
    };
    
    dbg!(rect2);
    // `dbg!` 매크로를 사용하면 표현식에 영향을 주지 않으며, 그 덕에 디버깅에 용이함.
}

// 기존 코드
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 튜플로 리팩터링
fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 구조체로 리팩터링
fn area_by_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}