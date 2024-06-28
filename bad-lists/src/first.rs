use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    value: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List {head: Link::Empty}
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_node = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr_node {
            curr_node = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        let mut list = super::List::new();

        assert_eq!(list.pop(), None);

        // Populate the list
        list.push(12);
        list.push(10);
        list.push(19);

        // Check normal removal
        assert_eq!(list.pop(), Some(19));
        assert_eq!(list.pop(), Some(10));
    }
}
