use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

enum List{
    Con(i32, Rc<List>),
    Nil
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop (&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

#[derive(Debug)]
enum List1 {
    Cons(Rc<RefCell<i32>>, Rc<List1>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons(i32, RefCell<Rc<List2>>),
    Nil,
}

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            Self::Cons(_, item) => Some(item),
            Self::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // let list = List::Con(1, Box::new(List::Con(2, Box::new(List::Con(3, Box::new(List::Nil))))));

    let x = 5;
    // let y = &x;
    let y = MyBox::new(x);

    println!("{} {}", x, *y);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let mut name = Box::new(String::from("abc"));

    // the name will be converted to *name then to &(*name)([..])
    hello(&name);

    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer creater.");

    drop(c);
    println!("CustomSmartPointer before end");


    let a = Rc::new(List::Con(1, Rc::new(List::Con(2, Rc::new(List::Con(3, Rc::new(List::Nil)))))));
    println!("strong_count: {}", Rc::strong_count(&a));

    let b = List::Con(3, Rc::clone(&a));
    println!("strong_count: {}", Rc::strong_count(&a));

    {
        let c = List::Con(4, Rc::clone(&a));
        println!("strong_count: {}", Rc::strong_count(&a));
    }
    println!("strong_count: {}", Rc::strong_count(&a));


    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(List1::Cons(Rc::clone(&value), Rc::new(List1::Nil)));

    let b = List1::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List1::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");


    let a = Rc::new(List2::Cons(5, RefCell::new(Rc::new(List2::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List2::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // will fall in cycle
    // println!("a next item {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn hello(name: &str) {
    println!("{name}");
}
