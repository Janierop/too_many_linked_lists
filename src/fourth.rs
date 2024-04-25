use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

struct List<T> {
    head: NodeLink<T>,
    tail: NodeLink<T>,
}

type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    element: T,
    prev: NodeLink<T>,
    next: NodeLink<T>,
}

struct IntoIter<T> {
    list: List<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None, tail: None }
    }

    fn push_front(&mut self, element: T) {
        // create new node
        // new head next is old head
        // old head prev is new head
        // if array was empty make sure tail also points to the new node

        match self.head.take() {
            Some(old_head) => {
                // create new head with old head as next
                self.head = Some(Rc::new(RefCell::new(
                    Node {
                        element,
                        prev: None,
                        next: Some(old_head.clone()),
                    }
                )));
                // set old head prev to new head
                old_head.borrow_mut().prev = self.head.clone();
            },
            None => {
                self.head = Some(Rc::new(RefCell::new(
                    Node {
                        element,
                        prev: None,
                        next: None,
                    }
                )));
                self.tail = self.head.clone();
            },
        }
    }

    fn push_back(&mut self, element: T) {
        let new_tail = Rc::new(RefCell::new(
            Node {
                element,
                prev: None,
                next: None,
            }
        ));
        match self.tail.take() {
            Some(old_tail) => {
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                old_tail.borrow_mut().next = Some(new_tail.clone());
                self.tail = Some(new_tail);
            },
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            },
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                },
                None => self.tail = None,
            }
            /* We removed the list's ref to old head and the next nodes ref to
            old head and there can be no other refs to node head because we do
            not expose ant ways for consumers of the api to take references to
            it. So we know that try_unwrap will be succellful. */
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                },
                None => self.head = None,
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().element
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.element)
        })
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.element)
        })
    }

    fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.element)
        })
    }

    fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.element)
        })
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

}

