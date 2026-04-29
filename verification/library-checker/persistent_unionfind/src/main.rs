// verification-helper: PROBLEM https://judge.yosupo.jp/problem/persistent_unionfind

use proconio::input;

use union_find_with_rollback::UnionFindWithRollback;

query::define_query! {
    Query {
        0 => Query0(k: i64, x: usize, y: usize),
        1 => Query1(k: i64, x: usize, y: usize),
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [Query; q],
    }
    let mut graph = vec![vec![]; q + 1];
    for (i, &query) in queries.iter().enumerate() {
        match query {
            Query0(k, _, _) => graph[(k + 1) as usize].push((i + 1, query)),
            Query1(k, _, _) => graph[(k + 1) as usize].push((i + 1, query)),
        }
    }

    let mut uf = UnionFindWithRollback::new(n);
    let mut res = vec![0; q];
    dfs(0, &graph, &mut uf, &mut res);
    for (i, &query) in queries.iter().enumerate() {
        if matches!(query, Query1(_, _, _)) {
            println!("{}", res[i]);
        }
    }
}

fn dfs(v: usize, graph: &[Vec<(usize, Query)>], uf: &mut UnionFindWithRollback, res: &mut [usize]) {
    for &(i, query) in &graph[v] {
        match query {
            Query0(_, x, y) => {
                uf.merge(x, y);
                dfs(i, graph, uf, res);
                uf.rollback();
            }
            Query1(_, x, y) => {
                res[i - 1] = if uf.same(x, y) { 1 } else { 0 };
                dfs(i, graph, uf, res);
            }
        }
    }
}
