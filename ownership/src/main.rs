fn calculate_length(s: &String) -> usize {
    s.len()  // Can read but not modify
}

fn modify_string(s: &mut String) {
    s.push_str(" modified");
}

fn main() {
    let mut text = String::from("Hello");
    let len = calculate_length(&text);
    modify_string(&mut text);
    
    println!("The length of '{}' is {}.", text, len);
}