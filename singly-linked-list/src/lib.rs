#![allow(dead_code)]

#[derive(Clone, PartialEq)]
struct Node<T: PartialEq> {
    data: T,
    next: Box<Option<Node<T>>>,
}

#[derive(Clone)]
struct LinkedList<T: PartialEq> {
    head: Box<Option<Node<T>>>,
}

impl<T: PartialEq> LinkedList<T> {
    fn empty() -> Self {
        Self {
            head: Box::new(None),
        }
    }

    fn push(&mut self, data: T) {
        let old_head = self.head.take();
        self.head = Box::new(Some(Node {
            data,
            next: Box::new(old_head),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    fn find(&self, data: T) -> bool {
        let mut current = self.head.as_ref();
        while current.is_some() {
            if current.as_ref().is_some_and(|node| node.data == data) {
                return true;
            } else {
                current = &current.as_ref().unwrap().next;
            }
        }
        false
    }
}

#[cfg(test)]
mod linked_list {

    use super::*;

    #[test]
    fn push_to_list() {
        let mut ll = LinkedList::empty();
        ll.push(3);
        ll.push(2);
        ll.push(1);

        assert!(ll.clone().head.is_some_and(|node| node.data == 1));

        assert!(ll
            .clone()
            .head
            .is_some_and(|node| node.next.is_some_and(|node| node.data == 2)));

        assert!(ll.clone().head.is_some_and(|node| node
            .next
            .is_some_and(|node| node.next.is_some_and(|node| node.data == 3))));

        assert!(ll.clone().head.is_some_and(|node| node
            .next
            .is_some_and(|node| node.next.is_some_and(|node| node.next.is_none()))));
    }

    #[test]
    fn pop_from_list() {
        let mut ll = LinkedList::empty();
        ll.push(3);
        ll.push(2);
        ll.push(1);

        assert!(ll.pop() == Some(1));
        assert!(ll.pop() == Some(2));
        assert!(ll.pop() == Some(3));
        assert!(ll.pop().is_none());
    }

    #[test]
    fn find_in_list() {
        let mut ll = LinkedList::empty();
        ll.push(3);
        ll.push(2);
        ll.push(1);

        assert!(ll.find(1));
        assert!(ll.find(2));
        assert!(!ll.find(10));
    }
}
