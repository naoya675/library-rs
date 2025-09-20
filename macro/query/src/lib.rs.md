---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_1_a/src/main.rs
    title: verification/aizu-online-judge/dsl_1_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_1_b/src/main.rs
    title: verification/aizu-online-judge/dsl_1_b/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_a/src/main.rs
    title: verification/aizu-online-judge/dsl_2_a/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_b/src/main.rs
    title: verification/aizu-online-judge/dsl_2_b/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_f/src/main.rs
    title: verification/aizu-online-judge/dsl_2_f/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_g/src/main.rs
    title: verification/aizu-online-judge/dsl_2_g/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_h/src/main.rs
    title: verification/aizu-online-judge/dsl_2_h/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dsl_2_i/src/main.rs
    title: verification/aizu-online-judge/dsl_2_i/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_d/src/main.rs
    title: verification/aizu-online-judge/grl_5_d/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/grl_5_e/src/main.rs
    title: verification/aizu-online-judge/grl_5_e/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/point_set_range_composite/src/main.rs
    title: verification/library-checker/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/range_affine_range_sum/src/main.rs
    title: verification/library-checker/range_affine_range_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind/src/main.rs
    title: verification/library-checker/unionfind/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind_with_potential/src/main.rs
    title: verification/library-checker/unionfind_with_potential/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/vertex_add_path_sum/src/main.rs
    title: verification/library-checker/vertex_add_path_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/vertex_add_subtree_sum/src/main.rs
    title: verification/library-checker/vertex_add_subtree_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/vertex_set_path_composite/src/main.rs
    title: verification/library-checker/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[macro_export]\nmacro_rules! define_query {\n    (\n        $name:ident\
    \ {\n            $( $tag:literal => $variant:ident ( $( $field:ident : $ty:ty\
    \ ),* $(,)? ) ),* $(,)?\n        }\n    ) => {\n        #[derive(Copy, Clone,\
    \ Debug, Eq, PartialEq)]\n        enum $name {\n            $( $variant( $( $ty\
    \ ),* ), )*\n        }\n        use $name::*;\n\n        impl proconio::source::Readable\
    \ for $name {\n            type Output = $name;\n            fn read<R: std::io::BufRead,\
    \ S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {\n        \
    \        match u32::read(source) {\n                    $(\n                 \
    \       $tag => {\n                            input! { from source, $( mut $field:\
    \ $ty ),* }\n                            $variant( $( $field ),* )\n         \
    \               }\n                    )*\n                    _ => unreachable!(),\n\
    \                }\n            }\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: macro/query/src/lib.rs
  requiredBy: []
  timestamp: '2025-09-06 15:04:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dsl_2_a/src/main.rs
  - verification/aizu-online-judge/dsl_1_b/src/main.rs
  - verification/aizu-online-judge/dsl_2_g/src/main.rs
  - verification/aizu-online-judge/grl_5_d/src/main.rs
  - verification/aizu-online-judge/grl_5_e/src/main.rs
  - verification/aizu-online-judge/dsl_2_i/src/main.rs
  - verification/aizu-online-judge/dsl_2_h/src/main.rs
  - verification/aizu-online-judge/dsl_2_f/src/main.rs
  - verification/aizu-online-judge/dsl_1_a/src/main.rs
  - verification/aizu-online-judge/dsl_2_b/src/main.rs
  - verification/library-checker/vertex_set_path_composite/src/main.rs
  - verification/library-checker/unionfind/src/main.rs
  - verification/library-checker/vertex_add_subtree_sum/src/main.rs
  - verification/library-checker/vertex_add_path_sum/src/main.rs
  - verification/library-checker/unionfind_with_potential/src/main.rs
  - verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  - verification/library-checker/point_set_range_composite/src/main.rs
  - verification/library-checker/range_affine_range_sum/src/main.rs
documentation_of: macro/query/src/lib.rs
layout: document
redirect_from:
- /library/macro/query/src/lib.rs
- /library/macro/query/src/lib.rs.html
title: macro/query/src/lib.rs
---
