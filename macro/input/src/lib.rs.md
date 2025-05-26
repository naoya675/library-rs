---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8\n\nmacro_rules! input\
    \ {\n    (source = $s:expr, $($r:tt)*) => {\n        let iter = $s.split_whitespace();\n\
    \        input_inner!{iter, $($r)*}\n    };\n    ($($r:tt)*) => {\n        let\
    \ s = {\n            use std::io::Read;\n            let mut s = String::new();\n\
    \            std::io::stdin().read_to_string(&mut s).unwrap();\n            s\n\
    \        };\n        let mut iter = s.split_whitespace();\n        input_inner!{iter,\
    \ $($r)*}\n    };\n}\n\nmacro_rules! input_inner {\n    ($iter:expr) => {};\n\
    \    ($iter:expr, ) => {};\n    ($iter:expr, $var:ident : $t:tt $($r:tt)*) =>\
    \ {\n        let $var = read_value!($iter, $t);\n        input_inner!{$iter $($r)*}\n\
    \    };\n}\n\nmacro_rules! read_value {\n    ($iter:expr, ( $($t:tt),* )) => {\n\
    \        ( $(read_value!($iter, $t)),* )\n    };\n    ($iter:expr, [ $t:tt ; $len:expr\
    \ ]) => {\n        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()\n\
    \    };\n    ($iter:expr, Chars) => {\n        read_value!($iter, String).chars().collect::<Vec<char>>()\n\
    \    };\n    ($iter:expr, usize1) => {\n        read_value!($iter, usize) - 1\n\
    \    };\n    ($iter:expr, usize0) => {\n        read_value!($iter, usize)\n  \
    \  };\n    ($iter:expr, $t:ty) => {\n        $iter.next().unwrap().parse::<$t>().expect(\"\
    Parse Error\")\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: macro/input/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-26 15:54:30+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: macro/input/src/lib.rs
layout: document
title: input
---

## Description
