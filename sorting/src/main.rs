mod bubble_sort;
mod mrc;
mod queue;

use bubble_sort::bubble_sort;
use queue::Queue;

fn main() {
    println!("Tesing Bubble Sort!");
    test_bubble_sort();
    test_queue();
}

fn test_bubble_sort() {
    let mut array = [1, 12, 4, 5, 98, 12, 89, 2, 0];
    let sorted_array = [0, 1, 2, 4, 5, 12, 12, 89, 98];
    bubble_sort(&mut array); 

    print_array!(array);
    assert_array_equality!(array, sorted_array);
}

fn test_queue() {
    let mut queue: Queue<i32> = Queue::new(); 
    queue.enqueue(3);
    queue.enqueue(4);
    queue.enqueue(1);
    queue.enqueue(3);

    println!("First value is: {}", queue.peek().unwrap());
    assert!(queue.len() == 4);

    assert!(queue.deque().unwrap() == 3);
    assert!(queue.len() == 3);
    println!("Deque 3");

    assert!(queue.deque().unwrap() == 4);
    println!("Deque 4");

    assert!(queue.deque().unwrap() == 1);
    println!("Deque 1");

    assert!(queue.deque().unwrap() == 3);
    println!("Deque 3");

    assert!(queue.deque().is_none());
    println!("Deque None");

}
