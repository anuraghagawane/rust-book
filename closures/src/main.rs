use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| {self.most_stocked()})
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            } 
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref1 = None;
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);


    let expensive_closure = |num: u32| -> u32 {
        println!("Tortoising...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("expensive_closure: {}", expensive_closure(10));

    // the type must be known
    let add_one = |x| x + 1;
    // let add_one = |x| {x + 1}; => also works

    let _n = add_one(10);

    let return_same = |x| x;

    // when first call this the type i32 is bound to the return_same function signature
    return_same(10);
    // as the i32 type is bound to return_same function signature below line will give a
    // compilation error
    // return_same(10.1);

    let list = vec![1, 2, 3];

    let only_borrows = || println!("fc: {list:?}");

    println!("Before: {list:?}");
    only_borrows();
    println!("after: {list:?}");

    let mut list = vec![1, 2, 3];

    // here mutable_borrow while have mut reference of list
    let mut mutable_borrow = || list.push(4);

    // this print will not compile cuz println macro expect a reference but we already have a
    // mutable_borrow for list
    // println!("Before: {list:?}");
    mutable_borrow();
    println!("after: {list:?}");

    let handle = thread::spawn(move || println!("Thread: {list:?}"));

    // if we are joining the thread we don't need to move the value in this example
    // handle.join().unwrap();

    //fnMut
    
    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 11, height: 6},
        Rectangle {width: 1, height: 8},
    ];

    // sort_by_key takes the closures having the fnMut trait
    // i.e. the value cannot be moved but can be used multiple times 
    // on other side in fnOnce the value can be moved but the closure can be called only once.
    // in fn, value is neither moved nor mutated so it can be called any number of times

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");
    list.sort_by_key(|r| {
        // sort_operations.push(value); => sort_by_key expects closure to implement fnMut
        r.width
    });

    println!("{list:#?}");
}

