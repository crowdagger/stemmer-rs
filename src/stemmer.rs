extern crate libc;
use self::libc::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::str;

pub struct Stemmer {
    stemmer: *mut c_void
}

extern {
    fn sb_stemmer_list() -> *const *const c_char;
    fn sb_stemmer_new(algorithm: *const c_char, charenc: *const c_char) -> *mut c_void;
    fn sb_stemmer_delete(stemmer: *mut c_void);
    fn sb_stemmer_stem(stemmer: *mut c_void,
                       word: *const c_char,
                       size:c_int) -> *const c_char;
}

impl Drop for Stemmer {
    fn drop(&mut self) {
        unsafe {
            if !self.stemmer.is_null() {
                sb_stemmer_delete(self.stemmer);
            }
        }
    }
}

impl Stemmer {
    /// Lists all existing algorithms, returning a vector of valid algorithms
    /// that can be used as argument for `Stemmer::new`
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

    /// Creates a new stemmer, provided `algorithm` is a valid one.
    ///
    /// # Arguments
    ///
    /// * `algorithm`: the name of a stemming algorithm. A list of supported
    /// algorithms can be obtained with `Stemmer::list()`
    ///
    /// # Returns
    ///
    /// * `Some(Stemmer)` if `algorithm` exists;
    /// * `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stemmer::Stemmer;
    /// let stemmer = Stemmer::new("english");
    /// assert!(stemmer.is_some());
    /// ```
    ///
    /// ```rust
    /// use stemmer::Stemmer;
    /// let stemmer = Stemmer::new("foobar");
    /// assert!(stemmer.is_none());
    /// ```
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

    /// Stems the `word`.
    ///
    /// Returns an owned string containing the stemmed version of the word.
    ///
    /// # Example
    ///
    /// ```
    /// use stemmer::Stemmer;
    /// let mut stemmer = Stemmer::new("french").unwrap();
    /// assert_eq!("éternu", &stemmer.stem("éternuerai"));
    /// ```
    pub fn stem(&mut self, word: &str) -> String {
        String::from(self.stem_str(word))
    }

    /// Stems the word and returns a str
    ///
    /// The `str` reference it returns is only valid as long as you don't
    /// call `stem` or stem_str` again; thus, Rust's borrowchecker won't let
    /// call one of them function if you have such a reference in scope.
    ///
    /// # Example
    ///
    /// ```
    /// use stemmer::Stemmer;
    /// let mut stemmer = Stemmer::new("english").unwrap();
    /// println!("{}", stemmer.stem_str("foo"));
    /// println!("{}", stemmer.stem_str("bar")); // ok
    /// println!("{}", stemmer.stem("baz")); // ok too
    /// let foo: &str = stemmer.stem_str("foo");
    /// // stemmer.stem("bar"); -> Compiler error because `stemmer` is already borrowed by `foo`
    /// ```
    pub fn stem_str (&mut self, word: &str) -> &str {
        unsafe {
            let word = CString::new(word).unwrap();
            let res = sb_stemmer_stem(self.stemmer,
                                      word.as_ptr(),
                                      word.to_bytes().len() as i32);
            let bytes:&[u8] = CStr::from_ptr(res).to_bytes();
            str::from_utf8_unchecked(bytes)
        }
    }
}
