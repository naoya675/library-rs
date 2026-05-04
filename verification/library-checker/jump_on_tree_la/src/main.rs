// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use std::collections::VecDeque;

use proconio::input;

use doubling::Doubling;
use level_ancestor::LevelAncestor;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        sti: [(usize, usize, usize); q],
    }
    let mut graph = vec![vec![]; n];
    let mut la = LevelAncestor::new(n);
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
        la.add_edge(a, b);
        la.add_edge(b, a);
    }
    la.init(0);

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
    for &(s, t, i) in &sti {
        let l = lca(&db, &depth, log, s, t);
        let d = depth[s] + depth[t] - 2 * depth[l];
        if i > d {
            println!("-1");
            continue;
        }
        if i <= depth[s] - depth[l] {
            println!("{}", la.la(s, i).unwrap());
        } else {
            println!("{}", la.la(t, d - i).unwrap());
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
