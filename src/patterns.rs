pub fn test() {
    println!("[PATTERNS] Start...");
    matching_literals();
    matching_named_vars();
    multiple_patterns();
    matching_range_of_values();
    destruct_structs();
    destruct_enums();
    destruct_nested_structs_and_enums();
    destruct_structs_and_tuples();
    ignoring_values();
    println!("[PATTERNS] End...");
}

fn matching_literals() {
    let x = 5;
    match x {
        1 => println!("ONE"),
        5 => println!("FIVE"),
        _ => println!("OTHER"),
    }
}

fn matching_named_vars() {
    let x = Some(10);
    let y = 5;

    match x {
        Some(50) => println!("50!"),
        // shadows the y in the match scope
        Some(y) => {
            println!("matched y = {y}");
            assert_eq!(10, y);
        }
        _ => println!("default x = {x:?}"),
    }

    println!("after match scope: x = {x:?} y = {y}");
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}

fn matching_range_of_values() {
    let x = 5;
    match x {
        1..=5 => println!("matched x = {x} which is in 1..=5"),
        _ => println!("OTHER"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("a to j: x = {x}"),
        'k'..='z' => println!("k to z: x = {x}"),
        _ => println!("Non-ascii"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destruct_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    println!("Point(x: {x}, y: {y})");
    let Point { x: a, y: b } = p;
    println!("a={a} b={b}");

    match p {
        Point { x, y: 0 } => println!("on the x axis x={x}"),
        Point { x: 0, y } => println!("on the y axis y={y}"),
        Point { x, y } => println!("on neither axis @ ({x}, {y})"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destruct_enums() {
    let msg = Message::ChangeColor(1, 2, 3);

    let msgs = vec![
        Message::Quit,
        Message::Move { x: 1, y: -20 },
        Message::Write(String::from("Hello, World!")),
        msg,
    ];

    for msg in msgs {
        match msg {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Write text={text}"),
            Message::ChangeColor(x, y, z) => println!("ChangeColor to ({x},{y},{z})"),
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destruct_nested_structs_and_enums() {
    let msg = Command::ChangeColor(Color::Rgb(1, 2, 3));

    let msgs = vec![
        Command::Quit,
        Command::Move { x: 1, y: -20 },
        Command::Write(String::from("Hello, World!")),
        msg,
        Command::ChangeColor(Color::Hsv(12, 12, 123)),
    ];

    for msg in msgs {
        match msg {
            Command::Quit => println!("quit"),
            Command::Move { x, y } => println!("Move to ({x}, {y})"),
            Command::Write(text) => println!("Write text={text}"),
            Command::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("ChangeColor to rgb({r},{g},{b})")
            }
            Command::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("ChangeColor to hsv({h}, {s}, {v})")
            }
        }
    }
}

fn destruct_structs_and_tuples() {
    println!("Hello world");
}

fn ignoring_values() {}
