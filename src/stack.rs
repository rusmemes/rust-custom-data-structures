type Link<T> = Option<Box<Node<T>>>;

pub struct Stack<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(Stack<T>);
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn empty(&self) -> bool {
        match self.head {
            None => true,
            Some(_) => false,
        }
    }

    pub fn push(&mut self, item: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { item, next }))
    }

    pub fn pop(&mut self) -> Option<T> {
        let option = self.head.take();

        if let Some(b) = option {
            let item = b.item;
            self.head = b.next;
            Some(item)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.item)
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
            &node.item
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.item
        })
    }
}

// not to blow the stack on huge stacks
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut option = self.head.take();
        while let Some(mut b) = option {
            option = b.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::stack::Stack;

    #[test]
    fn test() {
        let mut stack = Stack::new();

        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));

        if let Some(r) = stack.peek_mut() {
            *r = 2;
        }

        for x in stack.iter() {
            assert_eq!(x, &2);
        }

        for x in stack.iter_mut() {
            *x = 3;
        }

        for x in stack.into_iter() {
            assert_eq!(x, 3);
        }
    }
}
