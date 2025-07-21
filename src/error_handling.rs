use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn error_handling_examples() {
    let file = File::open("hello.txt");

    let _file = match file {
        Ok(f) => f,

        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Error creating file: {e:?}"),
            },
            _ => {
                panic!("Problem opening file: {e:?}");
            }
        },
    };
}

pub fn error_handling_examples_clean() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem opening file: {error:?}");
            })
        } else {
            panic!("Problem opening file: {error:?}");
        }
    });
}

pub fn read_username_from_file(filename: &String) -> Result<String, io::Error> {
    let username_file_result = File::open(filename);

    let mut username_file = match username_file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_from_file_shorter(filename: &String) -> Result<String, io::Error> {
    let mut username_file = File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn read_username_from_file_even_shorter(filename: &String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn test() {
    let username = read_username_from_file_shorter(&String::from("username.txt"));
    match username {
        Ok(un) => println!("Username: {un}"),
        Err(e) => panic!("There was an error reaading the username: {e:?}"),
    }
    let username = read_username_from_file_even_shorter(&String::from("username.txt"));
    match username {
        Ok(un) => println!("Username: {un}"),
        Err(e) => panic!("There was an error reaading the username: {e:?}"),
    }
}
