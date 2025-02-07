use crate::stack::Stack;

pub struct Queue<T> {
    incoming: Option<Stack<T>>,
    outgoing: Stack<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            incoming: Some(Stack::new()),
            outgoing: Stack::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.incoming = self.incoming.take().map(|mut list| {
            list.push(item);
            list
        })
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.move_incoming_if_need();
        self.outgoing.pop()
    }

    fn move_incoming_if_need(&mut self) {
        if self.outgoing.empty()
            && !self
                .incoming
                .as_ref()
                .map(|list| list.empty())
                .unwrap_or(true)
        {
            self.incoming = self.incoming.take().map(|list| {
                for x in list.into_iter() {
                    self.outgoing.push(x)
                }
                Stack::new()
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(4);
        queue.enqueue(5);
        assert_eq!(queue.dequeue(), Some(4));

        queue.enqueue(6);
        queue.enqueue(7);

        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(7));
        assert_eq!(queue.dequeue(), None);
    }
}
