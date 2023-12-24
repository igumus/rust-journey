use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: PLink<T>,
}

type PLink<T> = Option<Rc<Node<T>>>;

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

#[derive(Debug)]
pub struct PersistentStack<T> {
    head: PLink<T>,
    count: usize,
}

impl<T> PersistentStack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    pub fn prepend(&self, val: T) -> Self {
        Self {
            count: self.count + 1,
            head: Some(Rc::new(Node {
                value: val,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        let new_count = if self.count == 0 { 0 } else { self.count };
        Self {
            count: new_count,
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for PersistentStack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut link) = Rc::try_unwrap(node) {
                head = link.next.take();
            } else {
                break;
            }
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

#[cfg(test)]
mod tests {
    use super::PersistentStack;

    #[test]
    fn test_persistent_stack_creation() {
        let stack = PersistentStack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn test_persistent_stack_prepend() {
        let stack = PersistentStack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);
        let other = stack.prepend(1);
        assert_eq!(other.len(), 1);
        assert_eq!(other.is_empty(), false);
        assert_eq!(other.peek(), Some(&1));
        let other = other.prepend(2).prepend(3).prepend(4);
        assert_eq!(other.len(), 4);
        assert_eq!(other.is_empty(), false);
        assert_eq!(other.peek(), Some(&4));
    }

    #[test]
    fn test_persistent_stack_tail() {
        let stack = PersistentStack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);
        let tail = stack.tail();
        assert_eq!(tail.len(), 0);
        assert_eq!(tail.is_empty(), true);
    }

    #[test]
    fn test_persistent_stack_basics() {
        let list = PersistentStack::new();
        assert_eq!(list.peek(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.peek(), Some(&3));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&2));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&1));

        let list = list.tail();
        assert_eq!(list.peek(), None);

        let list = list.tail();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_persistent_stack_iter() {
        let list = PersistentStack::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
