---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs
    title: verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs
  - icon: ':x:'
    path: verification/library-checker/scc-tarjan/src/main.rs
    title: verification/library-checker/scc-tarjan/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[derive(Debug, Clone, Copy)]\npub struct Edge {\n    from: usize,\n    to:\
    \ usize,\n}\n\nimpl Edge {\n    pub fn new(from: usize, to: usize) -> Self {\n\
    \        Self { from, to }\n    }\n}\n\n#[derive(Debug, Clone)]\npub struct CompressedSparseRow\
    \ {\n    start: Vec<usize>,\n    elist: Vec<Edge>,\n}\n\nimpl CompressedSparseRow\
    \ {\n    pub fn new(n: usize, edges: &[(usize, Edge)]) -> Self {\n        let\
    \ mut start = vec![0; n + 1];\n        let mut elist = vec![Edge { from: 0, to:\
    \ 0 }; edges.len()];\n        for &(from, _) in edges {\n            start[from\
    \ + 1] += 1;\n        }\n        for i in 1..=n {\n            start[i] += start[i\
    \ - 1];\n        }\n        let mut counter = start.clone();\n        for &(from,\
    \ e) in edges {\n            elist[counter[from]] = e;\n            counter[from]\
    \ += 1;\n        }\n        Self { start, elist }\n    }\n}\n\n#[derive(Debug,\
    \ Clone)]\npub struct StronglyConnectedComponents {\n    size: usize,\n    edge:\
    \ Vec<(usize, Edge)>,\n}\n\nimpl StronglyConnectedComponents {\n    pub fn new(size:\
    \ usize) -> Self {\n        Self { size, edge: vec![] }\n    }\n\n    pub fn add_edge(&mut\
    \ self, from: usize, to: usize) {\n        assert!(from < self.size);\n      \
    \  assert!(to < self.size);\n        self.edge.push((from, Edge::new(from, to)));\n\
    \    }\n\n    // Tarjan's strongly connected components algorithm\n    pub fn\
    \ scc_ids(&mut self) -> (usize, Vec<usize>) {\n        let g = CompressedSparseRow::new(self.size,\
    \ &self.edge);\n\n        struct Env {\n            group_num: usize,\n      \
    \      now_ord: usize,\n            visited: Vec<usize>,\n            low: Vec<usize>,\
    \ // lowlink\n            ord: Vec<usize>, // dfs order\n            ids: Vec<usize>,\n\
    \        }\n\n        let mut env = Env {\n            group_num: 0,\n       \
    \     now_ord: 0,\n            visited: Vec::with_capacity(self.size),\n     \
    \       low: vec![0; self.size],\n            ord: vec![usize::MAX; self.size],\n\
    \            ids: vec![0; self.size],\n        };\n\n        // fn dfs(v: usize,\
    \ env: &mut Env) {}\n\n        struct Recursive<'a> {\n            f: &'a dyn\
    \ Fn(&Recursive<'a>, &mut Env, usize),\n        }\n\n        let dfs = Recursive\
    \ {\n            f: &|dfs: &Recursive<'_>, env: &mut Env, v: usize| {\n      \
    \          env.low[v] = env.now_ord;\n                env.ord[v] = env.now_ord;\n\
    \                env.now_ord += 1;\n                env.visited.push(v);\n   \
    \             for i in g.start[v]..g.start[v + 1] {\n                    let to\
    \ = g.elist[i].to;\n                    if env.ord[to] == usize::MAX {\n     \
    \                   (dfs.f)(dfs, env, to);\n                        env.low[v]\
    \ = env.low[v].min(env.low[to]);\n                    } else {\n             \
    \           env.low[v] = env.low[v].min(env.ord[to]);\n                    }\n\
    \                }\n                if env.low[v] == env.ord[v] {\n          \
    \          loop {\n                        let u = env.visited.pop().unwrap();\n\
    \                        env.ord[u] = self.size;\n                        env.ids[u]\
    \ = env.group_num;\n                        if u == v {\n                    \
    \        break;\n                        }\n                    }\n          \
    \          env.group_num += 1;\n                }\n            },\n        };\n\
    \n        for i in 0..self.size {\n            if env.ord[i] == usize::MAX {\n\
    \                (dfs.f)(&dfs, &mut env, i);\n            }\n        }\n     \
    \   for x in env.ids.iter_mut() {\n            *x = env.group_num - 1 - *x;\n\
    \        }\n        (env.group_num, env.ids)\n    }\n\n    pub fn scc(&mut self)\
    \ -> Vec<Vec<usize>> {\n        let (group_num, ids) = self.scc_ids();\n     \
    \   let mut counts = vec![0; group_num];\n        for &i in &ids {\n         \
    \   counts[i] += 1;\n        }\n        let mut groups = vec![vec![]; group_num];\n\
    \        for i in 0..group_num {\n            groups[i].reserve(counts[i]);\n\
    \        }\n        for i in 0..self.size {\n            groups[ids[i]].push(i);\n\
    \        }\n        groups\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: graph/strongly-connected-components-tarjan/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs
  - verification/library-checker/scc-tarjan/src/main.rs
documentation_of: graph/strongly-connected-components-tarjan/src/lib.rs
layout: document
title: Strongly Connected Components (Tarjan)
---

## Description

## Reference
- [https://atcoder.github.io/ac-library/production/document_en/scc.html](https://atcoder.github.io/ac-library/production/document_en/scc.html)
- [https://qiita.com/sysdev/items/4532d52ab9978cd9d4d4](https://qiita.com/sysdev/items/4532d52ab9978cd9d4d4)
- [https://ngtkana.hatenablog.com/entry/2024/12/10/011904](https://ngtkana.hatenablog.com/entry/2024/12/10/011904)
- [https://tubo28.me/compprog/algorithm/tarjan-scc/](https://tubo28.me/compprog/algorithm/tarjan-scc/)
<!--- [https://inzkyk.xyz/algorithms/depth_first_search/strong_components_in_linear_time/](https://inzkyk.xyz/algorithms/depth_first_search/strong_components_in_linear_time/)-->
