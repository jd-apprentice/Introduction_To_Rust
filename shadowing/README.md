# 🔤 Shadowing

https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html

If we remember about variables, they are inmutable by default so in order to change a value we had to use the `mut` keyword in order to make it mutable. But what about we want to keep the variable inmutable but modify his value? thats where Shadowing comes to play.

Once we defined a variable for example

```rust
// Remember inmutable by default
let myName: &str = "Jonathan";

println!("My name is: {}", myName); // Jonathan
```

If we want to change its value but don't add the `mut` keyword we should do something like this ->

```rust
let myName: &str = "Jonathan";

println!("My name is: {}", myName); // Jonathan

let myName: &str = "Gustavo"; // Here is where the shadowing occurs

println!("My name is: {}", myName); // Gustavo
```

As we can see in the example above we defined the variable with the same name two times, but what is exactly happening there?

You may think that the variable is changing its value but its not, is being destroyed and re-created with the new value, rust is smart enough to know that this variable exists and gets deleted.
