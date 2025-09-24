 //          0 (R)
 //       /       \
 //    1 (L)      2 (R)
 //   /   \      /   \
 // 3(LL) 4(LR) 5(RL) 6(RR)
pub struct Heap<T> {
    data: Vec<T>,
    comparator: fn(&T,&T)->bool
}
impl Default for Heap<i32> {
    // Creates a min-heap by default
    fn default() -> Self {
        Heap::new(|a,b| a<b)
    }
}
impl <T:Ord> Heap<T> {
    fn new(comparator: fn(&T,&T)->bool) -> Self {
        Heap { data: Vec::new(), comparator }
    }
    fn push(&mut self, value: T) {
        self.data.push(value);
        self.bubble_up(self.data.len() - 1);
    }
    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let popped_value = self.data.pop();
        self.bubble_down(0);
        popped_value
    }
    fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            // Swap the current node with its parent until the heap property is restored
            let parent_index = (index - 1) / 2;
            if (self.comparator)(&self.data[index], &self.data[parent_index]) {
                self.data.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }
    fn bubble_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let left_child_index = 2 * index + 1;
            let right_child_index = 2 * index + 2;
            let mut smallest_index = index;

            if left_child_index < len && (self.comparator)(&self.data[left_child_index], &self.data[smallest_index]) {
                smallest_index = left_child_index;
            }
            if right_child_index < len && (self.comparator)(&self.data[right_child_index], &self.data[smallest_index]) {
                smallest_index = right_child_index;
            }
            if smallest_index != index {
                self.data.swap(index, smallest_index);
                index = smallest_index;
            } else {
                break;
            }
        }
}
}
#[cfg(test)]
mod heap_tests {
use super::*;
    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new(|a: &i32, b: &i32| a < b);
        heap.push(5);
        heap.push(3);
        heap.push(8);
        heap.push(1);
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), None);
    }
    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new(|a: &i32, b: &i32| a > b);
        heap.push(3);
        heap.push(8);
        heap.push(1);
        heap.push(5);
        assert_eq!(heap.peek(), Some(&8));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }
    #[test]
    fn test_strings() {
        let mut heap = Heap::new(|a: &String, b: &String| a.len() < b.len());
        heap.push("apple".to_string());
        heap.push("banana".to_string());
        heap.push("kiwi".to_string());
        assert_eq!(heap.peek(), Some(&"kiwi".to_string()));
        assert_eq!(heap.pop(), Some("kiwi".to_string()));
    }
    #[test]
    fn test_empty_pop() {
        let mut heap: Heap<i32> = Heap::new(|a, b| a < b);
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.peek(), None);
        heap.push(10);
        assert_eq!(heap.pop(), Some(10));
        assert_eq!(heap.pop(), None);
    }
}
