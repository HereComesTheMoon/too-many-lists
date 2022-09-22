use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node { 
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                result = Some(node.elem);
                self.head = node.next;
            },
            Link::Empty => {
                result = None;
            },
        }
        result
    }

    pub fn peek(&self) -> Option<i32> {
        match &self.head {
            Link::Empty => { None },
            Link::More(node) => { Some(node.elem) },
        }
    }
}


mod test {
    #[cfg(test)]
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(1));
        list.push(2);
        assert_eq!(list.peek(), Some(2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }
}
