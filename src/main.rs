use crate::generics::largest;

pub mod dicts;
pub mod error_handling;
pub mod generics;
pub mod vectors;

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    vectors::test();
    dicts::test();

    let mut v = vec![3, 2, 1];

    let med = vectors::median(&mut v);

    println!("median={med}");

    let mut v = vec![1, 2, 3, 4];
    let med = vectors::median(&mut v);
    println!("expected=2.5 actual={med}");

    test(&mut v);
    let username = error_handling::read_username_from_file_shorter(&String::from("username.txt"));
    match username {
        Ok(un) => println!("Username: {un}"),
        Err(e) => panic!("There was an error reaading the username: {e:?}"),
    }
    let username =
        error_handling::read_username_from_file_even_shorter(&String::from("username.txt"));
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
