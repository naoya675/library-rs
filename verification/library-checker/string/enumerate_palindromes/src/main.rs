// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes

use fast_io::{Output, input};

use manacher::manacher;

fn main() {
    input! {
        s: Chars,
    }
    let mut out = Output::new();

    let s = s.into_iter().flat_map(|c| ['#', c]).skip(1).collect::<Vec<_>>();

    out.println_iter(manacher(&s).iter().enumerate().map(|(i, &k)| k - 1 + ((i ^ k) & 1)), " ");
}
