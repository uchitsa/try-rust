fn main() {
    let mut _mut_int = 7i32;

    {
        let _mut_int = _mut_int;

        //_mut_int = 50; //does not work
    }

    _mut_int = 3;
}