use std::rc::Rc;

struct Node<T> {
    element: T,
    next: Option<Rc<Node<T>>>,
}

struct List<T> {
    head: Option<Rc<Node<T>>>,
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>, 
}
impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn prepend(&self, element: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                element,
                next: self.head.clone(),
            })),
        }
    }

    fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone() )
        }
    }//wrong

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node_rc) = current {
            // if we are the last strong reference to the rc take ownership of its data.
            if let Ok(node) = Rc::try_unwrap(node_rc) {
                current = node.next;
            } else {
                break;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node_ref| {
            self.next = node_ref.next.as_deref();
            &node_ref.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}