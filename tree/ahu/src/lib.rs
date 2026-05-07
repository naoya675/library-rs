use std::collections::HashMap;

use centroid::centroids;

pub fn is_isomorphic(tree_a: &[Vec<usize>], tree_b: &[Vec<usize>]) -> bool {
    if tree_a.len() != tree_b.len() {
        return false;
    }
    let ca = centroids(tree_a);
    let cb = centroids(tree_b);
    if ca.len() != cb.len() {
        return false;
    }
    let mut canonical: HashMap<Vec<usize>, usize> = HashMap::new();
    let la1 = labels(tree_a, ca[0], &mut canonical)[ca[0]];
    let lb1 = labels(tree_b, cb[0], &mut canonical)[cb[0]];
    if ca.len() == 1 {
        return la1 == lb1;
    }
    let la2 = labels(tree_a, ca[1], &mut canonical)[ca[1]];
    let lb2 = labels(tree_b, cb[1], &mut canonical)[cb[1]];
    (la1 == lb1 && la2 == lb2) || (la1 == lb2 && la2 == lb1)
}

pub fn labels(tree: &[Vec<usize>], root: usize, canonical: &mut HashMap<Vec<usize>, usize>) -> Vec<usize> {
    let n = tree.len();
    let mut label = vec![0; n];
    let mut stack = vec![(root, None, false)];
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            stack.push((v, p, true));
            for &to in &tree[v] {
                if Some(to) != p {
                    stack.push((to, Some(v), false));
                }
            }
        } else {
            let mut key: Vec<usize> = tree[v].iter().filter(|&&to| Some(to) != p).map(|&to| label[to]).collect();
            key.sort_unstable();
            let next = canonical.len();
            label[v] = *canonical.entry(key).or_insert(next);
        }
    }
    label
}
