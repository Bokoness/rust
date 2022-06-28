//Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Jane");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    //Closure - accepts two i32 (num1, num2) return n1 + n2
    let add_nums1 = |n1: i32, n2: i32| n1 + n2;
    //closure can use outside variables (which is impossible in normal functions)
    let add_nums2 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Sum: {}", add_nums1(3, 3));
    println!("C Sum: {}", add_nums2(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2 //when we dont use semicolon it will replace return
}
