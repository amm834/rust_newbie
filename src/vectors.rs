use std::mem;
pub fn run() {
// Vector is resiizable array
    let mut array: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Push Vector
    array.push(20);
    // Pop
    array.pop();
    // Array
    array[0] = 0;
    // Capacity
    println!("Capacity is: {:?} bytes.", mem::size_of_val(&array));
    // Slice
    let slice: &[i32] = &array[1..3];
    println!("{:?}", slice);
    // Debug Mode
    println!("{:?}", array);
    //Loop Through
    for x in array.iter(){
    println!("Number loop: {}",x);
    }
    // Loop and Mutate Value
    for x in array.iter_mut(){
    *x *= 2;
    }
    println!("Mutate value: {:?}",
    array);
}
