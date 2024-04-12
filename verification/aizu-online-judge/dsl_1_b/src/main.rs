// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_B

use proconio::input;

use weighted_union_find::WeightedUnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut wuf = WeightedUnionFind::<i64>::new(n);
    for _ in 0..q {
        input! { query: usize, }
        match query {
            0 => {
                input! { x: usize, y: usize, z: i64, }
                wuf.merge(x, y, z);
            }
            1 => {
                input! { x: usize, y: usize, }
                if wuf.same(x, y) {
                    println!("{}", wuf.diff(x, y));
                } else {
                    println!("?");
                }
            }
            _ => unreachable!(),
        }
    }
}
