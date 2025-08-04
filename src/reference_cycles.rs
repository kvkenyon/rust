use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, rest) => Some(rest),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

use crate::reference_cycles::List::{Cons, Nil};

pub fn test() {
    println!("[reference_cycles] Start...");
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a ref count after create: {}", Rc::strong_count(&a));
    println!("a next item: {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a ref count after create b: {}", Rc::strong_count(&a));
    println!("b initial ref count: {}", Rc::strong_count(&b));
    println!("b next item: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b ref count after changing a: {}", Rc::strong_count(&b));
    println!("a ref count after changing b: {}", Rc::strong_count(&a));

    build_tree();

    println!("[reference_cycles] End...");
}

fn build_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    println!("Ref count leaf: {}", Rc::strong_count(&leaf));
    println!("Ref count branch: {}", Rc::strong_count(&branch));
}
