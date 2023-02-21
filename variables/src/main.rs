fn main() {
    /*
        Example on how to concatenate variables
    */

    let mut full_name_owned: String = "".to_owned();
    let first_name_borrowed: &str = "Jonathan ";
    let last_name_borrowed: &str = "Dyallo";
    let age: i32 = 27;
    let date_of_birth: i32 = 1995;

    full_name_owned.push_str(first_name_borrowed);
    full_name_owned.push_str(last_name_borrowed);
    println!(
        "Fullname is {}, age {} and birth date {}",
        full_name_owned, age, date_of_birth
    );
}
