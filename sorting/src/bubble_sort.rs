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
