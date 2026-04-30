/**

Control Flow

1. Loops
2. For Loops
3. While Loops


*/

fn main() {

    let mut count = 0u32;

    //Simple Loop
    loop {
        count+=1;
        println!("Continue loop: {}", count);
        if count == 10 {
            break;
        }
    }

    // Nesting and Labels
    'outer:loop {
         loop {
            count+=1;
            println!("Labels Loop: {}",count);
            if count == 20 {
                break 'outer;
            }
        }
    }

    // Returning value from loop

    let final_counter = loop {
        count+=1;
        println!("Return Loop: {}",count);
        if count == 40 {
            break count;
        }
    };

    println!("Final Counter Value : {}",final_counter);

    // For loop

    for n in 0..=final_counter {
        println!("For Loop : {}",n);
    }




}
