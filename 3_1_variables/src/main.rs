fn shadowing() {
    // 이전 변수와 동일한 이름으로 새 변수를 선언할 수 있으며, 같은 스코프 내에서 적용 가능함
    let x = 5;
    
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the outer scope is: {x}");

    let spaces = "   ";
    let mut mut_spaces = spaces;
    println!("The value of spaces is: {spaces}");
    // 섀도잉은 mut 키워드를 사용한 것과는 다르게, 다른 타입을 적용할 수도 있음
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    // mut_spaces = mut_spaces.len(); // 에러 발생
}

fn main() {
    let mut x = 5; // mut 키워드는 변수의 변경 가능성을 나타냄
    println!("The value of x is: {x}");
    // x = 6; 기본적으로 변수는 변경할 수 없음
    x = 6; // 그러나 mut 키워드를 사용하면 변경할 수 있음
    println!("The value of x is: {x}");
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // const 키워드로 선언된 상수는 항상 변경할 수 없음
    // 런타임에만 계산 가능한 변수 값으로는 상수를 설정할 수 없고, 반드시 상수 식으로만 상수를 설정할 수 있음
    
    shadowing();
}
