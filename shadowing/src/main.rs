fn main() {
    let age: i32 = 27; // Inmutable

    println!("The age is: {}", age); // 27

    /* Here age gets destroyed and re-created it with the new value
    its not actually chaning is being re-created
     */
    let age: i32 = 42; // Shadowing
    println!("The age is: {}", age); // 42
}
