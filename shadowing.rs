fn main() {
    let shadowing_binding = 1;

    {
        println!("before being shadowed: {}", shadowing_binding);

        let shadowing_binding = "abc";

        println!("shadowed in inner block: {}", shadowing_binding);
    }
    println!("outside inner block: {}", shadowing_binding);

    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}