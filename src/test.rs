use Stemmer;

#[test]
fn unknown_algorithm() {
    let stemmer = Stemmer::new("foo");
    assert_eq!(stemmer.is_none(), true);
}

#[test]
fn known_algorithm() {
    let stemmer = Stemmer::new("english");
    assert_eq!(stemmer.is_some(), true);
}

#[test]
fn stem_french() {
    let mut stemmer = Stemmer::new("french").unwrap();
    assert_eq!(stemmer.stem("mangera"), stemmer.stem("mangé"));
}

#[test]
fn stem_english() {
    let mut stemmer = Stemmer::new("english").unwrap();
    assert_eq!(stemmer.stem("borrowing"), stemmer.stem("borrowed"));
}

#[test]
fn stem_english_multiple() {
    let mut stemmer = Stemmer::new("english").unwrap();
    assert_eq!(stemmer.stem("borrowing"), stemmer.stem("borrowed"));
    assert_eq!(stemmer.stem("following"), stemmer.stem("followed"));
}

#[test]
fn stem_str_french() {
    let mut stemmer = Stemmer::new("french").unwrap();
    assert_eq!("éternu", stemmer.stem_str("éternuerai"));
}

#[test]
fn stem_str_multiple() {
    let mut stemmer = Stemmer::new("french").unwrap();
    let strings = vec!("foo", "bar", "baz");
    for s in strings {
        stemmer.stem_str(s); //no assert, we just check that it compiles
    }
}

// The following test must NOT compile. But I don't know how to tell Rust that :(
// #[test]
// fn stem_str_wrong() {
//     let mut stemmer = Stemmer::new("french").unwrap();
//     let s1 = stemmer.stem_str("foo");
//     let s2 = stemmer.stem_str("bar"); //should not compile: s1 is no more valid
//     println!("{}{}", s1, s2); //very wrong if it executes: undefined behaviour
// }
