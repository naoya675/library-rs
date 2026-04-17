// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_B

use proconio::input;

use mincostflow::MinCostFlow;

fn main() {
    input! {
        v: usize,
        e: usize,
        f: i64,
        uvcd: [(usize, usize, i64, i64); e],
    }
    let mut mcf = MinCostFlow::new(v);
    for &(u, v, c, d) in &uvcd {
        mcf.add_edge(u, v, c, d);
    }

    let (max_flow, min_cost) = mcf.flow_with(0, v - 1, f);
    println!("{}", if max_flow < f { -1 } else { min_cost });
}
