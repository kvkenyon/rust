use std::collections::HashMap;

pub fn test() {
    println!("[dicts]");

    let mut scores = HashMap::new();
    scores.insert("blue", 2);
    scores.insert("red", 3);

    println!(
        "Score: {} {} : {} {}",
        "blue",
        scores.get("blue").copied().unwrap_or(0),
        "red",
        scores.get("red").copied().unwrap_or(0)
    );

    let key = String::from("Yellow");
    let value = String::from("123");

    let mut scores = HashMap::new();
    scores.insert(key, value);

    println!("[dicts] Overwrite key.");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    println!("{scores:?}");
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    println!("[dicts] Insert if not exists.");
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("{scores:?}");

    println!("[dicts] Insert or update previous value.");

    let text = "I am a teaspot short and stout and a moomoo is a cow boy cat";

    let mut counter = HashMap::new();
    for word in text.split_whitespace() {
        let count = counter.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{counter:?}");
}
