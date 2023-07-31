use std::io;

fn main() {
    println!("Hello, World");

    // Define a variable
    let mut guess: String = String::new();

    // References are immutable as well
    match io::stdin().read_line(&mut guess) {
        Ok(_length) => println!("Read: {}", guess),
        Err(err) => println!("Error: {}", err)
    }

}
