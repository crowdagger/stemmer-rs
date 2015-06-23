extern crate libc;
use libc::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;

#[repr(C,packed)]
struct SbStemmer;
type SbSymbol = u8;

#[derive(Debug)]
pub struct Stemmer {
    stemmer: *mut SbStemmer
}
    

#[link(name = "stemmer", kind="static")]
extern {
    fn sb_stemmer_list () -> *const *const c_char;
    fn sb_stemmer_new(algorithm:*const c_char, charenc: *const c_char) -> *mut SbStemmer;
    fn sb_stemmer_delete(stemmer:*mut SbStemmer);
    fn sb_stemmer_stem(stemmer:*mut SbStemmer,
                       word: *const c_char,
                       size:c_int) -> *const c_char;
}

impl Drop for Stemmer {
    fn drop(&mut self) {
        println!("dropping");
        unsafe {
            if !self.stemmer.is_null() {
                sb_stemmer_delete(self.stemmer);
            }
        }
    }
}

impl Stemmer {
    pub fn list() -> Vec<&'static str>
    {
        let mut i = 0;
        unsafe {
            let list:*const *const c_char = sb_stemmer_list();
            let mut res = vec!();
            loop {
                let string_ptr:*const c_char = *list.offset(i);
                if string_ptr.is_null() {
                    return res;
                } else {
                    let bytes:&[u8] = CStr::from_ptr(string_ptr).to_bytes();
                    let s:&str = str::from_utf8_unchecked(bytes);
                    res.push(s);
                    i += 1;
                }
            }
        }
    }

    
    pub fn new(algorithm: &str) -> Option<Stemmer> {
        let algo = CString::new(algorithm).unwrap();
        let enc = CString::new("UTF_8").unwrap();
        unsafe {
            let stemmer = sb_stemmer_new(algo.as_ptr(),enc.as_ptr());
            if stemmer.is_null() {
                return None;
            } else {            
                return Some(Stemmer{stemmer:stemmer});
            }
        }
    }

    pub fn stem(self, word: &str) -> String {
        unsafe {
            self.stem_unsafe(word).to_string()
        }
    }

    pub unsafe fn stem_unsafe (self, word: &str) -> &'static str {
        let word = CString::new(word).unwrap();
        let res = sb_stemmer_stem(self.stemmer,
                                  word.as_ptr(),
                                  word.to_bytes().len() as i32);
        let bytes:&[u8] = CStr::from_ptr(res).to_bytes();
        let s:&str = str::from_utf8_unchecked(bytes);
        return s;
    }
}

    
#[test]
fn it_works() {
}
