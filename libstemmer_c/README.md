libstemmer_c
============

These files come from from the C version of the libstemmer
distribution (written by Martin Porter), available for download from:

http://snowball.tartarus.org/dist/libstemmer_c.tgz

Since the Rust bindings only use the UTF8 implementations, other char
encodings have been dropped; similarly, Makefile and other
build-related files have been removed since `build.rs` takes care of
that. So if you are interested in stemming in C, go check this library instead.

