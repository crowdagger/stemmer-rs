// ! Stemmer library
//
// Wraps Snowball's C stemming library so it can be used safely in Rust.
//
// The two most important methods it provides are the following:
//
// `new`, which creates a stemmer for the given language;
// `stem`, which returns a `String` containing the stem of a word.
//
// # Examples
//
// ```
// use stemmer::Stemmer;
// let mut stemmer = Stemmer::new("english").unwrap();
// assert_eq!(stemmer.stem("rusted"), stemmer.stem("rust"));
// ```
//
// Note that (since version 0.3) `stem` requires a mutable reference to a
// `Stemmer`. This is because there is another method:
//
// `stem_str`, which returns a `&str` instead of a `String`.
//
// This method is less costly, but the reference it returns borrows the
// `stemmer`, so the compiler won't let you call `stem` or `stem_str` as long
// as it is in scope:
//
// ```
// use stemmer::Stemmer;
// let mut stemmer = Stemmer::new("english").unwrap();
// let stemmed: &str = stemmer.stem_str("foo");
// stemmer.stem_str("bar"); // the compiler won't let you do this
// ```
//
//
// The last useful method is:
//
// `list`, which returns a `Vector` containing all implemented languages.
//
// ```
// use stemmer::Stemmer;
// for language in Stemmer::list() {
// println!("{}", language);
// }
// ```
//

mod stemmer;
#[cfg(test)]
mod test;

pub use stemmer::Stemmer;
