// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_5_D

use proconio::input;

use fenwick_tree::FenwickTree;
use lower_bound::LowerBound;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut x = a.clone();
    x.sort();
    let mut ft = FenwickTree::<i64>::new(n);
    let mut res = 0;

    for i in 0..n {
        let p = x.lower_bound(&a[i]);
        res += ft.sum(p + 1, n);
        ft.add(p, 1);
    }

    println!("{}", res);
}
