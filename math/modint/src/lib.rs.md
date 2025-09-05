---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verification/aizu-online-judge/dpl_5_d/src/main.rs
    title: verification/aizu-online-judge/dpl_5_d/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/point_set_range_composite/src/main.rs
    title: verification/library-checker/point_set_range_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/range_affine_range_sum/src/main.rs
    title: verification/library-checker/range_affine_range_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind_with_potential/src/main.rs
    title: verification/library-checker/unionfind_with_potential/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verification/library-checker/vertex_set_path_composite/src/main.rs
    title: verification/library-checker/vertex_set_path_composite/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://qiita.com/namn1125/items/5100cb85021a1d6e8f6c
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.13/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// reference: https://qiita.com/namn1125/items/5100cb85021a1d6e8f6c\n\n#[derive(Debug,\
    \ Clone, Copy, PartialEq, Eq)]\npub struct StaticModint<const MOD: u64> {\n  \
    \  value: u64,\n}\n\nimpl<const MOD: u64> StaticModint<MOD> {\n    pub fn new(n:\
    \ u64) -> Self {\n        Self {\n            value: (n % MOD),\n            //\
    \ value: (n.rem_euclid(MOD)),\n        }\n    }\n\n    pub fn value(&self) ->\
    \ u64 {\n        self.value\n    }\n\n    fn ext_gcd(&self, a: i64, b: i64) ->\
    \ (i64, i64, i64) {\n        let (mut x0, mut y0, mut r0) = (1, 0, a);\n     \
    \   let (mut x1, mut y1, mut r1) = (0, 1, b);\n        while r1 != 0 {\n     \
    \       let t = r0 / r1;\n            x0 -= t * x1;\n            y0 -= t * y1;\n\
    \            r0 -= t * r1;\n            std::mem::swap(&mut x0, &mut x1);\n  \
    \          std::mem::swap(&mut y0, &mut y1);\n            std::mem::swap(&mut\
    \ r0, &mut r1);\n        }\n        // (x0, y0, r0)\n        (x0.rem_euclid(b),\
    \ y0.rem_euclid(b), r0.rem_euclid(b))\n    }\n\n    pub fn pow(&self, mut n: u64)\
    \ -> Self {\n        let mut value = *self;\n        let mut res = Self::new(1);\n\
    \        while n > 0 {\n            if n & 1 != 0 {\n                res = res\
    \ * value;\n            }\n            value = value * value;\n            n >>=\
    \ 1;\n        }\n        res\n    }\n\n    pub fn inv(&self) -> Self {\n     \
    \   let (x, _, _) = self.ext_gcd(self.value() as i64, MOD as i64);\n        Self\
    \ { value: x as u64 }\n        // self.pow(MOD - 2)\n    }\n}\n\nimpl<const MOD:\
    \ u64> std::ops::Add for StaticModint<MOD> {\n    type Output = Self;\n    fn\
    \ add(self, rhs: Self) -> Self {\n        Self {\n            value: (self.value\
    \ + rhs.value) % MOD,\n        }\n    }\n}\n\nimpl<const MOD: u64> std::ops::AddAssign\
    \ for StaticModint<MOD> {\n    fn add_assign(&mut self, rhs: Self) {\n       \
    \ *self = *self + rhs;\n    }\n}\n\nimpl<const MOD: u64> std::ops::Sub for StaticModint<MOD>\
    \ {\n    type Output = Self;\n    fn sub(mut self, rhs: Self) -> Self {\n    \
    \    if self.value < rhs.value {\n            self.value += MOD;\n        }\n\
    \        Self {\n            value: (self.value - rhs.value) % MOD,\n        }\n\
    \    }\n}\n\nimpl<const MOD: u64> std::ops::SubAssign for StaticModint<MOD> {\n\
    \    fn sub_assign(&mut self, rhs: Self) {\n        *self = *self - rhs;\n   \
    \ }\n}\n\nimpl<const MOD: u64> std::ops::Mul for StaticModint<MOD> {\n    type\
    \ Output = Self;\n    fn mul(self, rhs: Self) -> Self {\n        Self {\n    \
    \        value: (self.value * rhs.value) % MOD,\n        }\n    }\n}\n\nimpl<const\
    \ MOD: u64> std::ops::MulAssign for StaticModint<MOD> {\n    fn mul_assign(&mut\
    \ self, rhs: Self) {\n        *self = *self * rhs;\n    }\n}\n\nimpl<const MOD:\
    \ u64> std::ops::Div for StaticModint<MOD> {\n    type Output = Self;\n    fn\
    \ div(self, rhs: Self) -> Self {\n        if rhs.value == 0 {\n            panic!();\n\
    \        }\n        self * rhs.inv()\n    }\n}\n\nimpl<const MOD: u64> std::ops::DivAssign\
    \ for StaticModint<MOD> {\n    fn div_assign(&mut self, rhs: Self) {\n       \
    \ *self = *self / rhs;\n    }\n}\n\nimpl<const MOD: u64> std::ops::Neg for StaticModint<MOD>\
    \ {\n    type Output = Self;\n    fn neg(self) -> Self {\n        Self::new(0)\
    \ - self\n    }\n}\n\npub trait Zero {\n    fn zero() -> Self;\n    fn is_zero(&self)\
    \ -> bool;\n}\n\nimpl<const MOD: u64> Zero for StaticModint<MOD> {\n    fn zero()\
    \ -> Self {\n        Self::new(0)\n    }\n\n    fn is_zero(&self) -> bool {\n\
    \        Self::new(0) == *self\n    }\n}\n\npub trait One {\n    fn one() -> Self;\n\
    \    fn is_one(&self) -> bool;\n}\n\nimpl<const MOD: u64> One for StaticModint<MOD>\
    \ {\n    fn one() -> Self {\n        Self::new(1)\n    }\n\n    fn is_one(&self)\
    \ -> bool {\n        Self::new(1) == *self\n    }\n}\n\nimpl<const MOD: u64> Default\
    \ for StaticModint<MOD> {\n    fn default() -> Self {\n        Self::new(0)\n\
    \    }\n}\n\nimpl<const MOD: u64> std::fmt::Display for StaticModint<MOD> {\n\
    \    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n  \
    \      write!(f, \"{}\", self.value)\n    }\n}\n\nimpl<const MOD: u64> From<u64>\
    \ for StaticModint<MOD> {\n    fn from(value: u64) -> Self {\n        Self::new(value)\n\
    \    }\n}\n\n/*\nmacro_rules! impl_from {\n    ($($type:ty), *) => {\n       \
    \ $(\n            impl<const MOD: u64> From<$type> for StaticModint<MOD> {\n \
    \               fn from(value: $type) -> Self {\n                    Self::new(value\
    \ as u64)\n                }\n            }\n        )*\n    };\n}\n\nimpl_from!(i8,\
    \ u8, i16, u16, i32, u32, u64, i64, isize, usize);\n*/\n\n/*\nmacro_rules! impl_ops\
    \ {\n    ($trait:ident, $fn:ident, $op:tt) => {\n        impl<const MOD: u64>\
    \ std::ops::$trait for StaticModint<MOD> {\n            fn $fn(&mut self, rhs:\
    \ Self) {\n                *self = *self $op rhs;\n            }\n        }\n\
    \    };\n}\n\nimpl_ops!(AddAssign, add_assign, +);\nimpl_ops!(SubAssign, sub_assign,\
    \ -);\nimpl_ops!(MulAssign, mul_assign, *);\nimpl_ops!(DivAssign, div_assign,\
    \ /);\n*/\n"
  dependsOn: []
  isVerificationFile: false
  path: math/modint/src/lib.rs
  requiredBy: []
  timestamp: '2025-06-21 17:54:09+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verification/aizu-online-judge/dpl_5_d/src/main.rs
  - verification/library-checker/vertex_set_path_composite/src/main.rs
  - verification/library-checker/unionfind_with_potential/src/main.rs
  - verification/library-checker/unionfind_with_potential_non_commutative_group/src/main.rs
  - verification/library-checker/point_set_range_composite/src/main.rs
  - verification/library-checker/range_affine_range_sum/src/main.rs
documentation_of: math/modint/src/lib.rs
layout: document
title: Modint
---

## Description
