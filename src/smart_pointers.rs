use std::fmt;
use std::ops::Deref;

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

use crate::smart_pointers::BinaryTree::{Empty, NonEmpty};
use crate::smart_pointers::List::{Cons, Nil};

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

    println!("[SMART_POINTERS] End...");
}
