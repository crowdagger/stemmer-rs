extern crate gcc;

fn main() {
    gcc::Config::new()
        .include("libstemmer_c/include/")
        .file("libstemmer_c/src_c/stem_UTF_8_danish.c")
        .file("libstemmer_c/src_c/stem_UTF_8_dutch.c")
        .file("libstemmer_c/src_c/stem_UTF_8_english.c")
        .file("libstemmer_c/src_c/stem_UTF_8_finnish.c")
        .file("libstemmer_c/src_c/stem_UTF_8_french.c")
        .file("libstemmer_c/src_c/stem_UTF_8_german.c")
        .file("libstemmer_c/src_c/stem_UTF_8_hungarian.c")
        .file("libstemmer_c/src_c/stem_UTF_8_italian.c")
        .file("libstemmer_c/src_c/stem_UTF_8_norwegian.c")
        .file("libstemmer_c/src_c/stem_UTF_8_porter.c")
        .file("libstemmer_c/src_c/stem_UTF_8_portuguese.c")
        .file("libstemmer_c/src_c/stem_UTF_8_romanian.c")
        .file("libstemmer_c/src_c/stem_UTF_8_russian.c")
        .file("libstemmer_c/src_c/stem_UTF_8_spanish.c")
        .file("libstemmer_c/src_c/stem_UTF_8_swedish.c")
        .file("libstemmer_c/src_c/stem_UTF_8_turkish.c")
        .file("libstemmer_c/runtime/api.c")
        .file("libstemmer_c/runtime/utilities.c")
        .file("libstemmer_c/libstemmer/libstemmer_utf8.c")
        .compile("libstemmer.a");
}
    
