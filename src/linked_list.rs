struct LinkedList<'a, T> {
    head: Option<& 'a Node<'a, T>>,
}

struct Node<'a, T> {
    next: Option<& 'a Node<'a, T>>,
    value: T,
}

impl <'a, T> LinkedList<'a, T> {
    fn push(&self, next: Node<T>) {

    }

    fn pop(&self) -> Option<&Node<'a, T>> {
        self.head
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn pop() {

    }
}