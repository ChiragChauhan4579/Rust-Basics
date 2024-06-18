fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
    let some_string = change(&mut s1);
    println!("{some_string}");

    let len2 = calculate_length(&s1);
    println!("The length of '{s1}' is {len2}.")


}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) -> &mut String{
    some_string.push_str(", world");
    some_string
}
