use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect(); //fetch all cli variables from terminal
    let command = args[1].clone(); //fetching the first value form the terminal after the files name
    print!("Args: {:?}", args);
    print!("Command: {}", command)
}
