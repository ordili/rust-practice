use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: val,
            next: None,
        }
    }
}
pub struct LinkedList<T> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { len: 0, head: None }
    }

    pub fn insert_head(&mut self, t: T) {
        let mut node = Box::new(Node::new(t));
        node.next = self.head;
        self.head = NonNull::new(Box::into_raw(node));
        self.len += 1;
    }

    pub fn remove_head(&mut self) -> Option<T> {
        match self.head {
            Some(head) => unsafe {
                self.len -= 1;
                let node = Box::from_raw(head.as_ptr());
                self.head = node.next;
                Some(node.val)
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::list::LinkedList;
    #[test]
    pub fn test_insert_remove() {
        let mut list = LinkedList::new();
        list.insert_head(1);
        list.insert_head(2);
        list.insert_head(3);
        assert_eq!(list.remove_head(), Some(3));
        assert_eq!(list.remove_head(), Some(2));
        assert_eq!(list.remove_head(), Some(1));
        assert_eq!(list.remove_head(), None);
    }
}
