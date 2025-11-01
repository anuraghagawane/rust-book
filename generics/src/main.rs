struct Point1<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T1, U1> Point2<T1, U1> {
    fn mixup<T2, U2> (self, other: Point2<T2, U2>) -> Point2<T1, U2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let v = vec![1, 4, 5, 2];
    let result = largest(&v);
    println!("Largest number is: {result}");

    let v = vec!['z', 'd', 'a', 'b'];
    let result = largest(&v);
    println!("Largest number is: {result}");

    let point1 = Point1{x: 10, y: 10};
    let point1 = Point1{x: 10.0, y: 10.1};
    let point1 = Point2{x: 10, y: 10.1};


    let point = Point{x: 10, y: 10};
    println!("p.x = {}", point.x());
    // this will not give distance_from_origin as we have declared distance for only f32
    // println!("distance_from_origin: {}", point.distance_from_origin());

    let point = Point{x: 10.0, y: 10.1};
    println!("distance_from_origin: {}", point.distance_from_origin());

    let p1 = Point2{x: 5, y: 10.3};
    let p2 = Point2{x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3. y);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
