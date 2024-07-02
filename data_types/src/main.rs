use std::io;

fn main() {
    tuple();
    arrays();
    invalid_array();
}


// TUPLE TYPES

 fn tuple(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // we are going to use pattern matching 
    // to destruct the tuple

    let (x, y, z) = tup;

    println!("the value of yx, y and z is: {x}, {y}, {z}");
 }

 // ARRAYS 

 fn arrays(){
    let a = [1,2,3,4,5,6];
    println!("the value of a is: {}", a[0]);
 }

 // Invalid array access

 fn invalid_array(){
    let a = [1,2,3,4,5,6];

    println!("Please enter any array index for an element");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element = a[index];

    println!("the value of the element at the index {index} is : {element}")
 }