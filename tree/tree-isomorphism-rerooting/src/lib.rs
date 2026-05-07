use modint_mersenne61::ModintMersenne61;
use rerooting::Rerooting;

pub fn is_isomorphic(tree_a: &[Vec<usize>], tree_b: &[Vec<usize>]) -> bool {
    if tree_a.len() != tree_b.len() {
        return false;
    }
    let r: Vec<ModintMersenne61> = (0..tree_a.len()).map(|_| ModintMersenne61::rand()).collect();
    let mut ha: Vec<ModintMersenne61> = hash_h(tree_a, &r).into_iter().map(|(h, _)| h).collect();
    let mut hb: Vec<ModintMersenne61> = hash_h(tree_b, &r).into_iter().map(|(h, _)| h).collect();
    ha.sort_by_key(|&h| h.value());
    hb.sort_by_key(|&h| h.value());
    ha == hb
}

pub fn hash_h(tree: &[Vec<usize>], r: &[ModintMersenne61]) -> Vec<(ModintMersenne61, usize)> {
    let n = tree.len();

    type Cost = ();
    type Data = (ModintMersenne61, usize);

    let mut g = Rerooting::<Cost, Data, _, _, _, _>::new(
        n,
        |x, y| (x.0 * y.0, std::cmp::max(x.1, y.1)),
        || (ModintMersenne61::new(1), 0),
        |_| (ModintMersenne61::new(1), 0),
        |x, _, _, _| (x.0 + r[x.1], x.1 + 1),
    );
    for u in 0..n {
        for &v in &tree[u] {
            g.add_edge(u, v, ());
        }
    }
    g.run()
}

pub fn hash(tree: &[Vec<usize>], r: ModintMersenne61) -> Vec<ModintMersenne61> {
    let n = tree.len();

    type Cost = ();
    type Data = ModintMersenne61;

    let mut g = Rerooting::<Cost, Data, _, _, _, _>::new(n, |x, y| x * y, || ModintMersenne61::new(1), |_| ModintMersenne61::new(1), |x, _, _, _| x + r);
    for u in 0..n {
        for &v in &tree[u] {
            g.add_edge(u, v, ());
        }
    }
    g.run()
}
