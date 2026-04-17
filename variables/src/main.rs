fn main() {
    println!("Hello, world!");

    // Shadowing
    let t = 10;
    let t = t + 10;
    println!("The value of t is: {}", t);
}
