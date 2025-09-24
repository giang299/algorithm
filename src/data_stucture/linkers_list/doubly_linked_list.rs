use std::ptr::NonNull as NoneNull;
struct Node<T> {
    value: T,
    next:Option<NoneNull<Node<T>>>,
    prev:Option<NoneNull<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}
pub struct DoublyLinkedList<T> {
    len: usize,
    head:Option<NoneNull<Node<T>>>,
    tail:Option<NoneNull<Node<T>>>,
    marker: std::marker::PhantomData<Box<Node<T>>>,
}
impl<T: Default> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        DoublyLinkedList::new()
    }
}
impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            len: 0,
            head: None,
            tail: None,
            marker: std::marker::PhantomData,
        }
    }
    pub fn push_front(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.next = self.head;
        new_node.prev = None;
        let node_ptr = NoneNull::new(Box::into_raw(new_node));
        match self.head {
            None =>{self.tail = node_ptr},
            Some(old_head) => unsafe { (*old_head.as_ptr()).prev = node_ptr },
        }
        self.head = node_ptr;
        self.len += 1;
    }
    pub fn push_back(&mut self, value: T) {
        let mut new_node = Box::new(Node::new(value));
        new_node.prev = self.tail;
        new_node.next = None;
        let node_ptr = NoneNull::new(Box::into_raw(new_node));
        match self.tail {
            None => {self.head = node_ptr},
            Some(old_tail) => unsafe { (*old_tail.as_ptr()).next = node_ptr },
        }
        self.tail = node_ptr;
        self.len += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            unsafe {
                let boxed_node = Box::from_raw(node.as_ptr());
                self.head = boxed_node.next;
                match self.head {
                    None => self.tail = None,
                    Some(new_head) => (*new_head.as_ptr()).prev = None,
                }
                self.len -= 1;
                boxed_node.value
            }
        })
    }
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            unsafe {
                let boxed_node = Box::from_raw(node.as_ptr());
                self.tail = boxed_node.prev;
                match self.tail {
                    None => self.head = None,
                    Some(new_tail) => (*new_tail.as_ptr()).next = None,
                }
                self.len -= 1;
                boxed_node.value
            }
        })
    }
    pub fn insert_ith(&mut self, index: usize, value: T) {
        if index == 0 {
            self.push_front(value);
        } else if index >= self.len {
            self.push_back(value);
        } else {
            let mut new_node = Box::new(Node::new(value));
            let mut current = self.head;
            for _ in 0..index {
                current = unsafe { current.unwrap().as_ref().next };
            }
            let prev = unsafe { current.unwrap().as_ref().prev };
            new_node.prev = prev;
            new_node.next = current;
            let new_node_ptr = NoneNull::new(Box::into_raw(new_node));
            unsafe {
                if let Some(prev) = prev {
                    (*prev.as_ptr()).next = new_node_ptr;
                }
                if let Some(current) = current {
                    (*current.as_ptr()).prev = new_node_ptr;
                }
            }
            self.len += 1;
        }
    }
    pub fn peek_ith(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let mut current = self.head;
        for _ in 0..index {
            current = unsafe { current.unwrap().as_ref().next };
        }
        current.map(|node| unsafe { &node.as_ref().value })
    }
    pub fn peek_front(&self) -> Option<&T> {
        self.head.map(|node| unsafe { &node.as_ref().value })
    }
    pub fn peek_back(&self) -> Option<&T> {
        self.tail.map(|node| unsafe { &node.as_ref().value })
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
#[cfg(test)]
mod doubly_linked_list_tests {
    use super::*;
    #[test]
    fn new_node_test() {
        let node = Node::new(5);
        assert_eq!(node.value, 5);
        assert!(node.prev.is_none());
        assert!(node.next.is_none());
    }
    #[test]
    fn doubly_linked_list_push_front_test() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&1));
        list.push_front(3);
        assert_eq!(list.peek_front(), Some(&3));
        assert_eq!(list.peek_back(), Some(&1));
    }
    #[test]
    fn doubly_linked_list_push_back_test() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));
        list.push_back(2);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&2));
        list.push_back(3);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));
    }
    #[test]
    fn doubly_linked_list_pop_front_test() {
        let mut list = DoublyLinkedList::new();
        assert!(list.pop_front().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.pop_front().is_none());
    }
    #[test]
    fn doubly_linked_list_pop_back_test() {
        let mut list = DoublyLinkedList::new();
        assert!(list.pop_back().is_none());
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.pop_back().is_none()); 
    }
    #[test]
    fn doubly_linked_list_peek_test() {
        let mut list = DoublyLinkedList::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        list.push_front(1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&1));
        list.pop_front();
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));
        list.pop_front();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
    }
    #[test]
    fn doubly_linked_list_is_empty_test() {
        let mut list = DoublyLinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
        list.pop_front();
        assert!(list.is_empty());
    }
    #[test]
    fn doubly_linked_list_default_test() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::default();
        assert!(list.is_empty());
    }
    #[test]
    fn doubly_linked_list_insert_ith_test() {
        let mut list = DoublyLinkedList::new();
        list.insert_ith(0, 1);
        assert_eq!(list.peek_front(), Some(&1));
        assert_eq!(list.peek_back(), Some(&1));
        list.insert_ith(0, 2);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&1));
        list.insert_ith(1, 3);
        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_back(), Some(&1));
        assert_eq!(list.peek_ith(1), Some(&3));
        list.insert_ith(3, 4);
        assert_eq!(list.peek_back(), Some(&4));
        assert_eq!(list.peek_ith(3), Some(&4));
    }
    #[test]
    fn doubly_linked_list_peek_ith_test() {
        let mut list = DoublyLinkedList::new();
        assert!(list.peek_ith(0).is_none());
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.peek_ith(0), Some(&1));
        assert_eq!(list.peek_ith(1), Some(&2));
        assert_eq!(list.peek_ith(2), Some(&3));
        assert!(list.peek_ith(3).is_none());
    }
}
