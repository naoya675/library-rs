// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_C

use proconio::input;

use strongly_connected_components_tarjan::StronglyConnectedComponents;
use union_find::UnionFind;

fn main() {
    input! {
        v: usize,
        e: usize,
        st: [(usize, usize); e],
        q: usize,
        uv: [(usize, usize); q],
    }
    let mut scc = StronglyConnectedComponents::new(v);
    st.iter().for_each(|&(s, t)| scc.add_edge(s, t));
    let groups = scc.scc();

    let mut uf = UnionFind::new(v);
    for group in groups {
        for i in 0..group.len() {
            uf.merge(group[0], group[i]);
        }
    }

    for &(u, v) in &uv {
        println!("{}", if uf.same(u, v) { 1 } else { 0 });
    }
}
