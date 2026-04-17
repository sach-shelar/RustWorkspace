const THRE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Hello, world!");
    // Constant
    /*
       1. Constant are always immutable
       2. We must write the type explicitly.
       3. Constant can be declared at any scope. including global scope.
       4. The value must be compile time constant.
    */

    const MAX_POINT: u32 = 100_000;
    println!("{MAX_POINT}");

    println!("Three hourse in seconds: {THRE_HOURS_IN_SECONDS}");

    // Shadowing
    /*
       1.
    */
    let t = 10;
    let t = t + 10;
    println!("The value of t is: {}", t);
}
