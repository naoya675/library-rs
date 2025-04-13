// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D

use proconio::input;

use mod_combinatorial::ModCombinatorial;

type Mcomb = ModCombinatorial<1000000007>;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mc = Mcomb::new(n + k);
    println!("{}", mc.homo(k, n));
}
