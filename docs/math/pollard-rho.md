---
title: Pollard's Rho algorithm
documentation_of: //math/pollard-rho/src/lib.rs
---

Pollard's rho 素因数分解法。
与えられた数を素因数分解する乱択アルゴリズム。
合成数 $n$ の非自明な約数を期待計算量 $O(n^{1/4})$ で発見し、素数判定法 (Miller-Rabin) と組合せて再帰的に素因数へ分解する。

## Reference
- John M. Pollard, ["A Monte Carlo method for factorization"](https://doi.org/10.1007/BF01933667), BIT vol. 15(3) 1975, pp. 331-334.
<!--- [https://lpha-z.hatenablog.com/entry/2023/01/15/231500](https://lpha-z.hatenablog.com/entry/2023/01/15/231500)-->
- [https://manabitimes.jp/math/1192](https://manabitimes.jp/math/1192)
- [https://qiita.com/Kiri8128/items/eca965fe86ea5f4cbb98](https://qiita.com/Kiri8128/items/eca965fe86ea5f4cbb98)
- [https://qiita.com/t_fuki/items/7cd50de54d3c5d063b4a](https://qiita.com/t_fuki/items/7cd50de54d3c5d063b4a)
