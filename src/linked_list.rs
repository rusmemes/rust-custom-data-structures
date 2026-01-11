struct LinkedList<T> {
    head: Option<ListNode<T>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList { head: None }
    }
}

impl<T> LinkedList<T>
where
    T: PartialOrd,
{
    fn contains(&self, value: &T) -> bool {
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            if node.contains(value) {
                return true;
            } else {
                curr = node.next.as_deref();
            }
        }

        false
    }
}

struct ListNode<T> {
    element: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(element: T) -> Self {
        ListNode {
            element,
            next: None,
        }
    }

    fn set_next(&mut self, node: ListNode<T>) {
        self.next = Some(Box::new(node));
    }
}

impl<T> ListNode<T>
where
    T: PartialOrd,
{
    fn contains(&self, value: &T) -> bool {
        self.element == *value
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

struct LinkedListIter<'a, T> {
    node: Option<&'a ListNode<T>>,
}

struct LinkedListIterMut<'a, T> {
    node: Option<&'a mut ListNode<T>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node.take() {
            None => None,
            Some(node) => {
                let some = Some(&mut node.element);
                self.node = node.next.as_deref_mut();
                some
            }
        }
    }
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            None => None,
            Some(node) => {
                let option = &node.next;
                let option = option.as_deref();
                self.node = option;
                Some(&node.element)
            }
        }
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Default::default()
    }

    fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            node: self.head.as_ref(),
        }
    }

    fn iter_mut(&mut self) -> LinkedListIterMut<'_, T> {
        LinkedListIterMut {
            node: self.head.as_mut(),
        }
    }

    fn push(&mut self, element: T) {
        let tail: Option<&mut ListNode<T>> = self.get_tail();

        match tail {
            Some(node) => {
                node.set_next(ListNode::new(element));
            }
            None => {
                self.head = Some(ListNode::new(element));
            }
        }
    }

    fn pop(&mut self) -> Option<T> {
        let mut node = self.head.take()?;
        let next = node.next.take();
        self.head = next.map(|node| *node);
        Some(node.element)
    }

    fn get_tail(&mut self) -> Option<&mut ListNode<T>> {
        let mut curr = self.head.as_mut();

        while let Some(node) = curr {
            if node.next.is_some() {
                curr = node.next.as_deref_mut();
            } else {
                curr = Some(node);
                break;
            }
        }

        curr
    }
}
