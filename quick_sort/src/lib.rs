pub fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low >= high {
        return
    }
    let pivot_index = particion(arr, low, high);

    quick_sort(arr, low, pivot_index - 1);
    quick_sort(arr, pivot_index, high);

}

pub fn particion(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut idx = low;

    let mut has_already_iterated = false;

    for i in low..high {
        if arr[i] <= pivot {
            idx += if has_already_iterated {1} else {
                has_already_iterated = true;
                0
            };
            let temp = arr[idx];
            arr[idx] = arr[i];
            arr[i] = temp;
        } 
    }

    idx += 1;
    arr[high] = arr[idx];
    arr[idx] = pivot;

    return idx
}


#[cfg(test)]
mod tests{
    use crate::quick_sort;

    
    #[test]
    fn test_sorging() {
        let mut unsorted_array = [1, 12, 3, 5, 8, 3, 6];
        let sorted_array = [1, 3, 3, 5, 6, 8, 12];
        let unsorted_len = unsorted_array.len();

        quick_sort(&mut unsorted_array, 0, unsorted_len - 1);

        println!("{:?}", unsorted_array);

        for i in 0..unsorted_len {
            assert_eq!(unsorted_array[i], sorted_array[i])
        }
    }
}
