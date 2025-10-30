use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // unrecoverable alerts
    // panic!("paniced here")
    
    // let a = vec![1, 2, 3];

    //a[50];


    // recoverable errors
    
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //    Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         }
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     }
    // };
    
    // let greeting_file = greeting_file_result.unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|e| {
    //             panic!("Problem creating the file: {e:?}");
    //         })
    //     }
    //     else {
    //         panic!("Problem opening the file {error:?}");
    //     }
    // });

    // let greeting_file_result = File::open("hello.txt").unwrap();
    // let greeting_file_result = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    let _ = read_username_from_file();

    let greeting_file = File::open("hello.txt")?;

    last_char_of_first_line("");

    Ok(())
}

// verbose implementation of the error propogation
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// using the ? operator instead of match
// fn read_username_from_file () -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// one liner for error propogation
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//
//     Ok(username)
// }

// even shorted using in-built function
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
