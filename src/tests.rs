#[path = "mcount.rs"]
mod mcount;
use crate::mcount::*;

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn test2() {
        let hello_str = "hello";

        println!("{} len is {} ", hello_str, crate::mcount::my_mcount(hello_str));
    }
}