#[derive(Debug)]
enum SpreadSheet {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{v:?}");

    let mut v = vec![1, 2, 3];
    println!("{v:?}");

    v.push(4);
    println!("{v:?}");

    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(20);

    // this will not working as pushing into the vector can make the memory to reallocate on heap
    // and the above third pointer might then be pointing to sometime random or null
    // v.push(6);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("there is not third value"),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }


    let rows = vec![
        SpreadSheet::Int(5),
        SpreadSheet::Float(5.6),
        SpreadSheet::Text(String::from("blue")),
    ];

    for row in &rows {
        match row {
            SpreadSheet::Int(row) => println!("{row:?}"),
            SpreadSheet::Float(row) => println!("{row:?}"),
            SpreadSheet::Text(row) => println!("{row:?}"),
        }
    }

   
}// here vector goes out of scope and is freed
