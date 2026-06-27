// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_12_A

use proconio::input;

use kruskal::minimum_spanning_tree;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }
    let mut edges = vec![];
    for i in 0..n {
        for j in i + 1..n {
            if a[i][j] != -1 {
                edges.push((i, j, a[i][j]));
            }
        }
    }

    println!("{}", minimum_spanning_tree(n, &mut edges).0);
}
