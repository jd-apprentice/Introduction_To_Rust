// Derive + Debug allow us to print the struct in a pretty way
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
    birth_date: &'a str,
}

fn main() {
    let name: &str = "Jonathan Dyallo";
    let age: i32 = 27;
    let birth_date: &str = "1995-09-28";
    let jonathan: Person = Person {
        name,
        age,
        birth_date,
    };

    // Pretty print
    println!("{:#?}", jonathan);
}
