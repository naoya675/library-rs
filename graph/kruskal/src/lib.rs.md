---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: data-structure/union-find/src/lib.rs
    title: Union Find
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_2_a/src/main.rs
    title: verification/aizu-online-judge/grl_2_a/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use union_find::UnionFind;\n\npub fn minimum_spanning_tree(size: usize, edge:\
    \ &mut [(usize, usize, i64)]) -> (i64, Vec<(usize, usize, i64)>) {\n    edge.sort_by(|a,\
    \ b| a.2.cmp(&b.2));\n    let mut uf = UnionFind::new(size);\n    let mut res\
    \ = 0;\n    let mut res_edge = vec![];\n    for &mut (from, to, cost) in edge\
    \ {\n        if uf.same(from, to) {\n            continue;\n        }\n      \
    \  uf.merge(from, to);\n        res += cost;\n        res_edge.push((from, to,\
    \ cost));\n    }\n    (res, res_edge)\n}\n\npub fn maximum_spanning_tree(size:\
    \ usize, edge: &mut [(usize, usize, i64)]) -> (i64, Vec<(usize, usize, i64)>)\
    \ {\n    edge.sort_by(|a, b| b.2.cmp(&a.2));\n    let mut uf = UnionFind::new(size);\n\
    \    let mut res = 0;\n    let mut res_edge = vec![];\n    for &mut (from, to,\
    \ cost) in edge {\n        if uf.same(from, to) {\n            continue;\n   \
    \     }\n        uf.merge(from, to);\n        res += cost;\n        res_edge.push((from,\
    \ to, cost));\n    }\n    (res, res_edge)\n}\n"
  dependsOn:
  - data-structure/union-find/src/lib.rs
  isVerificationFile: false
  path: graph/kruskal/src/lib.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/grl_2_a/src/main.rs
documentation_of: graph/kruskal/src/lib.rs
layout: document
title: Kruskal
---

## Description
