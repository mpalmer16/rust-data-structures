#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T: PartialOrd + Debug> {
    data: T,
    left: Box<Option<Node<T>>>,
    right: Box<Option<Node<T>>>,
}

impl<T: PartialOrd + Debug> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: Box::new(None),
            right: Box::new(None),
        }
    }

    pub fn insert(&mut self, data: T) {
        if self.data < data {
            if let Some(node) = self.right.as_mut() {
                node.insert(data);
            } else {
                self.right = Box::new(Some(Node::new(data)));
            }
        } else if let Some(node) = self.left.as_mut() {
            node.insert(data);
        } else {
            self.left = Box::new(Some(Node::new(data)));
        }
    }

    pub fn print(&self) {
        if let Some(node) = self.left.as_ref() {
            node.print();
        }
        print!("{:?} ", self.data);
        if let Some(node) = self.right.as_ref() {
            node.print();
        }
    }

    pub fn find(&self, data: T) -> Option<T> {
        if self.data == data {
            Some(data)
        } else if self.data < data {
            if let Some(node) = self.right.as_ref() {
                node.find(data)
            } else {
                None
            }
        } else if let Some(node) = self.left.as_ref() {
            node.find(data)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod binary_tree {
    use super::*;

    #[test]
    fn create_new_tree() {
        let btree = Node::new(1);

        assert!(btree.data == 1);
        assert!(btree.left.is_none());
        assert!(btree.right.is_none());
    }

    #[test]
    fn insert_into_tree() {
        let mut btree = Node::new(5);

        btree.insert(1);
        btree.insert(6);
        btree.insert(3);
        btree.insert(2);
        btree.insert(4);

        assert!(btree.data == 5);
        assert!(btree.right.unwrap().data == 6);
        assert!(btree.left.unwrap().right.unwrap().data == 3);
    }

    #[test]
    fn find_in_tree() {
        let mut btree = Node::new(5);

        btree.insert(1);
        btree.insert(6);
        btree.insert(3);
        btree.insert(2);
        btree.insert(4);

        assert!(btree.find(4) == Some(4));
        assert!(btree.find(10).is_none());
    }
}
