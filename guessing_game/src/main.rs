use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Rust에서 변수는 기본적으로 변경할 수 없음
    // let apples = 5; // immutable
    // let mut bananas = 6; // mutable
    
    let secret_number = rand::thread_rng().gen_range(1..=100); // 1~100 사이의 난수 생성
    // 1..=100은 범위 표현식
    
    // println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");

        // 사용자 입력을 저장할 변수 생성
        let mut guess = String::new();

        // use std::io를 했기 때문에 아래와 같이 사용 가능
        // 만약 위의 키워드를 입력하지 않았다면 std::io::stdin()과 같이 사용해야 함
        io::stdin()
            // &mut guess는 guess 변수를 가변으로 참조하고 있다는 의미
            .read_line(&mut guess)
            // read_line()은 io::Result 타입을 반환
            // io::Result 타입은 열거형으로 Ok와 Err 두 가지 값을 가짐
            // expect를 호출하지 않으면 컴파일 시 경고가 발생
            .expect("Failed to read line");

        // 기존의 guess 변수는 String 타입이기 때문에 아래의 비교식을 사용할 수 없으므로 이를 변경함.
        // parse()를 사용하기 위해 타입을 명확하게 명시하였음
        // u32 타입은 부호가 없는 32비트 정수 타입, - 부호를 추가하기 위해서는 i32 타입을 사용
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 잘못된 입력이 들어올 경우 다음 루프로 통과시킴
            Err(_) => continue,
        };
        
        // 이미 선언된 변수여도 섀도잉을 통해 중복 선언이 되는 것처럼 이전 변수를 숨길 수 있음

        println!("You guessed: {}", guess);
        // println!("You guessed: {guess}"); // 괄호 안에 변수를 넣어도 무방함

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y = {}", y);
}