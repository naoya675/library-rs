pub fn centroids(tree: &[Vec<usize>]) -> Vec<usize> {
    let n = tree.len();
    let mut res = vec![];
    let mut siz = vec![1; n];
    let mut par = vec![None; n];
    let mut stack = vec![(0, None, false)];
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            stack.push((v, p, true));
            par[v] = p;
            for &to in &tree[v] {
                if Some(to) != p {
                    stack.push((to, Some(v), false));
                }
            }
        } else {
            let mut max = 0;
            for &to in &tree[v] {
                if Some(to) != p {
                    siz[v] += siz[to];
                    max = max.max(siz[to]);
                }
            }
            if max.max(n - siz[v]) <= n / 2 {
                res.push(v);
            }
        }
    }
    res
}

pub fn centroids_dead(tree: &[Vec<usize>], root: usize, dead: &[bool]) -> Vec<usize> {
    let n = tree.len();
    let mut comp = vec![];
    let mut res = vec![];
    let mut siz = vec![1; n];
    let mut par = vec![None; n];
    let mut stack = vec![(root, None, false)];
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            stack.push((v, p, true));
            par[v] = p;
            comp.push(v);
            for &to in &tree[v] {
                if !dead[to] && Some(to) != p {
                    stack.push((to, Some(v), false));
                }
            }
        } else {
            for &to in &tree[v] {
                if !dead[to] && Some(to) != par[v] {
                    siz[v] += siz[to];
                }
            }
        }
    }
    for &v in &comp {
        let mut max = 0;
        for &to in &tree[v] {
            if !dead[to] && Some(to) != par[v] {
                max = max.max(siz[to]);
            }
        }
        if max.max(siz[root] - siz[v]) <= siz[root] / 2 {
            res.push(v);
        }
    }
    res
}
