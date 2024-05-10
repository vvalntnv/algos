mod bubble_sort;
mod mrc;

use bubble_sort::bubble_sort;
fn main() {
    println!("Tesing Bubble Sort!");
    test_bubble_sort();
}

fn test_bubble_sort() {
    let mut array = [1, 12, 4, 5, 98, 12, 89, 2, 0];
    let sorted_array = [0, 1, 2, 4, 5, 12, 12, 89, 98];
    bubble_sort(&mut array); 

    print_array!(array);
    assert_array_equality!(array, sorted_array);
}
