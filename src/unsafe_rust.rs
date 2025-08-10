use std::slice;
use std::fmt;
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
       Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        } 
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, rhs: Meters) -> Self::Output {
       Millimeters(self.0 + (rhs.0 * 1000))
    }
}

// disambiguating methods with the same name

trait Wizard {
    fn fly(&self);
}

trait Pilot {
    fn fly(&self);
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*wagging arms wildly*");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Magic man in the air.");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

// disambiguating non-method functions

trait Animal {
    fn baby_name();
}

struct Wolfie;

impl Wolfie {
    fn baby_name() {
        println!("Wolfie the Wolfowitz Chalom");
    }
}

impl Animal for Wolfie {
    fn baby_name() {
        println!("Puppy!");
    }
}

// Supertraits

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Orphan Rule workaround using newtype pattern

// wont work b/c trait and struct not def in our crate
//impl fmt::Display for Vec<String> {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       write!(f, "[{}]", self.join(", "))
//    }
//
//}

// create thin newtype wrapper

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("just called a Rust fn from C!!!");
}

static HELLO_WORLD: &str = "Hello, World!";

pub fn test() {
    println!("[UNSAFE] Start...");
    raw_ptrs();

    //unsafe {
    //    println!("Absolute value -3 according to C: {}", abs(-3));
    //}
    println!("Absolute value -3 according to C: {}", abs(-3));

    modify_mut_static_var();
    println!("[UNSAFE] End...");


    println!("[ADVANCED_TRAITS] Start...");
    let a = Point { x: 100, y: 200};
    let b = Point {x: 2, y: 3};
    let c = a + b;

    println!("a + b = {c:?}");


    println!("disambiguating methods...");
    let human = Human;

    human.fly();
    Wizard::fly(&human);
    Pilot::fly(&human);

    println!("disambiguating non-method functions...");
    Wolfie::baby_name();
    //Animal::baby_name(); WONT COMPILE, Rust can't determine which function to call
    // need fully qualified name
    <Wolfie as Animal>::baby_name();

    println!("supertraits...");
    let p = Point {x: 1, y: 200};
    p.outline_print();

    println!("newtype wrapper...");
    let wrapper_vec = Wrapper(vec![String::from("Hello"), String::from("World")]);
    println!("{}", wrapper_vec);

    println!("[ADVANCED_TRAITS] End...");
}

unsafe fn dangerous() {
    println!("DANGEROUS TINGGGGS");
}

fn raw_ptrs() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // compiler error call to unsafe function in safe block
    //dangerous();

    unsafe {
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);
        *r2 = 78;
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);

        dangerous();
    }

    let values = &mut [1, 2, 3, 4, 5, 6];

    let (x, y) = split_at_mut(values, 4);
    println!("x = {:?}", x);
    println!("y = {:?}", y);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let length = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= length);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), length - mid),
        )
    }
}

static mut COUNTER: u32 = 0;

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn modify_mut_static_var() {
    println!("name is: {HELLO_WORLD}");

    unsafe {
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}





