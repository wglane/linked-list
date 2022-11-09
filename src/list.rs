#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    data: T,
    next: NextNode<T>,
}

// equivalent to *Node<T> in e.g. C++
type NextNode<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: NextNode<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            // take whatever head pointed to (value AND ownership)
            // and make new_node.next point to it (set head temporarily to None).
            // note that we can mutate self's values but NOT change ownership, unless we use Option::take()
            next: self.head.take(),
        });

        // now set head to new_node
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            // note Box<T> implements Deref trait, which automatically dereferences with the dot (`.`) operator.
            self.head = node.next;
            node.data
        })
    }

    /// alternate implementation of self.pop()
    fn pop_verbose(&mut self) -> Option<T> {
        let node = self.head.take();
        match node {
            None => None,
            Some(x) => {
                self.head = x.next;
                Some((*x).data)
            }
        }
    }

    fn size(&self) -> usize {
        let mut count = 0;
        let mut cur = &self.head;
        while cur.is_some() {
            cur = &cur.as_ref().unwrap().next;
            count += 1;
        }
        count
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    fn empty(&self) -> bool {
        self.head.is_none()
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// Iter has some (vague) lifetime, 'a.
impl<T> Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_push_pop() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
        assert_eq!(list.empty(), true);

        list.push(3);
        list.push(2);
        list.push(1);
        assert_eq!(list.size(), 3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);

        list.push(3);
        list.push(2);
        list.push(1);

        assert_eq!(list.pop_verbose(), Some(1));
        assert_eq!(list.pop_verbose(), Some(2));
        assert_eq!(list.pop_verbose(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek() {
        let s = "cat".to_string();
        let mut list = List::new();
        list.push(s);
        assert_eq!(list.peek().unwrap(), "cat");

        match list.peek_mut() {
            None => (),
            Some(s) => {
                *s = "dog".to_string();
            }
        }
        assert_ne!(list.peek().unwrap(), "cat");
        assert_eq!(list.peek().unwrap(), "dog");
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        let mut iter = list.into_iter();
        for i in 1..4 {
            assert_eq!(iter.next().unwrap(), i);
        }
        assert_eq!(iter.next(), None);

        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        let mut iter = list.iter();
        for i in 1..4 {
            assert_eq!(iter.next().unwrap(), &i);
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_take() {
        let mut thing: Option<i32> = Some(42);
        let other: Option<i32> = thing.take();
        assert!(other.unwrap() == 42);
        assert!(thing.is_none());
    }
}
