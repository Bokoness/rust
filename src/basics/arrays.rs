use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];
    //let mut numbers: [i32; 4] = [1,2,3]; //will throw an error, because expected 4 values

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Slices
    let complete_array_as_slice = &numbers;
    println!("Complete slice: {:?}", complete_array_as_slice);
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}
