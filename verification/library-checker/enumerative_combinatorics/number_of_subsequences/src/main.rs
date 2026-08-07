// verification-helper: PROBLEM https://judge.yosupo.jp/problem/number_of_subsequences

// Reference:
// - https://noshi91.hatenablog.com/entry/2023/02/26/135340
// - https://atcoder.jp/contests/abc446/editorial/16429

use fast_io::{Output, input, output};

use lower_bound::LowerBound;
use modint::Modint;

type Mint = Modint<998244353>;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut out = Output::new();

    let mut sorted = a.clone();
    sorted.sort();
    sorted.dedup();
    let a: Vec<usize> = a.iter().map(|v| sorted.lower_bound(v)).collect();

    let mut dp = vec![Mint::new(0); a.len()];
    let mut sum = Mint::new(1);
    for i in 0..n {
        let tmp = dp[a[i]];
        dp[a[i]] = sum;
        sum += sum - tmp;
    }

    output!(out, (sum - Mint::new(1)).value());
}
