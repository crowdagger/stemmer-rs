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
    let stemmer = Stemmer::new("french").unwrap();
    assert_eq!(stemmer.stem("mangera"), stemmer.stem("mangé"));
}

#[test]
fn stem_english() {
    let stemmer = Stemmer::new("english").unwrap();
    assert_eq!(stemmer.stem("borrowing"), stemmer.stem("borrowed"));
}
