// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primitive_root

use fast_io::{Output, input, output};

use primitive_root::primitive_root;

fn main() {
    input! {
        q: usize,
        p: [u64; q],
    }
    let mut out = Output::new();

    for p in p {
        output!(out, primitive_root(p));
    }
}
