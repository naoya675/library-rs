use std::{cmp::Reverse, collections::BinaryHeap};

pub fn topological_sort(size: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut indegree = vec![0; size];
    for from in 0..size {
        for i in 0..graph[from].len() {
            let to = graph[from][i];
            indegree[to] += 1;
        }
    }
    let mut heap = BinaryHeap::new();
    for from in 0..size {
        if indegree[from] == 0 {
            heap.push(Reverse(from));
        }
    }
    let mut res = vec![];
    while let Some(Reverse(from)) = heap.pop() {
        res.push(from);
        for i in 0..graph[from].len() {
            let to = graph[from][i];
            indegree[to] -= 1;
            if indegree[to] == 0 {
                heap.push(Reverse(to));
            }
        }
    }
    if res.len() != size {
        return vec![];
    }
    res
}
