---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/number_of_substrings/src/main.rs
    title: verification/library-checker/number_of_substrings/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/suffixarray/src/main.rs
    title: verification/library-checker/suffixarray/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://atcoder.github.io/ac-library/production/document_en/string.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.12/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "//! https://atcoder.github.io/ac-library/production/document_en/string.html\n\
    \n#[derive(Debug)]\npub struct SuffixArray;\n\nimpl SuffixArray {\n    fn sa_naive<T:\
    \ Copy + Ord + PartialOrd>(s: &Vec<T>) -> Vec<usize> {\n        let n = s.len();\n\
    \        let mut sa = (0..n).collect::<Vec<usize>>();\n        sa.sort_by(|&a,\
    \ &b| {\n            if a == b {\n                return std::cmp::Ordering::Greater;\n\
    \            }\n            let mut i = a;\n            let mut j = b;\n     \
    \       while i < n && j < n {\n                if s[i] != s[j] {\n          \
    \          if s[i] < s[j] {\n                        return std::cmp::Ordering::Less;\n\
    \                    } else {\n                        return std::cmp::Ordering::Greater;\n\
    \                    }\n                }\n                i += 1;\n         \
    \       j += 1;\n            }\n            if a == n {\n                std::cmp::Ordering::Less\n\
    \            } else {\n                std::cmp::Ordering::Greater\n         \
    \   }\n        });\n        sa\n    }\n\n    fn sa_doubling(s: &Vec<usize>) ->\
    \ Vec<usize> {\n        let n = s.len();\n        let mut sa = (0..n).collect::<Vec<usize>>();\n\
    \        let mut rank = s.clone();\n        let mut rank_temp = vec![0; n];\n\
    \        let mut k = 1;\n        while k < n {\n            let cmp = |&a: &usize,\
    \ &b: &usize| {\n                if rank[a] != rank[b] {\n                   \
    \ rank[a].cmp(&rank[b])\n                } else {\n                    let ra\
    \ = if a + k < n { rank[a + k] as isize } else { -1 };\n                    let\
    \ rb = if b + k < n { rank[b + k] as isize } else { -1 };\n                  \
    \  if ra < rb {\n                        std::cmp::Ordering::Less\n          \
    \          } else {\n                        std::cmp::Ordering::Greater\n   \
    \                 }\n                }\n            };\n            sa.sort_by(cmp);\n\
    \            rank_temp[sa[0]] = 0;\n            for i in 1..n {\n            \
    \    rank_temp[sa[i]] = rank_temp[sa[i - 1]] + if cmp(&sa[i - 1], &sa[i]) == std::cmp::Ordering::Less\
    \ { 1 } else { 0 };\n            }\n            std::mem::swap(&mut rank, &mut\
    \ rank_temp);\n            k <<= 1;\n        }\n        sa\n    }\n\n    fn sa_is(s:\
    \ &Vec<usize>, upper: usize) -> Vec<usize> {\n        let n = s.len();\n     \
    \   if n == 0 {\n            return vec![];\n        }\n        if n == 1 {\n\
    \            return vec![0];\n        }\n        if n == 2 {\n            return\
    \ if s[0] < s[1] { vec![0, 1] } else { vec![1, 0] };\n        }\n\n        let\
    \ mut sa = vec![None; n];\n        let mut ls = vec![0; n];\n        for i in\
    \ (0..n - 1).rev() {\n            if s[i] == s[i + 1] {\n                ls[i]\
    \ = ls[i + 1];\n            } else if s[i] < s[i + 1] {\n                ls[i]\
    \ = 1;\n            } else if s[i] > s[i + 1] {\n                ls[i] = 0;\n\
    \            }\n        }\n        let mut sum_l = vec![0; upper + 1];\n     \
    \   let mut sum_s = vec![0; upper + 1];\n        for i in 0..n {\n           \
    \ if ls[i] == 0 {\n                sum_s[s[i]] += 1;\n            } else {\n \
    \               sum_l[s[i] + 1] += 1;\n            }\n        }\n        for i\
    \ in 0..upper + 1 {\n            sum_s[i] += sum_l[i];\n            if i < upper\
    \ {\n                sum_l[i + 1] += sum_s[i];\n            }\n        }\n\n \
    \       let induce = |lms: &Vec<usize>, sa: &mut Vec<Option<usize>>| {\n     \
    \       sa.fill(None);\n            let mut buf = sum_s.clone();\n           \
    \ for &d in lms {\n                if d == n {\n                    continue;\n\
    \                }\n                sa[buf[s[d]]] = Some(d);\n               \
    \ buf[s[d]] += 1;\n            }\n            let mut buf = sum_l.clone();\n \
    \           sa[buf[s[n - 1]]] = Some(n - 1);\n            buf[s[n - 1]] += 1;\n\
    \            for i in 0..n {\n                if let Some(v) = sa[i] {\n     \
    \               if v >= 1 && ls[v - 1] == 0 {\n                        sa[buf[s[v\
    \ - 1]]] = Some(v - 1);\n                        buf[s[v - 1]] += 1;\n       \
    \             }\n                }\n            }\n            let mut buf = sum_l.clone();\n\
    \            for i in (0..n).rev() {\n                if let Some(v) = sa[i] {\n\
    \                    if v >= 1 && ls[v - 1] != 0 {\n                        buf[s[v\
    \ - 1] + 1] -= 1;\n                        sa[buf[s[v - 1] + 1]] = Some(v - 1);\n\
    \                    }\n                }\n            }\n        };\n\n     \
    \   let mut lms_map = vec![None; n + 1];\n        let mut m = 0;\n        for\
    \ i in 1..n {\n            if ls[i - 1] == 0 && ls[i] != 0 {\n               \
    \ lms_map[i] = Some(m);\n                m += 1;\n            }\n        }\n \
    \       let mut lms = vec![];\n        lms.reserve(m);\n        for i in 1..n\
    \ {\n            if ls[i - 1] == 0 && ls[i] != 0 {\n                lms.push(i);\n\
    \            }\n        }\n        induce(&lms, &mut sa);\n        if m > 0 {\n\
    \            let mut sorted_lms = vec![];\n            sorted_lms.reserve(m);\n\
    \            for &v_option in &sa {\n                if let Some(v) = v_option\
    \ {\n                    if lms_map[v].is_some() {\n                        sorted_lms.push(v);\n\
    \                    }\n                }\n            }\n            let mut\
    \ rec_s = vec![0; m];\n            let mut rec_upper = 0;\n            rec_s[lms_map[sorted_lms[0]].unwrap()]\
    \ = 0;\n            for i in 1..m {\n                let mut l = sorted_lms[i\
    \ - 1];\n                let mut r = sorted_lms[i];\n                let end_l\
    \ = if lms_map[l].unwrap() + 1 < m { lms[lms_map[l].unwrap() + 1] } else { n };\n\
    \                let end_r = if lms_map[r].unwrap() + 1 < m { lms[lms_map[r].unwrap()\
    \ + 1] } else { n };\n                let mut same = true;\n                if\
    \ end_l - l != end_r - r {\n                    same = false;\n              \
    \  } else {\n                    while l < end_l {\n                        if\
    \ s[l] != s[r] {\n                            break;\n                       \
    \ }\n                        l += 1;\n                        r += 1;\n      \
    \              }\n                    if l == n || s[l] != s[r] {\n          \
    \              same = false;\n                    }\n                }\n     \
    \           if !same {\n                    rec_upper += 1;\n                }\n\
    \                rec_s[lms_map[sorted_lms[i]].unwrap()] = rec_upper;\n       \
    \     }\n            let rec_sa = SuffixArray::sa_is(&rec_s, rec_upper);\n   \
    \         for i in 0..m {\n                sorted_lms[i] = lms[rec_sa[i]];\n \
    \           }\n            induce(&sorted_lms, &mut sa);\n        }\n        sa.into_iter().map(|a|\
    \ a.unwrap()).collect()\n    }\n\n    pub fn suffix_array<T: Copy + Ord + PartialOrd>(s:\
    \ &Vec<T>) -> Vec<usize> {\n        let n = s.len();\n        let mut idx = (0..n).collect::<Vec<usize>>();\n\
    \        idx.sort_by(|&a, &b| s[a].cmp(&s[b]));\n        let mut t = vec![0; n];\n\
    \        let mut now = 0;\n        for i in 0..n {\n            if i > 0 && s[idx[i\
    \ - 1]] != s[idx[i]] {\n                now += 1;\n            }\n           \
    \ t[idx[i]] = now;\n        }\n        Self::sa_is(&t, now)\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: string/suffix-array/src/lib.rs
  requiredBy: []
  timestamp: '2025-05-26 15:54:30+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/library-checker/suffixarray/src/main.rs
  - verification/library-checker/number_of_substrings/src/main.rs
documentation_of: string/suffix-array/src/lib.rs
layout: document
title: Suffix Array
---

## Description
