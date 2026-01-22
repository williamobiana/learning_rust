fn main() {
    let person = ("Alice", 30, true);
    println!("Name: {}, Age: {}, Employed: {}", person.0, person.1, person.2);
    let (name, age, employed) = person;
    println!("Name: {}, Age: {}, Employed: {}", name, age, employed);
}