fn main() {
    println!("Array Sample");


    // fixed size of array
    let xs: [i32; 4] = [1,2,3,4];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [1;500];

    eprintln!("Size of array is {}", ys.len());

    for i in 0..ys.len() {
        println!("{}",ys[i]+(i as i32));
    }

    // Slice in array
    custom_slice(&ys[90 .. 100]);


    //empty array
    let empty_array: [i32; 0] = [];
    assert_eq!(&empty_array, &[]);

}

fn custom_slice(slice: &[i32]){

    for i in 0..slice.len(){
        eprintln!("Slice : {}",slice[i] + (i as i32));
    }

}