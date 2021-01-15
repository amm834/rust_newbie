use std::mem;
pub fn run() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    // Array
    array[0] = 0;
    // Capacity
    println!("Capacity is: {:?} bytes.", mem::size_of_val(&array));
    // Slice
    let slice: &[i32] = &array[1..3];
    println!("{:?}", slice);
    // Debug Mode
    println!("{:?}", array);

}
