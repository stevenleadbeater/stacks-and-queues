struct LinkedList<T> {
    head: Option<Node<T>>,
}

#[derive(Clone)]
struct Node<T> {
    next: Option<Box<Node<T>>>,
    value: T,
}

impl <T> LinkedList<T> where T: Clone {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
        }
    }
    fn push(&mut self, next: T) {
        self.head = match self.head.clone() {
            Some(head) => Some(Node::<T> {
                next: Some(Box::new(head)),
                value: next
            }),
            None => Some(Node::<T> {
                next: None,
                value: next
            })
        };
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.clone() {
            Some(head) => {
                self.head = match head.next {
                    Some(next) => Some(*next),
                    None => None
                };
                Some(head.value)
            },
            None => None
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::linked_list::{LinkedList};

    #[test]
    fn pop_empty() {
        let mut list = LinkedList::<String>::new();
        assert!(list.pop().is_none());
    }

    #[test]
    fn pop_to_empty() {
        let mut list = LinkedList::<String>::new();
        list.push("test".parse().unwrap());
        list.pop();
        assert!(list.pop().is_none());
    }

    #[test]
    fn push_pop() {
        let mut list = LinkedList::<String>::new();
        list.push("test".parse().unwrap());
        let item = list.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), "test");
    }

    #[test]
    fn push_push_pop() {
        let mut list = LinkedList::<String>::new();
        list.push("test".parse().unwrap());
        list.push("test2".parse().unwrap());
        let item = list.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), "test2");
    }

    #[test]
    fn push_push_pop_pop() {
        let mut list = LinkedList::<String>::new();
        list.push("test".parse().unwrap());
        list.push("test2".parse().unwrap());
        let item = list.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), "test2");
        let item = list.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), "test");
    }

    #[test]
    fn push_push_pop_pop_pop() {
        let mut list = LinkedList::<String>::new();
        list.push("test".parse().unwrap());
        list.push("test2".parse().unwrap());
        let item = list.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), "test2");
        let item = list.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), "test");
        assert!(list.pop().is_none());
    }
}