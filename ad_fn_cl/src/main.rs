fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32>  {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

#[macro_export]
macro_rules! anurag {
    ( $( $x: expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    println!("do_twice: {}", do_twice(add_one, 5));

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = 
        list_of_numbers.iter().map(|x| x.to_string()).collect();

    println!("list_of_strings: {:?}", list_of_strings);

    let list_of_strings: Vec<String> = 
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("list_of_strings: {:?}", list_of_strings);

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{}", returns_closure()(2));

    let handlers = vec![returns_closure(), returns_initialized_closure(12)];
    for handler in handlers {
        println!("output: {:?}", handler(5));
    }

    let a = anurag![1, 2, 3];
    println!("{:?}", a);
}
