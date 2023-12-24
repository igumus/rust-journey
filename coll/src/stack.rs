use std::boxed::Box;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct IntoIter<T>(Stack<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
#[derive(Debug)]
pub struct Stack<T> {
    head: Link<T>,
    count: usize,
}

impl<T> Stack<T> {
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
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn push(&mut self, item: T) {
        let node = Box::new(Node {
            value: item,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.count -= 1;
            node.value
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut item) = current_link {
            current_link = item.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack_creation() {
        let stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn test_stack_push() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.is_empty(), true);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.is_empty(), false);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek(), None);

        stack.push(1);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_peek_mut() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek_mut(), None);

        stack.push(1);
        assert_eq!(stack.len(), 1);
        if let Some(item) = stack.peek_mut() {
            *item += 1;
        }
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_into_iter() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        let mut iterator = stack.into_iter();
        assert_eq!(iterator.next(), Some(5));
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_stack_iter() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        let mut iterator = stack.iter();
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_stack_iter_mut() {
        let mut stack = Stack::<i32>::new();
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);

        for item in stack.iter_mut() {
            *item = 10;
        }

        for item in stack.iter() {
            assert_eq!(item, &10);
        }
    }
}
