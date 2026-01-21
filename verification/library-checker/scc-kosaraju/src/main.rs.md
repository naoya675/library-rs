---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: graph/strongly-connected-components-kosaraju/src/lib.rs
    title: Strongly Connected Components (Kosaraju)
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/scc
    links:
    - https://judge.yosupo.jp/problem/scc
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.14/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc\n\nuse\
    \ itertools::Itertools;\nuse proconio::input;\n\nuse strongly_connected_components_kosaraju::StronglyConnectedComponents;\n\
    \nfn main() {\n    std::thread::Builder::new()\n        .stack_size(64 * 1024\
    \ * 1024)\n        .spawn(actual_main)\n        .unwrap()\n        .join()\n \
    \       .unwrap();\n}\n\nfn actual_main() {\n    input! {\n        n: usize,\n\
    \        m: usize,\n        ab: [(usize, usize); m],\n    }\n    let mut scc =\
    \ StronglyConnectedComponents::new(n);\n    ab.iter().for_each(|&(a, b)| scc.add_edge(a,\
    \ b));\n    let groups = scc.scc();\n\n    println!(\"{}\", groups.len());\n \
    \   for group in groups {\n        println!(\"{} {}\", group.len(), group.iter().join(\"\
    \ \"));\n    }\n}\n"
  dependsOn:
  - graph/strongly-connected-components-kosaraju/src/lib.rs
  isVerificationFile: true
  path: verification/library-checker/scc-kosaraju/src/main.rs
  requiredBy: []
  timestamp: '2026-01-01 00:11:18+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verification/library-checker/scc-kosaraju/src/main.rs
layout: document
redirect_from:
- /verify/verification/library-checker/scc-kosaraju/src/main.rs
- /verify/verification/library-checker/scc-kosaraju/src/main.rs.html
title: verification/library-checker/scc-kosaraju/src/main.rs
---
