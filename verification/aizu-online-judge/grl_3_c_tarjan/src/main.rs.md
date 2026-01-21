---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: data-structure/union-find/src/lib.rs
    title: Union Find
  - icon: ':question:'
    path: graph/strongly-connected-components-tarjan/src/lib.rs
    title: Strongly Connected Components (Tarjan)
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_C
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_C
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_3_C\n\
    \nuse proconio::input;\n\nuse strongly_connected_components_tarjan::StronglyConnectedComponents;\n\
    use union_find::UnionFind;\n\nfn main() {\n    input! {\n        v: usize,\n \
    \       e: usize,\n        st: [(usize, usize); e],\n        q: usize,\n     \
    \   uv: [(usize, usize); q],\n    }\n    let mut scc = StronglyConnectedComponents::new(v);\n\
    \    st.iter().for_each(|&(s, t)| scc.add_edge(s, t));\n    let groups = scc.scc();\n\
    \n    let mut uf = UnionFind::new(v);\n    for group in groups {\n        for\
    \ i in 0..group.len() {\n            uf.merge(group[0], group[i]);\n        }\n\
    \    }\n\n    for &(u, v) in &uv {\n        println!(\"{}\", if uf.same(u, v)\
    \ { 1 } else { 0 });\n    }\n}\n"
  dependsOn:
  - data-structure/union-find/src/lib.rs
  - graph/strongly-connected-components-tarjan/src/lib.rs
  isVerificationFile: true
  path: verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs
layout: document
redirect_from:
- /verify/verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs
- /verify/verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs.html
title: verification/aizu-online-judge/grl_3_c_tarjan/src/main.rs
---
