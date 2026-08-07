// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize

use fast_io::{Output, input};

use pollard_rho::factors_dup;

fn main() {
    input! {
        q: usize,
        a: [u64; q],
    }
    let mut out = Output::new();

    for a in a {
        let res = factors_dup(a);
        out.print(res.len());
        out.print(" ");
        out.println_iter(&res, " ");
    }
}
