// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_B

use fast_io::{Output, input, output};

use mincostflow::MinCostFlow;

fn main() {
    input! {
        v: usize,
        e: usize,
        f: i64,
        uvcd: [(usize, usize, i64, i64); e],
    }
    let mut out = Output::new();

    let mut mcf = MinCostFlow::new(v);
    for (u, v, c, d) in uvcd {
        mcf.add_edge(u, v, c, d);
    }

    let (max_flow, min_cost) = mcf.flow_with(0, v - 1, f);

    output!(out, if max_flow < f { -1 } else { min_cost });
}
