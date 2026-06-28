// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use std::collections::VecDeque;

use proconio::input;

use doubling::Doubling;
use level_ancestor_offline::level_ancestor_offline;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        sti: [(usize, usize, usize); q],
    }
    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut db = Doubling::<usize>::new(n, n, |x, y| x + y, 0);
    let mut f = vec![0; n];
    let mut depth = vec![0; n];
    let mut queue = VecDeque::from([(0, None)]);
    while let Some((v, par)) = queue.pop_front() {
        for &to in &graph[v] {
            if Some(to) != par {
                f[to] = v;
                depth[to] = depth[v] + 1;
                queue.push_back((to, Some(v)));
            }
        }
    }
    db.doubling(&f, &vec![0; n]);

    let log = (n.next_power_of_two().ilog2() + 1) as usize;
    let mut queries = vec![];
    for &(s, t, i) in &sti {
        let l = lca(&db, &depth, log, s, t);
        let d = depth[s] + depth[t] - 2 * depth[l];
        if i > d {
            queries.push((0, n));
        } else if i <= depth[s] - depth[l] {
            queries.push((s, i));
        } else {
            queries.push((t, d - i));
        }
    }
    let res = level_ancestor_offline(&graph, 0, &queries);

    for i in 0..q {
        if let Some(v) = res[i] {
            println!("{}", v);
        } else {
            println!("-1");
        }
    }
}

fn lca(db: &Doubling<usize>, depth: &[usize], log: usize, mut u: usize, mut v: usize) -> usize {
    if depth[u] < depth[v] {
        std::mem::swap(&mut u, &mut v);
    }
    u = db.kth(u, depth[u] - depth[v]).0;
    if u == v {
        return u;
    }
    for k in (0..log).rev() {
        let nu = db.kth(u, 1 << k).0;
        let nv = db.kth(v, 1 << k).0;
        if nu != nv {
            u = nu;
            v = nv;
        }
    }
    db.kth(u, 1).0
}
