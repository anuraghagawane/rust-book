#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        !(self.x == other.x && self.y == other.y)
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.x > other.x && self.y > other.y
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

struct Guess {
    value: i32
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 {
            panic!("value should be greater than 0")
        }
        else if value > 100 {
            panic!("value should be lesser than 101")
        }
        Guess{ value }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn another() {
        // panic!("Should panic here");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let rect1 = Rectangle {
            x: 5,
            y: 10
        };

        let rect2 = Rectangle {
            x: 4,
            y: 9
        };

        let result = rect1.can_hold(&rect2);

        assert!(result);
    }

    #[test]
    fn smaller_can_hold_larger() {
        let rect1 = Rectangle {
            x: 5,
            y: 10
        };

        let rect2 = Rectangle {
            x: 4,
            y: 9
        };

        let result = rect2.can_hold(&rect1);

        assert!(!result);
    }

    #[test]
    fn it_add_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn check_rect_equal() {
        let rect1 = Rectangle {
            x: 5,
            y: 10
        };

        let rect2 = Rectangle {
            x: 5,
            y: 10
        };

        assert_eq!(rect1, rect2);
    }

    #[test]
    fn check_rect_not_equal() {
        let rect1 = Rectangle {
            x: 5,
            y: 10
        };

        let rect2 = Rectangle {
            x: 5,
            y: 11
        };

        assert_ne!(rect1, rect2);
    }

    #[test]
    fn greeting_with_name() {
        let result = greeting("Anurag");
        assert!(
            result.contains("Anurag"),
            "Greeting contains no name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic(expected = "lesser than 101")]
    fn guess_larger() {
        let _guess = Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add_two(2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("Should be four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 5);
    }
}
