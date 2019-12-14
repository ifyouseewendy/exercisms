use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut head = &self.head;
        let mut count = 0;

        while let Some(node) = head.as_ref() {
            head = &node.next;
            count = count + 1;
        }

        count
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut v: Vec<T> = self.into();
        v.reverse();
        Self::from_iter(v)
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(v: I) -> Self {
        let mut list = Self::new();
        for elem in v.into_iter() {
            list.push(elem);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut head = self.head;
        let mut v = vec![];

        while let Some(node) = head.take() {
            v.push(node.elem);
            head = node.next;
        }

        v.reverse();
        v
    }
}
