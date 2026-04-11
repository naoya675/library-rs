---
title: Montgomery Modint (64bit)
documentation_of: //math/montgomery-modint-64/src/lib.rs
---

奇数の法 $M$ に対する modint。

## Montgomery form

Montgomery form の構造については [Montgomery Modint]({{ site.baseurl }}/math/montgomery-modint/src/lib.rs) を参照。

## Montgomery reduction + 乗算 (Multiplication)

$x \cdot y \cdot R^{-1} \bmod M$ ($R = 2^{64}$) を 64bit の範囲で計算する。

$x \cdot y$ の上位・下位 64bit をそれぞれ $\text{high}(x \cdot y)$, $\text{low}(x \cdot y)$ と書く。

$v + r \cdot M$ が $R$ で割り切れるような $r$ を見つけることで、$(v + r \cdot M) / R$ はシフト演算で計算できる。
$v = x \cdot y$ として $v + r \cdot M \equiv 0 \pmod{R}$ を $r$ について解くと、

$$
r = \text{low}(x \cdot y) \cdot (-M^{-1}) \bmod R
$$

この $r$ により $v + r \cdot M \equiv 0 \pmod{R}$ が成り立つ。従って、

$$
\text{reduce}(v) = (v + r \cdot M) / R
$$

$v$ と $r \cdot M$ をそれぞれ上位・下位に分解すると、

$$
v + r \cdot M = (\text{high}(v) + \text{high}(r \cdot M)) \cdot R + (\text{low}(v) + \text{low}(r \cdot M))
$$

$\text{low}(v) + \text{low}(r \cdot M) \equiv 0 \pmod{R}$ より、$\text{low}(v) + \text{low}(r \cdot M)$ は $0$ または $R$ である。
$\text{low}(v) \neq 0$ のとき和は $R$ で $\text{carry} = 1$ (繰り上がり)、$\text{low}(v) = 0$ のとき和は $0$ で $\text{carry} = 0$ である。

$R$ で割ると、

$$
\text{reduce}(v) = \text{high}(v) + \text{high}(r \cdot M) + \text{carry}
$$

### $\text{high}$ の計算

$x \cdot y$ の上位 64bit を 64bit の範囲で計算する。
$x, y$ をそれぞれ上位・下位 32bit に分割する。

$$
x = x_u \cdot 2^{32} + x_d, \; y = y_u \cdot 2^{32} + y_d
$$

このとき積は次のように展開される。

$$
x \cdot y = x_u y_u \cdot 2^{64} + (x_u y_d + x_d y_u) \cdot 2^{32} + x_d y_d
$$

各部分積は 32bit $\times$ 32bit = 64bit である。$\text{high}(x \cdot y)$ に直接寄与するのは、

- $x_u y_u$ 全体
- $x_u y_d$ の上位 32bit
- $x_d y_u$ の上位 32bit

さらに、上位 64bit への繰り上がりに寄与する以下の 3 項の和を $\mathit{mid}$ とする。

- $x_u y_d$ の下位 32bit
- $x_d y_u$ の下位 32bit
- $x_d y_d$ の上位 32bit

$\mathit{mid}$ の繰り上がり $\lfloor \mathit{mid} / 2^{32} \rfloor$ が上位 64bit に加算される。従って、

$$
\text{high}(x \cdot y) = x_u y_u + \lfloor x_u y_d / 2^{32} \rfloor + \lfloor x_d y_u / 2^{32} \rfloor + \lfloor \mathit{mid} / 2^{32} \rfloor
% \text{high}(x \cdot y) = x_u y_u + (x_u y_d \gg 32) + (x_d y_u \gg 32) + (\mathit{mid} \gg 32)
$$

### $\text{low}$ の計算

$x \cdot y$ の下位 64bit は乗算 (wrapping multiplication) で直接得られる。

$$
\text{low}(x \cdot y) = x \cdot y \bmod R
$$

## Reference
- [https://rsk0315.hatenablog.com/entry/2022/11/27/060616](https://rsk0315.hatenablog.com/entry/2022/11/27/060616)
- [https://ei1333.hateblo.jp/entry/2020/01/16/173956](https://ei1333.hateblo.jp/entry/2020/01/16/173956)
- [https://drken1215.hatenablog.com/entry/2023/05/23/233000](https://drken1215.hatenablog.com/entry/2023/05/23/233000)
- [https://yu212.hatenablog.com/entry/2023/12/14/203400](https://yu212.hatenablog.com/entry/2023/12/14/203400)
