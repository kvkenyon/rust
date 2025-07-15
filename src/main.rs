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
}

fn main() {
    vectors::test();
    hashy::hashy::test();
}
