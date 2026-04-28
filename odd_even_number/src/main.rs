fn main() {

    let num = 32;
    println!("Check this {} number is even or odd", num);

    if(is_odd_or_even(num)) {
        println!("This is even number");
    } else {
        println!("This is odd number");
    }

}

fn is_odd_or_even(num: i32) -> bool{

    if num % 2 == 0 {
        true
    } else {
        false
    }

}
