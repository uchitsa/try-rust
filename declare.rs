fn main() {
    let a_bind;
    {
        lext x = 2;
        a_bind = x * x;   
    }      
    println!("bind a: {}", a_bind);

    let another_bind;
    //println!("another bind: {}", another_bind); //does not work

    another_bind = 1;
    println!("another bind: {}", another_bind);
}