extern crate libc;
use libc::*;

#[link(name = "stemmer", kind="static")]
extern {
    fn sb_stemmer_list () -> *const *const c_char;
}

#[test]
fn it_works() {
    unsafe {
        println!("miaou");
        println!("{:?}", sb_stemmer_list());
    }
}
