/// Last In First Out data structure
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>> 
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

struct Stack<T> {
    tail: Option<*mut Node<T>>, 
    length: u32
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            tail: None,
            length: 0
        }
    } 

    pub fn append(&mut self, item: T) {
        let new_node = Box::new(Node::new(item)); 
        let node_pointer = Box::into_raw(new_node);

        let new_tail = self.tail.take()
            .map(|tail| {
                unsafe {
                    (*node_pointer).next = Some(Box::from_raw(tail));
                    node_pointer
                }
            })
            .or_else(|| Some(node_pointer));

        self.tail = new_tail;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.take()
            .map(|tail| {
                unsafe {
                    let mut tail = Box::from_raw(tail);
                    let next = tail.next.take();

                    if let Some(node) = next {
                        self.tail = Some(Box::into_raw(node));
                    } else {
                        self.tail = None
                    }

                    let val = tail.value;
                    self.length -= 1;
                    val
                }
            })
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            if let Some(node) = self.tail {
                Some(&(*node).value)
            } else {
                None
            }
        }
    }
}

impl<I, T> From<I> for Stack<T> 
where
    I: IntoIterator<Item = T>
{
    fn from(value: I) -> Self {
        let mut iterator = value.into_iter();
        let mut stack = Stack::new();

        loop {
            let current = iterator.next();
            match current {
                None => break,
                Some(value) => {
                    stack.append(value)
                }
            }
        }
        return stack;
    }
}

#[cfg(test)]
mod test_stack {
    use crate::stack::Stack;

    #[test]
    fn test_stack_from() {
        let arr = [1, 2, 3, 4, 5, 6]; 
        let mut stack = Stack::from(arr);

        assert_eq!(stack.peek().unwrap(), &6);
        assert_eq!(stack.pop().unwrap(), 6);
        assert_eq!(stack.pop().unwrap(), 5);
        assert_eq!(stack.pop().unwrap(), 4);
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 1);
        assert_eq!(stack.pop(), None);
    }
}
