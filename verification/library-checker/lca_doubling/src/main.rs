// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use proconio::input;

use doubling::Doubling;

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n - 1],
        uv: [(usize, usize); q],
    }
    let mut db = Doubling::<usize>::new(n, n, |x, y| x + y, 0);
    let mut f = vec![0; n];
    let mut g = vec![0; n];
    let mut d = vec![0; n];
    f[0] = 0;
    for (i, &p) in p.iter().enumerate() {
        f[i + 1] = p;
        d[i + 1] = d[p] + 1;
    }
    db.doubling(&f, &g);

    let log = (n.next_power_of_two().ilog2() + 1) as usize;
    for (u, v) in uv {
        println!("{}", lca(&db, &d, log, u, v));
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
