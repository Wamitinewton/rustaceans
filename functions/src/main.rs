fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurements(5, 'c');

    statement_and_expressions();

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(5);

    println!("The value of y is: {y}");
}

// PARAMETERS

fn another_function(x: i32){
    println!("the value of x is: {x}");
}


// Defining Multiple Parameters

fn  print_labeled_measurements(value: i32, unit_label: char){
    println!("{value}{unit_label}");
}

// STATEMENT AND EXPRESSIONS

fn statement_and_expressions(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Function Return Values

fn five() -> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}