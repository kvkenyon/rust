use std::fmt::Display;

fn dangling_refs() {
    println!("r has lifetime 'a: ----------+--");
    let r;
    {
        println!("x has lifetime 'b: --+--");
        let x = 123;
        r = &x;
        println!("r={r}");
    }
    // x does not live long enough
    //println!("r={r}");
}

fn no_dangling_refs() {
    println!("x has lifetime 'b");
    let x = 5;
    println!("r has lifetime 'a");
    let r = &x;
    println!("r={r}");
}

fn generic_lifetimes_in_functions() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let string1 = String::from("the longests string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    println!("Lifetimes should refer too the smaller lifetime of the pair.");

    // COMPILE ERROR
    let string1 = String::from("Longgggggggggggggggggggggg string");
    let result;
    {
        let string2 = String::from("shorttter");
        result = longest(string1.as_str(), string2.as_str());
        println!("{result}");
    }
    //println!("The longest string is {result}");
}

/*
* DOES NOT COMPILE BECAUSE WE HAVE NO EXPLICIT LIFETIME SPECIFIER
fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
*/

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn lifetime_annotation_syntax<'a>() {
    println!("Examples of annotation syntax for lifetimes.");
    let _reference = &5;
    let _ref_with_lifetime: &'a i32 = &5;
    let _mut_ref_with_lifetime: &'a mut i32;
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn lifetime_annotations_in_structs<'a>() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{i:?}");
    println!("i.part = {}", i.part);
}

fn lifetime_elision_rules() {
    println!("Sometimes you don't need to use lifetimes if the pattern used is in the lifetime elision rules.");
    println!("[RULE 1] each input type gets a different lifetime parameter.");
    println!(
        "[RULE 2] if there is only one input lifetime, each output gets the same lifetime as input."
    );
    println!("[RULE 3] Given multiple input lifetime params, if a parameter is &self or &mut self, all output get the same lifetime as self.");

    let part = String::from("Important part!");
    let announcement = String::from("NEW STUFF!");
    let ie = ImportantExcerpt {
        part: part.as_str(),
    };
    ie.announce_and_return_part(&announcement);
}

fn static_lifetime() {
    let s: &'static str = "Hello, World!";
    println!("{s}");
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test() {
    println!("[lifetimes] Loading lifetime tests...");

    dangling_refs();

    println!("The borrow checker ensures data outlives its references.");
    println!("The borrow checker compare scopes to determine whether all borrows are valid.");

    no_dangling_refs();

    lifetime_annotation_syntax();

    generic_lifetimes_in_functions();

    lifetime_annotations_in_structs();

    lifetime_elision_rules();

    static_lifetime();

    let x = String::from("What it do?!");
    let y = String::from("WHat it do?");

    let ann = String::from("Breaking news!!!");
    let l = longest_with_announcement(&x, &y, &ann);
    println!("The longest is {l}");
    println!("[lifetimes] Lifetime tests completed...");
}
