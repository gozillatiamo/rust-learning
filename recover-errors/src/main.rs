use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // greeting_file_3();
    println!("username is: {:?}", read_username_from_file());
    println!(
        "last char of first line is: {:?}",
        last_char_of_first_line("Hello World")
    );
    Ok(())
}

fn greeting_file_1() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the ile: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

fn greeting_file_2() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opeing the file: {error:?}");
        }
    });
}

fn greeting_file_3() {
    // If the file does not exist, then panic with error message.
    // let greeting_file = File::open("hello.txt").unwrap();

    // If the file does not exist, then panic with custom error message.
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // using File and ? operatior
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
