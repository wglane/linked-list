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
            None => {
                return None;
            }
            Some(x) => {
                self.head = x.next;
                return Some((*x).data);
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
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_push_pop() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);

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
    fn test_take() {
        let mut thing: Option<i32> = Some(42);
        let other: Option<i32> = thing.take();
        assert!(other.unwrap() == 42);
        assert!(thing.is_none());
    }
}
