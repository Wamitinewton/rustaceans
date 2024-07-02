# DATA TYPES

```rust
let guess: u32 = "42".parse().expect("Not a number!")


```
failure to add u32 in the above code will return an error since the compiler needs more info on what data is being processed

```sh
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^        ----- type must be known at this point
  |
  = note: cannot satisfy `<_ as FromStr>::Err == _`
help: consider giving `guess` an explicit type
  |
2 |     let guess: /* Type */ = "42".parse().expect("Not a number!");
  |              ++++++++++++

For more information about this error, try `rustc --explain E0284`.
error: could not compile `no_type_annotations` (bin "no_type_annotations") due to 1 previous error


```
### SCALAR TYPES
- Integers
- Floating-point numbers
- Booleans
- Characters


Integer types in Rust
### Integer types in Rust

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128-bit| i128   | u128     |
| arch   | isize  | usize    |


### TUPLE TYPES

A tuple is a general way of grouping together a number of values witha variety of types into one compund type

```rust

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

``` 
the variable 'tup', binds the entire tuple because the tuple is considered a single compund element
To get the individual values out of a tuple, we can use pattern matching to destructure a tuple values like this:

``` rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

```

This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called destructuring because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.


### using period to access tuple

```rust

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

```
This program creates the tuple x and then accesses each element of the tuple using their respective indices.

As with most programming languages, the first index in a tuple is 0.

### THE ARRAY ELEMENT

We write the values in an array as a comma-separated list inside square brackets:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}

```

The type of a is inferred to be [i32; 5] which is an array of 5 i32s.


### ARRAY ELEMENT ACCESS

To access an element of an array, we use the same syntax as with tuples:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

```

### INVALID ARRAY ACCESS

Let’s see what happens if you try to access an element of an array that is past the end of the array.

Say you run this code, similar to the guessing game in Chapter 2, to get an array index from the user:

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

```

This code compiles successfully. If you run this code using cargo run and enter 0, 1, 2, 3, or 4, the program will print out the corresponding value at that index in the array.

If you instead enter a number past the end of the array, such as 10, you’ll see output like this:

```sh
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
