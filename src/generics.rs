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

// Struct Definitions

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    pub fn mixup<X1, Y1>(self, other: Point<X1, Y1>) -> Point<T, Y1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64, f64> {
    pub fn dot(&self, other: &Point<f64, f64>) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

#[derive(Debug)]
enum Maybe<T> {
    Just(T),
    None,
}

fn struct_generics() {
    println!("Defining structs with generic types.");
    let origin = Point { x: 0.0, y: 0.0 };
    println!("origin @ {},{}", origin.x, origin.y);

    println!("Method implementations");

    let p = Point { x: 1.0, y: 2.0 };

    dbg!(p.x());
    dbg!(p.y());

    let other = Point { x: -1.58, y: 22.2 };
    dbg!(p.dot(&other));

    println!("Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.");

    let p0 = dbg!(Point { x: 2.0, y: 3.0 });

    let p1 = dbg!(Point {
        x: String::from("Hello"),
        y: String::from("world"),
    });

    dbg!(p0.mixup(p1));
}

fn enum_generics() {
    println!("Defining enums with generic types.");
    let result = Maybe::Just(3);
    let other: Maybe<String> = Maybe::None;
    println!("result: {result:?}");
    println!("other: {other:?}");
}

fn generic_functions() {
    println!("[generic_functions] A generic function to compute the largest element in an array of type T: PartialOrder.");
    let vec = vec![12, 2, 92, 23, 102, -1, 2032, 1, 293, 2013, 2, 231, 2, 23];
    let max = largest(&vec).unwrap_or_else(|error| {
        panic!("{error}");
    });
    println!("The max of {vec:?} is {max}");
}

#[derive(Debug)]
enum Optioni32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Optionf64 {
    Some(f64),
    None,
}

fn monomorphization() {
    println!("[monomorphization] Rust replaces all generic methods and types with concrete types during compile time.");
    let integer: Optioni32 = dbg!(Optioni32::Some(2));
    let float: Optionf64 = dbg!(Optionf64::Some(123123.123));
    println!("integer={:?} float={:?}", integer, float);

    match integer {
        Optioni32::Some(x) => println!("Match integer: {x}"),
        Optioni32::None => println!("Match: NONE"),
    };

    match float {
        Optionf64::Some(x) => println!("Match float {x}"),
        Optionf64::None => println!("Match NONE"),
    }

    let n1 = Optioni32::None;
    let n2 = Optionf64::None;

    match n1 {
        Optioni32::Some(x) => println!("Match integer: {x}"),
        Optioni32::None => println!("Match: NONE"),
    };

    match n2 {
        Optionf64::Some(x) => println!("Match float {x}"),
        Optionf64::None => println!("Match NONE"),
    }
}

pub fn test() {
    println!("[generics::test] Loading generic test...");
    println!("[generics::test] Tests running...");

    generic_functions();
    struct_generics();
    enum_generics();
    monomorphization();
}
