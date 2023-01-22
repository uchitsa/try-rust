#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of mondey!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldier fight!"),
    }
}