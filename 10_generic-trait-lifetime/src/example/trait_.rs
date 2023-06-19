// 10.2 - 트레잇 정의하기
// 다양한 종류, 분략ㄷ의 텍스트를 갖는 여러 가지 구조체
// `NewsArticle` 구조체는 특정 지역에서 등록된 뉴스 기사 저장
// `Tweet` 구조체는 최대 280자의 콘텐츠와 해당 트윗이 새 트윗인지, 리트윗인지, 다른 트윗의 대답인지를 나타내는 메타 데이터를 저장
// 두 구조체 인스턴스에 저장된 데이터를 종합해 보여주는 종합 미디어 라이브러리 크레이트 `aggregator`를 만든다고 가정
pub trait Summary {
    fn summarize(&self) -> String;
    // 메서드 시그니처 뒤에 구현을 넣는 대신 세미콜론을 집어넣음
    // 이 트레잇을 구현하는 각 타입이 이 메서드에 맞는 동작을 직접 제공해야 함
    // 컴파일러는 `Summary` 트레잇이 있는 모든 타입에 정확히 이와 같은 시그니처 `summarize` 메서드를 가지고 있도록 강제할 것
}

// 10.2 - 특정 타입에 트레잇 구현하기
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
pub fn test_main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// 10.2 - 기본 구현
pub struct NewsArticle2 {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
impl Summary2 for NewsArticle2 {} // 기본 구현을 사용하기 위해 비어있는 블록을 명시
                                  // 기본 구현을 생성한다고 해서 이미 구현되어 있는 구조체의 구현을 변경할 필요는 없음
                                  // 기본 구현을 오버라이딩 하는 문법과 기본 구현이 없는 트레잇 메서드를 구현하는 문법은 동일하기 때문
pub fn test_main_2() {
    let article = NewsArticle2 {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
// 기본 구현 안쪽에서 트레잇의 다른 메서드를 호출할 수도 있음
pub trait Summary3 {
    // 호출할 다른 메서드가 기본 구현을 제공하지 않는 메서드여도 상관없음
    // 이런 방식으로 트레잇은 작은 부분만 구현을 요구하면서 유용한 기능을 많이 제공
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct Tweet3 {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary3 for Tweet3 {
    // 구현하기 위해서는 이 메서드만 정의하면 됨
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 10.2 - 매개변수로서의 트레잇
// `Summary` 트레잇을 사용하여 `summarize` 메서드를 호출하는 `notify` 함수 정의
pub fn notify(item: &impl Summary3) {
    // 구체적인 타입을 명시하는 대신 `impl` 키워드와 트레잇의 이름을 명시
    // 이 매개변수에는 지정된 트레잇을 구현하는 타입이라면 어떤 타입이든 전달받을 수 있음
    // 만약 `Summary` 트레잇을 구현하지 않은 타입으로 함수를 호출한다면 컴파일 에러 발생
    println!("Breaking news! {}", item.summarize());
}

// 10.2 - 트레잇을 구현하는 타입을 반환하기
pub fn returns_summarizable() -> impl Summary3 {
    // 반환 타입에 구체적 타입명이 아닌 `impl Summary`를 작성하여 트레잇을 구현하는 타입을 반환한다고 명시
    // 이 함수를 호출하는 쪽의 코드에서는 구체적인 타입을 알 필요가 없음
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// `impl trait` 문법을 쓴다고 해서 다양한 타입을 반환할 수는 없음
// `impl trait` 문법이 컴파일러 내에 구현된 방식으로 인한 제약 때문에 허용되지 않음
pub fn returns_summarizable_error() -> impl Summary3 {
    // if switch {
    //     NewsArticle {
    //         headline: String::from("Penguins win the Stanley Cup Championship!"),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         ),
    //     }
    // } else {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
}

// 10.2 - 트레잇 바운드를 사용해 조건부로 메서드 구현하기
// `Pair<T>` 타입은 언제나 새로운 인스턴스를 반환하는 `new` 함수를 구현함
use std::fmt::Display;
pub struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        // `new` 함수는 제네릭 타입 `T`를 사용하여 `Pair<T>` 구조체의 인스턴스를 생성
        Self { x, y }
    }
}
// 비교 가능하게 해주는 `PartialOrd` 트레잇과 출력을 가능하게 만드는 `Display` 트레잇을 모두 구현한 타입인 경우에 대해서만 `cmp_display` 메서드를 구현하고 있음
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            printf!("The largest member is x = {}", self.x);
        } else {
            printf!("The largest member is y = {}", self.y);
        }
    }
}
