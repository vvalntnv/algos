pub fn bubble_sort(arr: &mut [isize]) {
    let length = arr.len();

    for i in 0..length {
        for j in 0..(length - i - 1) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_array_equality, bubble_sort::bubble_sort, print_array};

    #[test]
    fn test_bubble_sort() {
        let mut array = [1, 12, 4, 5, 98, 12, 89, 2, 0];
        let sorted_array = [0, 1, 2, 4, 5, 12, 12, 89, 98];
        bubble_sort(&mut array); 

        print_array!(array);
        assert_array_equality!(array, sorted_array);
    }
}
