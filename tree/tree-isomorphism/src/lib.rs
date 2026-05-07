use centroid::centroids;
use modint_mersenne61::ModintMersenne61;

type Mint = ModintMersenne61;

pub fn is_isomorphic(tree_a: &[Vec<usize>], tree_b: &[Vec<usize>]) -> bool {
    if tree_a.len() != tree_b.len() {
        return false;
    }
    let ca = centroids(tree_a);
    let cb = centroids(tree_b);
    if ca.len() != cb.len() {
        return false;
    }
    let r: Vec<Mint> = (0..tree_a.len()).map(|_| Mint::rand()).collect();
    let ha1 = hash_d(tree_a, ca[0], &r)[ca[0]];
    let hb1 = hash_d(tree_b, cb[0], &r)[cb[0]];
    if ca.len() == 1 {
        return ha1 == hb1;
    }
    let ha2 = hash_d(tree_a, ca[1], &r)[ca[1]];
    let hb2 = hash_d(tree_b, cb[1], &r)[cb[1]];
    (ha1 == hb1 && ha2 == hb2) || (ha1 == hb2 && ha2 == hb1)
}

pub fn hash_d(tree: &[Vec<usize>], root: usize, r: &[Mint]) -> Vec<Mint> {
    let n = tree.len();
    let mut hash = vec![Mint::new(1); n];
    let mut depth = vec![0; n];
    let mut stack = vec![(root, None, false)];
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            stack.push((v, p, true));
            hash[v] = Mint::new(1);
            for &to in &tree[v] {
                if Some(to) != p {
                    depth[to] = depth[v] + 1;
                    stack.push((to, Some(v), false));
                }
            }
        } else {
            for &to in &tree[v] {
                if Some(to) != p {
                    hash[v] = hash[v] * (hash[to] + r[depth[to]]);
                }
            }
        }
    }
    hash
}

pub fn hash_h(tree: &[Vec<usize>], root: usize, r: &[Mint]) -> Vec<Mint> {
    let n = tree.len();
    let mut hash = vec![Mint::new(1); n];
    let mut height = vec![0; n];
    let mut stack = vec![(root, None, false)];
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            stack.push((v, p, true));
            hash[v] = Mint::new(1);
            height[v] = 0;
            for &to in &tree[v] {
                if Some(to) != p {
                    stack.push((to, Some(v), false));
                }
            }
        } else {
            for &to in &tree[v] {
                if Some(to) != p {
                    hash[v] = hash[v] * (hash[to] + r[height[to]]);
                    height[v] = height[v].max(height[to] + 1);
                }
            }
        }
    }
    hash
}
