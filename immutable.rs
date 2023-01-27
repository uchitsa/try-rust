fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before changes: {}", mutable_binding);

    mutable_binding += 1;

    println!("After changes: {}", mutable_binding);

    // _immutable_binding += 1;

    println!("Immutable: {}", _immutable_binding);
}