# 🔤 Variables

Variables are defined in several ways which the most basic example could be ->

```rs
// <reserved-word> <variable-name>: <T> = <value>

// https://doc.rust-lang.org/reference/variables.html

let first_name: &str = "Jonathan";
let last_name: &str = "Dyallo";
```

For numbers could be something like this ->

```rs
let age: i32 = 27;
let date_of_birth: i32 = 1995;
```

If we want the variable to be mutable (because all the variables are inmutable by default) we have to add the `mut` keyword after the `reserved-word`

```rs
let full_name: &str = "Jonathan Dyallo";
let mut mutable_full_name: &str = "Jonathan Gustavo";
let inmutable_full_name: &str = "Jonathan Martin";

mutable_full_name = "Jonathan Gustavo Dyallo";
inmutable_full_name = "Jonathan Martin Dyallo"; // Variable is inmutable

println!("Name is: {}", mutable_full_name) // Jonathan Gustavo Dyallo
```

To keep a variable inmutable and change his value we need to apply the concept of shadowing.
