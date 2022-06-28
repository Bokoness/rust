pub fn fmt() {
    println!("My name is {1} and Im {0} years old", 36, "Ness");

    println!(
        "My name is {name} and Im {age} years old",
        name = "Ness",
        age = 36,
    );

    println!("Binary: {:b} Hex: {:x} Ocatal: {:o}", 10, 10, 10);

    //Placeholder for debug (we can add tuple in placeholder)
    println!("{:?}", (12, true, "Hello World"));

    //Basic math placeholder
    println!("10 + 10 = {}", 10 + 10);
}
