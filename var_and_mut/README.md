# Rust Variables, Mutability, and Shadowing

This repository contains a simple Rust program that demonstrates the concepts of variables, mutability, and shadowing.

## Table of Contents
1. [Variables](#variables)
2. [Mutability](#mutability)
3. [Shadowing](#shadowing)



## Variables
In Rust, variables are immutable by default. To declare a mutable variable, you need to use the mut keyword.


```rust
fn main() {
    mutability();
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

```

In the mutability function, we declare a mutable variable x and assign it the value 5. We then print the value of x, change it to 6, and print it again.

## Mutability
Mutability allows you to modify the value of a variable. In the previous example, we used the mut keyword to declare x as mutable.

```rust

let mut x = 5;
x = 6;
```
By using mut, we can change the value of x from 5 to 6.

## Shadowing
Shadowing allows you to declare a new variable with the same name as an existing variable, effectively hiding it. The new variable will have its own scope.

```rust

fn shadowing() {
    let x = 5;
    let x = x + 1;

    {
        // opening an inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

```

In the shadowing function, we first declare a variable x and assign it the value 5. Then, we shadow x by declaring a new variable x and assigning it the value of x + 1.

Inside the inner scope, we declare another x and assign it the value of x * 2. We then print the value of x in the inner scope.

After the inner scope, we print the value of x, which is still 6 because the inner x is shadowed by the outer x.

I hope this README helps you understand the concepts of variables, mutability, and shadowing in Rust!