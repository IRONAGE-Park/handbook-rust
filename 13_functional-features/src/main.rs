use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShiftColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShiftColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShiftColor>) -> ShiftColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShiftColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for shirt in &self.shirts {
            match shirt {
                ShiftColor::Red => red_count += 1,
                ShiftColor::Blue => blue_count += 1,
            }
        }

        if red_count > blue_count {
            ShiftColor::Red
        } else {
            ShiftColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShiftColor::Red, ShiftColor::Blue, ShiftColor::Red],
    };

    let user_pref1 = Some(ShiftColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list); // error
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let mut num_sort_operations = 0;
    let value = String::from("by key called");
    list.sort_by_key(|r| {
        sort_operations.push(value.clone());
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
