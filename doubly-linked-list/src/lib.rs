#![allow(dead_code)]

#[derive(Clone, PartialEq)]
struct Node<T: Clone> {
    data: T,
    previous: Box<Option<Node<T>>>,
    next: Box<Option<Node<T>>>,
}

impl <T: Clone> Node<T> {
    fn new(data: T, previous: Option<Node<T>>, next: Option<Node<T>>) -> Self {
        Self {
            data,
            previous: Box::new(previous),
            next: Box::new(next),
        }
    }
}

struct DoublyLinkedList<T: Clone> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
}

impl <T: Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None
        }
    }

    fn push(&mut self, mut data: Node<T>) {
        if self.head.is_none() {
            self.head = Some(data.clone());
            self.tail = Some(data.clone());
        } else if let Some(tail) = self.tail.as_mut() {
            tail.next = Box::new(Some(data.clone()));
            data.previous = Box::new(self.tail.clone());
            self.tail = Some(data.clone());
        }
    }
}

#[cfg(test)]
mod doubly_linked_list {
    use super::*;

    #[test]
    fn push_to_list() {
        let mut dll = DoublyLinkedList::new();

        let new_node = Node::new(1, None, None);
        dll.push(new_node.clone());

        assert!(dll.head.clone().is_some_and(|node| node == new_node));

        let another_node = Node::new(2, None, None);
        dll.push(another_node.clone());

        assert!(dll.tail.clone().is_some_and(|node| node == another_node));

        assert!(dll.tail.clone().is_some_and(
            |node| node.previous.is_some_and(
                |node| node.data == 1
            )
        ));
    }
}
