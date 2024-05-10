#[macro_export]
macro_rules! print_array {
    ($array:expr) => {
        print!("[{}, ", $array[0]);
        for i in 1..$array.len() - 1 {
            print!("{}, ", $array[i]);
        }
        print!("{}]\n", $array[$array.len() - 1]);
    };    
}

#[macro_export]
macro_rules! assert_array_equality {
    ($array_1:expr, $array_2:expr) => {
        if $array_1.len() != $array_2.len() {
            panic!("Arrays not of same size");
        }

        for i in 0..$array_1.len() {
            assert_eq!($array_1[i], $array_2[i]);
        }
    };
}
