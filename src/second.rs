struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct IntoIter<T> {
    list: List<T>,
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

    fn push(&mut self, element: T) {
        self.head = Some(Box::new(
            Node {
                element, 
                next: self.head.take(),
            }
        ));
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node_box) => {
                self.head = node_box.next;
                Some(node_box.element)
            },
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node_box| &node_box.element)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node_box| &mut node_box.element)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node_box) = current {
            current = node_box.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T>  {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
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

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node_ref| {
            self.next = node_ref.next.as_deref_mut();
            &mut node_ref.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
        list.push(3);
        assert_eq!(list.peek(), Some(&3));

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek(), Some(&1));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        assert_eq!(list.peek(), Some(&4));
        list.push(5);
        assert_eq!(list.peek(), Some(&5));

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.peek(), Some(&4));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.peek(), Some(&1));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.peek(), None);
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}