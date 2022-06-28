// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Ness";
    let age = 36;
    //age = 37; // will throw an error becuase vars are immutable
    println!("My name is {} and I'm {}", name, age);
    let mut age = 37; //shadowing are allowed
    println!("My name is {} and I'm {}", name, age);
    age = 36; //its ok, becuase now age is mutable

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Ness", 36);
    println!("{} is {}", my_name, my_age);
}
