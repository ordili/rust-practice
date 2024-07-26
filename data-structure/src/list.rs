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

    // 在链表头部插入节点
    pub fn insert_head(&mut self, t: T) {
        let mut node = Box::new(Node::new(t));
        node.next = self.head;
        self.head = NonNull::new(Box::into_raw(node));
        self.len += 1;
    }

    // 删除链表头节点
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

    // insert at ith
    pub fn insert_at(&mut self, index: usize, t: T) {
        let index = if index > self.len { self.len } else { index };
        if index == 0 {
            self.insert_head(t);
            return;
        }

        let mut i = 0;
        let mut head = self.head;
        while i < index - 1 {
            match head {
                Some(head_ptr) => unsafe {
                    let node = Box::from_raw(head_ptr.as_ptr());
                    head = node.next;
                    i += 1;
                },
                _ => break,
            }
        }

        let mut new_node = Box::new(Node::new(t));
        match head {
            Some(head_ptr) => unsafe {
                let mut node = Box::from_raw(head_ptr.as_ptr());
                new_node.next = node.next;
                (*head_ptr.as_ptr()).next = NonNull::new(Box::into_raw(new_node));
                self.len += 1;
            },
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
#[cfg(test)]
mod test {
    use crate::list::LinkedList;
    #[test]
    pub fn test_insert_remove() {
        let mut list = LinkedList::new();
        for v in 0..=10 {
            list.insert_head(v);
            assert_eq!(list.len(), v + 1);
        }
        for v in 10..=0 {
            assert_eq!(list.remove_head(), Some(v));
        }
    }

    #[test]
    pub fn test_insert_at() {
        let mut list = LinkedList::new();
        list.insert_head(1);
        list.insert_at(0, 10);
        assert_eq!(list.remove_head(), Some(10));

        list.insert_at(1, 10);
        assert_eq!(list.remove_head(), Some(1));
    }
}
