ChangeLog
=========

0.3.2 (2015-12-10)
------------------
* `Cargo.toml`: fixed documentation and homepage links

0.3.1 (2015-11-12)
------------------
* updated `libc` dependency to 0.2
* fixed `Cargo.toml` dependencies version (removed bad "*"!) 

0.3.0 (2015-10-02)
------------------
* `stem` now takes `&mut self` instead of `&self`
* `stem_unsafe` renamed to `stem_str` since it is no more unsafe:
  borrowchecker can verify that `stem` is not run again while there is
  a reference to the returned `&str`
* added more tests

0.2.0 (2015-08-09)
------------------
* Completed the documentation
* Only compile `test.rs` when `cargo test` is runned
* Removed unnecessary use of `#[repr(packed)]
* Changed signature of `stem_unsafe`, which is now actually safe to
  call but returns an unsafe to use pointer 

0.1.1 (2015-06-23)
------------------
* Removed a silly `println`

0.1.0 (2015-06-23)
------------------
* Initial release
