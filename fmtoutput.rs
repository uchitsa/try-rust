fn main() {
    println!("{} days", 31);

    println!("{0} this is {1}. {1} this is {0}", "Alice", "Bob");

    println!("{} from {:b} people knows about binary code", 1, 2);

    #[allow(dead_code)]
    struct Structure(i32);

    //println!("This struct {:?} does want to print...", Structure(3));
}
