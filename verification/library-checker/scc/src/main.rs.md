---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: graph/strongly-connected-components/src/lib.rs
    title: Strongly Connected Components (Tarjan)
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/scc
    links:
    - https://judge.yosupo.jp/problem/scc
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc\n\nuse\
    \ itertools::Itertools;\nuse proconio::input;\n\nuse strongly_connected_components::StronglyConnectedComponents;\n\
    \nfn main() {\n    std::thread::Builder::new()\n        .stack_size(64 * 1024\
    \ * 1024)\n        .spawn(actual_main)\n        .unwrap()\n        .join()\n \
    \       .unwrap();\n}\n\nfn actual_main() {\n    input! {\n        n: usize,\n\
    \        m: usize,\n        ab: [(usize, usize); m],\n    }\n    let mut scc =\
    \ StronglyConnectedComponents::new(n);\n    ab.iter().for_each(|&(a, b)| scc.add_edge(a,\
    \ b));\n    let groups = scc.scc();\n\n    println!(\"{}\", groups.len());\n \
    \   for group in groups {\n        println!(\"{} {}\", group.len(), group.iter().join(\"\
    \ \"));\n    }\n}\n"
  dependsOn:
  - graph/strongly-connected-components/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/scc/src/main.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verification/library-checker/scc/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/scc/src/main.rs
- /verify/verification/library-checker/scc/src/main.rs.html
title: verification/library-checker/scc/src/main.rs
---
