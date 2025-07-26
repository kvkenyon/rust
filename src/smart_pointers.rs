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

    println!("[SMART_POINTERS] End...");
}
