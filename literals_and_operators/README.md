# 🔤 Literals and operators

- Rust also supports scientific [E-notation](https://en.wikipedia.org/wiki/Scientific_notation#E_notation), e.g. 1e6, 7.6e-4. The associated type is f64.
- Operator precedence is like [C-like languages](https://en.wikipedia.org/wiki/Order_of_operations#Programming_languages).

## 📚 Numeric operations

We can do simple operations with numbers, like addition, subtraction, multiplication, division, and so on.

```rust
fn main() {
    let a = 2 + 3 * 4;
    println!("a = {}", a);
}
```

## 📚 Boolean operations

We can also do operations with booleans, like `and`, `or`, and `not`.

```rust
fn main() {
    let a = true;
    let b = false;
    println!("a and b = {}", a && b);
    println!("a or b = {}", a || b);
    println!("not a = {}", !a);
}
```

This will print `a = 14`.

## 📚 Literals

Literals are values that are hard-coded into the program. They are immutable by default.

### 📚 Integer literals

Integer literals are numbers without a decimal point. They are of type `i32` by default.

```rust
fn main() {
    let a = 42;
    println!("a = {}", a);
}
```

### 📚 Floating-point literals

Floating-point literals are numbers with a decimal point. They are of type `f64` by default.

```rust
fn main() {
    let a = 42.0;
    println!("a = {}", a);
}
```
