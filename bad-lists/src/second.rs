pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {next: self.head.as_deref_mut()}
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: Link::None}
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_node = self.head.take();
        while let Link::Some(mut boxed_node) = curr_node {
            curr_node = boxed_node.next.take();
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

    #[test]
    fn peek() {
        let mut list = super::List::new();

        assert_eq!(list.peek(), None);

        list.push(12);
        list.push(10);
        list.push(19);

        assert_eq!(list.peek(), Some(&mut 19));
        
        list.peek().map(|value| {
            *value = 23
        });

        assert_eq!(list.pop(), Some(23));
        assert_eq!(list.peek(), Some(&mut 10));
    }

    #[test]
    fn iter() {
        let mut list = super::List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
