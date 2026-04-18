fn main() {
    // There are two types of data types in rust
    // 1. Scaler Types

    /*
       1. Signed integers i8, i16, i32, i64, i128 and isize
       2. Unsigned integers u8, u16, u32, u64, u128 and usize
       3. Floating point f32, f64
       4. char unicode scalers values like 'a', 'b' 4 byte each
       5. bool either true or false
       6. The unit type(), whose only possible values is an empty tuple: ()
    */

    let logical: bool = true;

    let a_float: f64 = 2.0;
    // suffix annotation

    let an_integer = 10i32;

    println!("{} {} {}", logical, a_float, an_integer);

    //Mutable variable

    let mut x = 12i32;

    println!("{}",x);



    
}
