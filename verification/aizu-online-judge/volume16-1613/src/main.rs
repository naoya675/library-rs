// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1613

use std::collections::{HashMap, HashSet, VecDeque};

use proconio::{input, marker::Chars};

use ahu::labels;

fn main() {
    loop {
        input! {
            h1: usize,
            w1: usize,
        }
        if (h1, w1) == (0, 0) {
            break;
        }
        input! {
            p1: [Chars; h1],
            h2: usize,
            w2: usize,
            p2: [Chars; h2],
        }
        let tree1 = p2tree(&p1, h1, w1);
        let tree2 = p2tree(&p2, h2, w2);
        let mut canonical: HashMap<Vec<usize>, usize> = HashMap::new();
        let l1 = labels(&tree1, 0, &mut canonical)[0];
        let l2 = labels(&tree2, 0, &mut canonical)[0];
        println!("{}", if l1 == l2 { "yes" } else { "no" });
    }
}

fn p2tree(p: &[Vec<char>], h: usize, w: usize) -> Vec<Vec<usize>> {
    let h = h + 2;
    let w = w + 2;
    let mut grid = vec![vec!['.'; w]; h];
    for i in 0..h - 2 {
        for j in 0..w - 2 {
            grid[i + 1][j + 1] = p[i][j];
        }
    }

    let dx = [0, 0, 1, -1, 1, 1, -1, -1];
    let dy = [1, -1, 0, 0, 1, -1, 1, -1];
    let mut comp = vec![vec![None; w]; h];
    let mut n = 0;
    for x in 0..h {
        for y in 0..w {
            if comp[x][y].is_some() {
                continue;
            }
            comp[x][y] = Some(n);
            let mut que = VecDeque::new();
            que.push_back((x, y));
            while let Some((x, y)) = que.pop_front() {
                for k in 0..if grid[x][y] == '.' { 4 } else { 8 } {
                    if x as isize + dx[k] < 0 || y as isize + dy[k] < 0 {
                        continue;
                    }
                    let nx = (x as isize + dx[k]) as usize;
                    let ny = (y as isize + dy[k]) as usize;
                    if nx >= h || ny >= w {
                        continue;
                    }
                    if comp[nx][ny].is_some() {
                        continue;
                    }
                    if grid[x][y] != grid[nx][ny] {
                        continue;
                    }
                    comp[nx][ny] = Some(n);
                    que.push_back((nx, ny));
                }
            }
            n += 1;
        }
    }

    let mut set: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for x in 0..h {
        for y in 0..w {
            for k in 0..4 {
                if x as isize + dx[k] < 0 || y as isize + dy[k] < 0 {
                    continue;
                }
                let nx = (x as isize + dx[k]) as usize;
                let ny = (y as isize + dy[k]) as usize;
                if nx >= h || ny >= w {
                    continue;
                }
                if comp[x][y] != comp[nx][ny] {
                    set[comp[x][y].unwrap()].insert(comp[nx][ny].unwrap());
                }
            }
        }
    }

    let mut tree = vec![vec![]; n];
    for i in 0..n {
        for &j in &set[i] {
            if i < j {
                tree[i].push(j);
                tree[j].push(i);
            }
        }
    }
    tree
}
