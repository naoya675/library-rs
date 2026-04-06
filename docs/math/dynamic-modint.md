---
title: Dynamic Modint
documentation_of: //math/dynamic-modint/src/lib.rs
---

## 乗算 (Barrett reduction)

事前に次の定数を計算しておく。

$$
\mathit{im} = \left\lfloor \frac{2^{64}}{m} \right\rfloor
$$

$a, b < m < 2^{31}$ について $z = a \cdot b$ とする。
$m < 2^{31}$ より $z < 2^{62}$ で 64bit に収まる。

$z / m$ の近似値 $x$ を次で計算する。

$$
x = \left\lfloor \frac{z \cdot \mathit{im}}{2^{64}} \right\rfloor
$$

$\mathit{im} \leq 2^{64} / m$ より、

$$
z \cdot \mathit{im} \leq z \cdot \frac{2^{64}}{m}
$$

両辺を $2^{64}$ で割って床関数を取ると、

$$
x = \left\lfloor \frac{z \cdot \mathit{im}}{2^{64}} \right\rfloor \leq \left\lfloor \frac{z}{m} \right\rfloor
$$

さらに、誤差は高々 $1$ であることが示せるので [[2]](https://github.com/atcoder/ac-library/blob/master/atcoder/internal_math.hpp)、

$$
x \in \lbrace \lfloor z/m \rfloor, \; \lfloor z/m \rfloor - 1 \rbrace
$$

$v = z - x \cdot m$ とすると、

- $x = \lfloor z/m \rfloor$ のとき $\; v = z \bmod m \in [0, m)$
- $x = \lfloor z/m \rfloor - 1$ のとき $\; v = z \bmod m + m \in [m, 2m)$

いずれの場合も $v \in [0, 2m)$ に収まり、$v \geq m$ なら $m$ を引くことで正しい剰余が得られる。

## Reference
- [https://rsk0315.hatenablog.com/entry/2021/01/18/065720](https://rsk0315.hatenablog.com/entry/2021/01/18/065720)
- [https://github.com/atcoder/ac-library/blob/master/atcoder/internal_math.hpp](https://github.com/atcoder/ac-library/blob/master/atcoder/internal_math.hpp)
