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
}
