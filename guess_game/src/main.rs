use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number");

    let secrete_number = rand::thread_rng().gen_range(1..=100);

    println!("Please intput your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("The secrete number is: {secrete_number}");
    println!("You gussed: {guess}");

    match guess.cmp(&secrete_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("To Big"),
        Ordering::Equal => println!("You win"),
    }
}
