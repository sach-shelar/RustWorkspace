fn main() {

    let x:i32 = 20;

    prints_number(x);



}

fn prints_number(mut x: i32){

    x = x + 2;

    println!("{}",x);

}
