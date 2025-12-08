// verification-helper: [EXCLUSION] https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_4_B

use itertools::Itertools;
use proconio::input;

use topological_sort::topological_sort;

fn main() {
    input! {
        v: usize,
        e: usize,
        st: [(usize, usize); e],
    }
    let mut graph = vec![vec![]; v];
    st.iter().for_each(|&(s, t)| graph[s].push(t));

    println!("{}", topological_sort(v, &graph).iter().join("\n"));
}
