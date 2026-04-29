
fn main() {

    println!("Enter Students Marks");


    let mark = 85;


    if mark < 35 {
        println!("You are failed, Do more Study man!");
    } else if mark > 35 && mark < 59 {
        println!("Passed!, but second class");
    } else if mark > 60 && mark < 74 {
        println!("First Class, Good");
    } else {
        println!("Excellent!.....");
    }


}
