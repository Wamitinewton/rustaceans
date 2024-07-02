Functions are prevalent in Rust code.

You’ve already seen one of the most important functions in the language: the main function, which is the entry point of many programs. 

You’ve also seen the fn keyword, which allows you to declare new functions.

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

```
Let’s start a new binary project named functions to explore functions further.

Place the another_function example in src/main.rs and run it.

You should see the following output:

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.
```

### Parameters

You can declare functions that take parameters.

```rust
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn main() {
    another_function(5);
}

```sh

$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/functions`
The value of x is: 5

```

### STATEMENT AND EXPRESSIONS

Statements are instructions that perform some action and do not return a value.

Expressions evaluate to a resultant value. Let’s look at some examples.

```rust
fn main() {
    let y = 6;
}

```
Function definitions are also statements; the entire preceding example is a statement in itself.

Statements do not return values. 

Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` (bin "functions") due to 1 previous error; 1 warning emitted

```

