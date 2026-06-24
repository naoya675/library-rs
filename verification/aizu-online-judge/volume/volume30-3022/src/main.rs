// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=3022

use proconio::input;

use block_cut_tree::BlockCutTree;

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(actual_main)
        .unwrap()
        .join()
        .unwrap();
}

fn actual_main() {
    input! {
        n: usize,
        m: usize,
        w: [usize; n],
        uv: [(usize, usize); m],
    }
    let mut bct = BlockCutTree::new(n);
    for &(u, v) in &uv {
        bct.add_edge(u - 1, v - 1);
    }
    bct.build();

    let spec_sum = w.iter().sum::<usize>();
    let mut spec = vec![0; bct.tree().len()];
    for i in 0..n {
        spec[bct.rev(i)] += w[i];
    }
    let mut res = vec![0; n];
    for i in 0..n {
        res[i] = spec_sum - w[i];
    }

    struct Env<'a> {
        tree: &'a [Vec<usize>],
        group: &'a [Vec<usize>],
        num_blocks: usize,
        spec: Vec<usize>,
        spec_sum: usize,
        res: Vec<usize>,
    }

    let mut env = Env {
        tree: bct.tree(),
        group: bct.group(),
        num_blocks: bct.bcc().bc().len(),
        spec,
        spec_sum,
        res,
    };

    fn dfs(env: &mut Env<'_>, v: usize, p: Option<usize>) -> usize {
        let mut sum = 0;
        let mut max = 0;
        for i in 0..env.tree[v].len() {
            let to = env.tree[v][i];
            if Some(to) == p {
                continue;
            }
            let res = dfs(env, to, Some(v));
            sum += res;
            max = max.max(res);
        }
        if v >= env.num_blocks {
            env.res[env.group[v][0]] = (env.spec_sum - sum - env.spec[v]).max(max);
        }
        sum + env.spec[v]
    }

    dfs(&mut env, 0, None);
    for i in 0..n {
        println!("{}", env.res[i]);
    }
}
