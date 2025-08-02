use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn traverse(&self) {
        match self {
            Cons(x, rest) => {
                println!("{x}");
                rest.traverse()
            }
            Nil => {
                println!("NIL");
            }
        }
    }
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[derive(Debug)]
enum BinaryTree<T> {
    NonEmpty(Box<TreeNode<T>>),
    Empty,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            NonEmpty(ref mut tree_node) => {
                if value <= tree_node.element {
                    tree_node.left.add(value);
                } else {
                    tree_node.right.add(value);
                }
            }
            Empty => {
                *self = NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: Empty,
                    right: Empty,
                }))
            }
        }
    }
}

impl<T: Ord + fmt::Display> fmt::Display for BinaryTree<T> {
    fn fmt(&self, dest: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            NonEmpty(ref node) => {
                writeln!(dest, " {} ", node.element)?;
                node.left.fmt(dest)?;
                node.right.fmt(dest)
            }
            Empty => {
                write!(dest, " NIL")
            }
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

use crate::smart_pointers::BinaryTree::{Empty, NonEmpty};
use crate::smart_pointers::List::{Cons, Nil};
use crate::smart_pointers::List2::{Cons as Cons2, Nil as Nil2};

pub fn test() {
    println!("[SMART_POINTERS] Start...");

    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
    );

    list.traverse();

    let btree = NonEmpty(Box::new(TreeNode {
        element: 0,
        left: NonEmpty(Box::new(TreeNode {
            element: 1,
            left: Empty,
            right: Empty,
        })),
        right: NonEmpty(Box::new(TreeNode {
            element: 2,
            left: Empty,
            right: Empty,
        })),
    }));

    println!("binary tree = {:?}", btree);

    let mut root = NonEmpty(Box::new(TreeNode {
        element: 12,
        left: Empty,
        right: Empty,
    }));

    root.add(1);
    root.add(2);
    root.add(100);
    root.add(12);
    root.add(124);
    root.add(14);
    root.add(-2);

    println!("tree: {:?}", root);
    println!("{}", root);

    println!("Deref:");
    let x = 5;
    let y = &x;
    println!("x = {x}");
    println!("*y = {}", *y);

    let x = 5;
    let y = Box::new(x);
    println!("x = {x}");
    println!("*Box[x] = {}", *y);

    let my_box = MyBox::new(123);
    println!("{}", *my_box);

    println!("deref coercion:");
    let my_box_string = MyBox::new(String::from("Yalla!!"));
    hello(&my_box_string);

    reference_counting();
    println!("[SMART_POINTERS] End...");
}

fn hello(s: &str) {
    println!("Hello, {s}!");
}

fn reference_counting() {
    println!("[REF_COUNTING] start..");
    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("Count after initialization: {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("Count after creating b: {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("Count after creating c: {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));

    println!("[REF_COUNTING] end..");
}

pub trait Messenger {
    fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    value: usize,
    max: usize,
    messenger: &'a T,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        LimitTracker {
            value: 0,
            max,
            messenger,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage = self.value as f64 / self.max as f64;
        if percentage >= 1.0 {
            self.messenger
                .send("WARNING! YOU'VE EXCEEDED MAX CAPACTIY!");
        } else if percentage >= 0.9 {
            self.messenger
                .send("You've used more than 90% of your capacity.");
        } else if percentage >= 0.75 {
            self.messenger
                .send("You've used more than 75% of your capacity.");
        } else if percentage >= 0.25 {
            self.messenger
                .send("You've used more than 25% of your capacity.");
        } else {
            self.messenger.send(&format!(
                "You've used more than {:?}% of your capacity.",
                percentage * 100.0
            ));
        }
    }
}

pub struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    pub fn new() -> Self {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

#[cfg(test)]
mod tests {
    use crate::smart_pointers::List3::{Cons3 as C, Nil3 as N};
    use crate::smart_pointers::{LimitTracker, MockMessenger};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_limit_tracker_70_plus() {
        let messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&messenger, 100);

        limit_tracker.set_value(72);

        assert_eq!(limit_tracker.messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn test_list3() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(C(Rc::clone(&value), Rc::new(N)));
        let b = C(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = C(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {a:?}");
        println!("b after = {b:?}");
        println!("c after = {c:?}");

        assert_eq!(*value.borrow(), 15);
    }
}
