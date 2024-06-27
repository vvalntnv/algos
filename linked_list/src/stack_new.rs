use std::mem;

pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    value: T,
    next: Link<T>
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    } 

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node{
            value,
            next: mem::replace(&mut self.head, None)
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
    
    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.value)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match &mut self.head {
            None => None,
            Some(node) => Some(&mut node.value)
        }
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, None);

        while let Some(mut node) = curr_link {
            curr_link = mem::replace(&mut node.next, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_creating_list() {
        let integer_list: List<i32> = List::new();
        
        match integer_list.head {
            Some(_) => assert!(false),
            None => assert!(true)
        }
    }

    #[test]
    fn basic_testing() {
        let mut list: List<usize> = List::new();

        list.push(12);
        list.push(3);
        list.push(4);

        assert_eq!(list.peek(), Some(4).as_ref());
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.peek(), Some(3).as_ref());
        assert_eq!(list.pop(), Some(3));

        assert_eq!(list.peek(), Some(12).as_ref());
        
        let mutatable_value = list.peek_mut();
        match mutatable_value {
            Some(value) =>  *value += 1,
            None => assert!(false)
        }

        assert_eq!(list.pop(), Some(13));

        assert_eq!(list.pop(), None);
    }
}
