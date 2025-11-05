fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // iterator takes the unmutable reference
    // let v1_iter = v1.into_iter(); // iterator takes the ownership
    // let v1_iter = v1.iter_mut(); // iterator takes the mutable reference

    for val in v1_iter {
        println!("{val}");
    }

    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    println!("{v2:?}");
}
