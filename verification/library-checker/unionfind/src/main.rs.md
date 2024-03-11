---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: data-structure/union-find/src/lib.rs
    title: Union Find
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind
    links:
    - https://judge.yosupo.jp/problem/unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind\n\
    \nuse proconio::input;\n\nuse union_find::UnionFind;\n\nfn main() {\n    input!\
    \ {\n        n: usize,\n        q: usize,\n        tuv: [(usize, usize, usize);\
    \ q],\n    }\n    let mut uf = UnionFind::new(n);\n    for (t, u, v) in tuv {\n\
    \        match t {\n            0 => {\n                uf.merge(u, v);\n    \
    \        }\n            1 => {\n                println!(\"{}\", if uf.same(u,\
    \ v) { 1 } else { 0 });\n            }\n            _ => unreachable!(),\n   \
    \     }\n    }\n}\n"
  dependsOn:
  - data-structure/union-find/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/unionfind/src/main.rs
  requiredBy: []
  timestamp: '2024-03-11 21:49:40+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/unionfind/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/unionfind/src/main.rs
- /verify/verification/library-checker/unionfind/src/main.rs.html
title: verification/library-checker/unionfind/src/main.rs
---