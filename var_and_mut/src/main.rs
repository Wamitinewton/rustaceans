fn main() {
    mutability();
    shadowing();
}

fn mutability(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}


// shadowing
fn shadowing(){
    let x = 5;
    let x = x + 1;

    {
        // opening an inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}