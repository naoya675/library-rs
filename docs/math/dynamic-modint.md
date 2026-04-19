---
title: Dynamic Modint
documentation_of: //math/dynamic-modint/src/lib.rs
---

## Barrett reduction

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

さらに、誤差は高々 $1$ であることが示せるので、

$$
x \in \lbrace \lfloor z/m \rfloor, \; \lfloor z/m \rfloor - 1 \rbrace
$$

$v = z - x \cdot m$ とすると、

- $x = \lfloor z/m \rfloor$ のとき $\; v = z \bmod m \in [0, m)$
- $x = \lfloor z/m \rfloor - 1$ のとき $\; v = z \bmod m + m \in [m, 2m)$

いずれの場合も $v \in [0, 2m)$ に収まり、$v \geq m$ なら $m$ を引くことで正しい剰余が得られる。

### 誤差の評価

$\mathit{im} = \lfloor 2^{64} / m \rfloor$ は次のように表せる。

$$
\mathit{im} = \frac{2^{64}}{m} - \varepsilon \quad (0 \leq \varepsilon < 1)
$$

これを代入すると、

$$
x = \left\lfloor \frac{z \cdot \mathit{im}}{2^{64}} \right\rfloor = \left\lfloor \frac{z}{m} - \frac{z\varepsilon}{2^{64}} \right\rfloor
$$

$x$ の上界・下界は以下のように得られる。

(上界) $z\varepsilon / 2^{64} \geq 0$ より、$x \leq \lfloor z / m \rfloor$

(下界) $z\varepsilon / 2^{64} < 1$ より $(z < 2^{62}, \varepsilon < 1)$、$x \geq \lfloor z / m \rfloor - 1$

## Reference
- [https://min-25.hatenablog.com/entry/2017/08/20/171214](https://web.archive.org/web/20211009144445/https://min-25.hatenablog.com/entry/2017/08/20/171214)
- [https://mitarushi.hatenablog.com/entry/2022/02/09/194722](https://mitarushi.hatenablog.com/entry/2022/02/09/194722)
- [https://natsugiri.hatenablog.com/entry/2020/04/06/030559](https://natsugiri.hatenablog.com/entry/2020/04/06/030559)
- [https://qiita.com/AkariLuminous/items/cebb1f15bf482fddd85e](https://qiita.com/AkariLuminous/items/cebb1f15bf482fddd85e)
- [https://rsk0315.hatenablog.com/entry/2021/01/18/065720](https://rsk0315.hatenablog.com/entry/2021/01/18/065720)
- [https://suisen-kyopro.hatenablog.com/entry/2023/09/13/060922](https://suisen-kyopro.hatenablog.com/entry/2023/09/13/060922)
