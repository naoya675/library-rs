use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

pub fn topological_sort(size: usize, graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut indegree = vec![0; size];
    for from in 0..size {
        for &to in &graph[from] {
            indegree[to] += 1;
        }
    }
    let mut queue = VecDeque::new();
    for from in 0..size {
        if indegree[from] == 0 {
            queue.push_back(from);
        }
    }
    let mut res = vec![];
    while let Some(from) = queue.pop_front() {
        res.push(from);
        for &to in &graph[from] {
            indegree[to] -= 1;
            if indegree[to] == 0 {
                queue.push_back(to);
            }
        }
    }
    if res.len() != size {
        return None;
    }
    Some(res)
}

pub fn topological_sort_lex(size: usize, graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    let mut indegree = vec![0; size];
    for from in 0..size {
        for &to in &graph[from] {
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
        for &to in &graph[from] {
            indegree[to] -= 1;
            if indegree[to] == 0 {
                heap.push(Reverse(to));
            }
        }
    }
    if res.len() != size {
        return None;
    }
    Some(res)
}
