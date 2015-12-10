# stemmer-rs

Stemming library for Rust, using bindings to Snowball C implementation (see http://snowball.tartarus.org/)

## Usage

* `Stemmer::list()` returns a vector with all possible algorithms that
can be used.
* `Stemmer::new(algorithm)` creates a new Stemmer. Note that it
returns an option, as it can fail if `algorithm` is not a valid one.
* Once you have a (mutable) `Stemmer`, `stemmer.stem(word)` stems a
  word. 

Note that the C Stemming library (or at least the part that is used by
the Rust bindings) is included and statically linked by the Rust
bindings, so you don't have any other libraries to install when you
use this Rust library.

## Documentation

See the
[documentation](http://lise-henry.github.io/rust/stemmer/index.html)
for this library.

## Credits

These are just Rust bindings for the Snowball C stemming library,
written by Martin Porter. See http://snowball.tartarus.org/index.php
for the Snowball project and
http://snowball.tartarus.org/dist/libstemmer_c.tgz for the C version.

## License

The Rust bindings, as well as the original C implementation, is
covered by the BSD license
(http://opensource.org/licenses/bsd-license.html). 
