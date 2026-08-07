// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_graph_vertex_add_component_sum

use fast_io::{Output, define_query, input, output};

use offline_dynamic_connectivity::OfflineDynamicConnectivity;
use union_find_with_rollback_abstract::UnionFindWithRollbackAbstract;

define_query! {
    Query {
        0 => Query0(u: usize, v: usize),
        1 => Query1(u: usize, v: usize),
        2 => Query2(v: usize, x: i64),
        3 => Query3(v: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        queries: [Query; q],
    }
    let mut out = Output::new();

    let mut dc = OfflineDynamicConnectivity::new(n, q);
    for (t, query) in queries.iter().enumerate() {
        match *query {
            Query0(u, v) => dc.insert(t, u, v),
            Query1(u, v) => dc.remove(t, u, v),
            _ => {}
        }
    }

    let mut uf = UnionFindWithRollbackAbstract::from_slice(&a, |x, y| x + y, 0, |x| -x);
    let mut res = vec![0; q];
    dc.run(
        &mut uf,
        |uf, u, v| {
            uf.merge(u, v);
        },
        |uf| uf.rollback(),
        |uf, t| match queries[t] {
            Query2(v, x) => uf.apply(v, x),
            Query3(v) => res[t] = uf.prod(v),
            _ => {}
        },
    );

    for (t, query) in queries.iter().enumerate() {
        if matches!(query, Query3(_)) {
            output!(out, res[t]);
        }
    }
}
