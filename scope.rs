fn main() {
    let long_live_binding = 1;

    {
        let short_live_binding = 2;
        println!("inner short: {}", short_live_binding);
    }

    //println!("outer short: {}", short_live_binding);

    println!("outer long: {}", long_live_binding);
}