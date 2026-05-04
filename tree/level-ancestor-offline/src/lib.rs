pub fn level_ancestor_offline(graph: &[Vec<usize>], root: usize, queries: &[(usize, usize)]) -> Vec<Option<usize>> {
    let n = graph.len();
    let q = queries.len();
    let mut vtoq = vec![vec![]; n];
    for (i, &(v, k)) in queries.iter().enumerate() {
        vtoq[v].push((i, k));
    }
    let mut res = vec![None; q];
    let mut path = vec![];
    let mut stack = vec![];
    path.reserve(n);
    stack.reserve(n);
    stack.push((root, None, false));
    while let Some((v, par, visited)) = stack.pop() {
        if !visited {
            stack.push((v, par, true));
            path.push(v);
            for &(i, k) in &vtoq[v] {
                if k < path.len() {
                    res[i] = Some(path[path.len() - 1 - k]);
                }
            }
            for &to in &graph[v] {
                if Some(to) != par {
                    stack.push((to, Some(v), false));
                }
            }
        } else {
            path.pop();
        }
    }
    res
}
