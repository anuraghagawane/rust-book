#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //let width1 = 30;
    //let height1 = 50;

    //println!("area is: {}", area(width1, height1));
    
    //let rect1 = (30, 50);
    //println!("area is: {}", area(rect1));
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area is: {}", area(&rect1));
    //println!("{rect1.area()}");
    dbg!(rect1.area());
    dbg!(rect1.can_hold(&rect2));
    dbg!(rect1.can_hold(&rect3));

    let sq = Rectangle::square(4);
    dbg!(sq);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
