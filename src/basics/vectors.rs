use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value from the vector
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value from the vector
    println!("Single Value: {}", numbers[0]); //1

    // Get vector length
    println!("Vector Length: {}", numbers.len()); //5

    // Vectors are heap allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); //24 bytes

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice); // [2, 20]

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers); //[2,4,40,8,10]
}
