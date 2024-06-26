use std::u32;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { 
            value: value,
            next: None
        }
    }
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
    length: u32
}

impl<T> Queue<T> {
    pub fn new() -> Self { 
        Queue {
            head: None, 
            tail: None, 
            length: 0
        }
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    pub fn enqueue(&mut self, value: T) { 
        let new_node = Box::new(Node::new(value));
        let new_node_ptr = Box::into_raw(new_node);

        // Check if head is None
        // If is None, head and tails should be the same
        // when appending if head is Some, only add to the tail
        unsafe {
            if let Some(tail) = self.tail {
                (*tail).next = Some(Box::from_raw(new_node_ptr));
            } else {
                self.head = Some(Box::from_raw(new_node_ptr));
            }
            self.tail = Some(new_node_ptr); 
        }

        self.length += 1; 
    }

    pub fn deque(&mut self) -> Option<T> {
        let head = self.head.take();
        
        if let Some(head) = head {
            let new_head = head.next;
            self.head = new_head;
            self.length -= 1;

            if self.head.is_none() {
                self.tail = None;
            }

            Some(head.value)
        } else {
            None
        }

    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head) = &self.head {
            Some(&head.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::Queue;

    #[test]
    fn test_queue() {
        let mut queue: Queue<i32> = Queue::new(); 
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(1);
        queue.enqueue(3);

        println!("First value is: {}", queue.peek().unwrap());
        assert!(queue.len() == 4);

        assert!(queue.deque().unwrap() == 3);
        assert!(queue.len() == 3);
        println!("Deque 3");

        assert!(queue.deque().unwrap() == 4);
        println!("Deque 4");

        assert!(queue.deque().unwrap() == 1);
        println!("Deque 1");

        assert!(queue.deque().unwrap() == 3);
        println!("Deque 3");

        assert!(queue.deque().is_none());
        println!("Deque None");

    }
}
