use std::fs::File;
use std::io::{self, ErrorKind, Read};

mod hashy;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
mod vectors {
    pub fn test() {
        let row = vec![
            super::SpreadsheetCell::Int(32),
            super::SpreadsheetCell::Float(1.2),
            super::SpreadsheetCell::Text(String::from("Hello, World!")),
        ];

        for col in row.iter() {
            match col {
                super::SpreadsheetCell::Int(x) => println!("an int: {}", x),
                super::SpreadsheetCell::Float(x) => println!("a float: {}", x),
                super::SpreadsheetCell::Text(x) => println!("a string: {}", x),
            }
        }
        let mut v: Vec<i32> = vec![1, 2, 3];
        let mut v2: Vec<&mut i32> = Vec::new();

        for i in &mut v {
            v2.push(i);
        }
        *v2[0] = 5;
        let a = *v2[0];
        let b = v[0];
        println!("{a} {b}");

        let s = "dumb string literal";
        let s0 = s.to_string();

        println!("s={}, s0={}", s, s0);

        let mut s = String::new();

        s.push_str("Push a string");

        println!("mut s = {}", s);

        s.push('.');

        println!("push char {}", s);

        let s1 = "a dumb string".to_string();
        let s2 = "another dumb string".to_string();

        let s3 = s1 + " " + &s2;

        println!("s2={}, s3={}", s2, s3);

        let s1 = "tic".to_string();
        let s2 = "tac".to_string();
        let s3 = "toe".to_string();

        let s = format!("{s1}-{s2}-{s3}");

        println!("s={s}");

        let hello = String::from("Hola");

        println!("len {} = {}", hello, hello.len());

        let hello = String::from("Здравствуйте");
        println!("len {} = {}", hello, hello.len());
    }

    pub fn median(v: &mut Vec<i32>) -> f64 {
        v.sort();
        if v.len() % 2 == 0 {
            let mid = v[v.len() / 2];
            let mid2 = v[(v.len() / 2) - 1];
            return ((mid + mid2) / 2).into();
        }

        v[v.len() / 2].into()
    }
}

fn main() {
    vectors::test();
    hashy::hashy::test();

    let mut v = vec![3, 2, 1];

    let med = vectors::median(&mut v);

    println!("median={med}");

    let mut v = vec![1, 2, 3, 4];
    let med = vectors::median(&mut v);
    println!("expected=2.5 actual={med}");

    test(&mut v);
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

    let vec = vec![12, 2, 92, 23, 102, -1, 2032, 1, 293, 2013, 2, 231, 2, 23];

    let max = largest(&vec).unwrap_or_else(|error| {
        panic!("{error}");
    });
    println!("The max of {vec:?} is {max}");
}

fn test(v: &mut Vec<i32>) {
    let n = v.len();
    for (i, t) in v.iter().enumerate().rev() {
        println!("i={i} t={t}");
    }
    println!("n={n}");
}

fn error_handling_examples() {
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

fn error_handling_examples_clean() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem opening file: {error:?}");
            })
        } else {
            panic!("Problem opening file: {error:?}");
        }
    });
}

fn read_username_from_file(filename: &String) -> Result<String, io::Error> {
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

fn read_username_from_file_shorter(filename: &String) -> Result<String, io::Error> {
    let mut username_file = File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_even_shorter(filename: &String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

fn largest<T: PartialOrd>(v: &[T]) -> Result<&T, &str> {
    if v.len() == 0 {
        return Err("Empty array.");
    }
    let mut max = &v[0];
    for i in 1..v.len() {
        if &v[i] > &max {
            max = &v[i];
        }
    }
    Ok(max)
}
