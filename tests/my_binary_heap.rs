use canbench_demo::MyBinaryHeap;

#[test]
fn test_push_pop() {
    let mut heap = MyBinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(2);

    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), None);
}

#[test]
fn test_peek() {
    let mut heap = MyBinaryHeap::new();
    assert_eq!(heap.peek(), None);
    heap.push(5);
    heap.push(2);
    assert_eq!(heap.peek(), Some(&2));
}

#[test]
fn test_len_empty() {
    let mut heap = MyBinaryHeap::new();
    assert!(heap.is_empty());
    assert_eq!(heap.len(), 0);
    heap.push(10);
    assert!(!heap.is_empty());
    assert_eq!(heap.len(), 1);
}
