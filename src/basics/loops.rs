pub fn run() {
    let mut count = 0;

    // Infinite Loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    // While Loop (FizzBuzz algo)
    while count <= 100 {
        //if divisable by 5 and 3 print fuzzbuzz
        if count % 15 == 0 {
            println!("fizzbuzz");
        //if divisable by 3 print fizz
        } else if count % 3 == 0 {
            println!("fizz");
        //if divisable by 5 print buzz
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    // For Range (FizzBuzz algo)
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x);
        }
    }
}
