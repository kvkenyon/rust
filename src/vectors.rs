use crate::spreadsheet_cell;

pub fn test() {
    let row = vec![
        spreadsheet_cell::SpreadsheetCell::Int(32),
        spreadsheet_cell::SpreadsheetCell::Float(1.2),
        spreadsheet_cell::SpreadsheetCell::Text(String::from("Hello, World!")),
    ];

    for col in row.iter() {
        match col {
            spreadsheet_cell::SpreadsheetCell::Int(x) => println!("an int: {}", x),
            spreadsheet_cell::SpreadsheetCell::Float(x) => println!("a float: {}", x),
            spreadsheet_cell::SpreadsheetCell::Text(x) => println!("a string: {}", x),
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

    let mut v = vec![3, 2, 1];

    let med = median(&mut v);

    println!("median={med}");

    let mut v = vec![1, 2, 3, 4];
    let med = median(&mut v);
    println!("expected=2.5 actual={med}");

    local_test(&mut v);
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

fn local_test(v: &mut Vec<i32>) {
    let n = v.len();
    for (i, t) in v.iter().enumerate().rev() {
        println!("i={i} t={t}");
    }
    println!("n={n}");
}
