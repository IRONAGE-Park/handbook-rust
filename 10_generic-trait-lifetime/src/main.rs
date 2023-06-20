use crate::example::generic;
use crate::example::lifetime;
use crate::example::overview;
use crate::example::trait_;

// 10.2 - 예제 10-13
use crate::example::trait_::{Summary, Tweet}; // main에서 사용하기 위해서는 이렇게 둘 다 가져와야 함

fn main() {
    // overview::find_biggest();
    // overview::find_biggest_addition_requirement();
    // overview::find_biggest_remove_overlap();

    // generic::test_main_1();
    // generic::test_main_2();
    // generic::test_main_3();

    // trait_::test_main();
    // trait_::test_main_2();

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from("of course, as you probably already know, people"),
    //     reply: false,
    //     retweet: false,
    // };
    //
    // println!("1 new tweet: {}", tweet.summarize());

    // lifetime::test_main_1();
    // lifetime::test_main_2();
    // lifetime::test_main_3();
    // lifetime::test_main_4();
    // lifetime::test_main_5();
    // lifetime::test_other();
}

mod example;
