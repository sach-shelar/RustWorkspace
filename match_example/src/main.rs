use std::env;

fn main() {

    println!("Students Grade System");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter students marks!");
        return;
    }


    let marks = args[1].clone();
    
    match  marks.parse::<i32>().unwrap() {

        90..=100 => {
            println!("A, Excellent!.....");
        }

        80..=89 => {
            println!("B, Awesome");
        }

        70..=79 =>{
            println!("C, Good");
        }

        40..=69 => {
            println!("D, Ok");
        }

        _ => {
            println!("Bhai tu kar kya raha hai.....");
        }

    }



}
