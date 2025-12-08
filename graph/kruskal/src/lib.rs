use union_find::UnionFind;

pub fn minimum_spanning_tree(size: usize, edge: &mut Vec<(usize, usize, i64)>) -> (i64, Vec<(usize, usize, i64)>) {
    edge.sort_by(|a, b| a.2.cmp(&b.2));
    let mut uf = UnionFind::new(size);
    let mut res = 0;
    let mut res_edge = vec![];
    for &mut (from, to, cost) in edge {
        if uf.same(from, to) {
            continue;
        }
        uf.merge(from, to);
        res += cost;
        res_edge.push((from, to, cost));
    }
    (res, res_edge)
}

pub fn maximum_spanning_tree(size: usize, edge: &mut Vec<(usize, usize, i64)>) -> (i64, Vec<(usize, usize, i64)>) {
    edge.sort_by(|a, b| b.2.cmp(&a.2));
    let mut uf = UnionFind::new(size);
    let mut res = 0;
    let mut res_edge = vec![];
    for &mut (from, to, cost) in edge {
        if uf.same(from, to) {
            continue;
        }
        uf.merge(from, to);
        res += cost;
        res_edge.push((from, to, cost));
    }
    (res, res_edge)
}
