use std::thread;

pub fn test() {
    println!("[CLOSURES] Start");

    let add_one = |x: i32| -> i32 { x + 1 };
    let add_one_v1 = |x| x + 1;

    dbg!(add_one(2));
    dbg!(add_one_v1(3));

    let toilet = |_| ();
    let s1 = String::from("Going down the toilet bowl.");
    toilet(s1);

    //dbg!(s1); COMPILE ERROR (s1 is moved and dropped)

    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list = vec![1, 2, 3, 4];

    println!("Before defining closure: {list:?}");

    let mut mut_borrow = || list.push(5);

    mut_borrow();

    println!("After running closure: {list:?}");

    let list = vec![1, 2, 3];

    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    println!("[CLOSURES] End");
}
