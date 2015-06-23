extern crate stemmer;
use stemmer::Stemmer;


fn main() {
    println!("miaou");
    println!("{:?}", Stemmer::list());
    let stemmer = Stemmer::new("french");
    println!("{:?}", stemmer);
    let stemmer = stemmer.unwrap();
    println!("{}", stemmer.stem("éternuerai"));
}
