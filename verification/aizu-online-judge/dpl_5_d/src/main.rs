// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_5_D

use proconio::input;

use mod_combinatorial::ModCombinatorial;

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mc = ModCombinatorial::<MOD>::new(n + k);
    println!("{}", mc.homo(k, n));
}
