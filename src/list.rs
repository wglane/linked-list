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
            // take whatever head pointed to and make new_node.next point to it (set head temporarily to None)
            // note that we can mutate self's values but NOT change ownership, unless we use Option::take()
            next: self.head.take(),
        });

        // now set head to new_node
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        // TODO:
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_node() {
        // TODO:
    }

    #[test]
    fn test_take() {
        let mut thing: Option<i32> = Some(42);
        let other: Option<i32> = thing.take();
        assert!(other.unwrap() == 42);
        assert!(thing.is_none());
    }
}
