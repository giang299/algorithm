
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
impl<T: Default> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        SinglyLinkedList::new()
    }
}
impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None }
    }
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        if let Some(old_head) = self.head.take() {
            let mut new_node = new_node;
            new_node.next = Some(old_head);
            self.head = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(old_head) = self.head.take() {
            self.head = old_head.next;
            Some(old_head.value)
        } else {
            None
        }
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
#[cfg(test)]
mod singly_linked_list_tests {
    use super::*;
    #[test]
    fn new_node_test() {
        let node = Node::new(5);
        assert_eq!(node.value, 5);
        assert!(node.next.is_none());
    }
    #[test]
    fn singly_linked_list_push_front_test() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        assert_eq!(list.peek(), Some(&1));
        list.push_front(2);
        assert_eq!(list.peek(), Some(&2));
        list.push_front(3);
        assert_eq!(list.peek(), Some(&3));
    }
    #[test]
    fn singly_linked_list_pop_front_test() {
        let mut list = SinglyLinkedList::new();
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
    fn singly_linked_list_peek_test() {
        let mut list = SinglyLinkedList::new();
        assert!(list.peek().is_none());
        list.push_front(1);
        assert_eq!(list.peek(), Some(&1));
        list.push_front(2);
        assert_eq!(list.peek(), Some(&2));
        list.pop_front();
        assert_eq!(list.peek(), Some(&1));
        list.pop_front();
        assert!(list.peek().is_none());
    }
    #[test]
    fn singly_linked_list_is_empty_test() {
        let mut list = SinglyLinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
        list.pop_front();
        assert!(list.is_empty());
    }
    #[test]
    fn singly_linked_list_default_test() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::default();
        assert!(list.is_empty());
    }
}
