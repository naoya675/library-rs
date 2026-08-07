// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use fast_io::{Output, input};

use suffix_array::suffix_array;

fn main() {
    input! {
        s: Chars,
    }
    let mut out = Output::new();

    let sa = suffix_array(&s);

    out.println_iter(&sa, " ");
}
