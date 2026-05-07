use modint_mersenne61::ModintMersenne61;
use rerooting::Rerooting;

type Mint = ModintMersenne61;

pub fn is_isomorphic(tree_a: &[Vec<usize>], tree_b: &[Vec<usize>]) -> bool {
    if tree_a.len() != tree_b.len() {
        return false;
    }
    let r: Vec<Mint> = (0..tree_a.len()).map(|_| Mint::rand()).collect();
    let mut ha: Vec<Mint> = hash_h(tree_a, &r).into_iter().map(|(h, _)| h).collect();
    let mut hb: Vec<Mint> = hash_h(tree_b, &r).into_iter().map(|(h, _)| h).collect();
    ha.sort_by_key(|&h| h.value());
    hb.sort_by_key(|&h| h.value());
    ha == hb
}

pub fn hash_h(tree: &[Vec<usize>], r: &[Mint]) -> Vec<(Mint, usize)> {
    let n = tree.len();
    let mut g = Rerooting::<(), (Mint, usize), _, _, _, _>::new(
        n,
        |x: (Mint, usize), y: (Mint, usize)| (x.0 * y.0, x.1.max(y.1)),
        || (Mint::new(1), 0),
        |_| (Mint::new(1), 0),
        |x: (Mint, usize), _, _, _| (r[x.1] + x.0, x.1 + 1),
    );
    for u in 0..n {
        for &v in &tree[u] {
            g.add_edge(u, v, ());
        }
    }
    g.run()
}

pub fn hash(tree: &[Vec<usize>], r: Mint) -> Vec<Mint> {
    let n = tree.len();
    let mut g = Rerooting::<(), Mint, _, _, _, _>::new(n, |x, y| x * y, || Mint::new(1), |_| Mint::new(1), |x, _, _, _| x + r);
    for u in 0..n {
        for &v in &tree[u] {
            g.add_edge(u, v, ());
        }
    }
    g.run()
}
