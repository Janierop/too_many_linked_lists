use std::ptr;

struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    element: T,
    next: *mut Node<T>,
}

// ...

struct IntoIter<T>{
    list: List<T>,
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// implementing a singly linked queue so we push on the tail and pop off the head
impl<T> List<T> {
    fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    fn push(&mut self, element: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(
                Node { element, next: ptr::null_mut() }
            ));
            
            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail
            }
            self.tail = new_tail;
        }
    }

    fn pop(&mut self) -> Option<T> {
        unsafe {    
            if self.head.is_null() {
                None
            } else {
                let old_head = Box::from_raw(self.head);
                self.head = old_head.next;
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
                Some(old_head.element)
            }
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    fn iter(&self) -> Iter<T> {
        Iter { next: unsafe { self.head.as_ref() } }
    }

    fn iter_mut(&self) -> IterMut<T> {
        IterMut { next: unsafe { self.head.as_mut() } }
    }

    fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.element) }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.element) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_ref() };
            &node.element
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = unsafe { node.next.as_mut() };
            &mut node.element
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
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
