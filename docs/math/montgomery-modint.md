---
title: Montgomery Modint
documentation_of: //math/montgomery-modint/src/lib.rs
---

奇数の法 $M$ に対する modint。

## Montgomery form

値 $a$ を $aR \bmod M$ ($R = 2^{64}$) の形で保持する。

加算・減算は $aR \pm bR = (a \pm b)R$ なので通常の加算・減算と同じである。

乗算は $(aR)(bR) = abR^2$ となり $R$ が余分に掛かるため、Montgomery reduction により $R$ を取り除く。

$$
\text{reduce}(abR^2) = abR^2 \cdot R^{-1} = abR \pmod{M}
$$

通常値から Montgomery form への変換は、事前計算した $R^2 \bmod M$ を用いて

$$
\text{reduce}(a \cdot R^2) = aR \pmod{M}
$$

Montgomery form から通常値への変換は

$$
\text{reduce}(aR) = a \pmod{M}
$$

内部値は $[0, 2M)$ の範囲で管理し、Montgomery form から通常値に戻す際に $M$ 以上であれば $M$ を引いて $[0, M)$ に正規化する。

## Montgomery reduction

$v \cdot R^{-1} \bmod M$ ($R = 2^{64}$) を除算なしで計算する。

事前に $M \cdot M^{-1} \equiv 1 \pmod{R}$ を満たす $M^{-1}$ を求めておく。

$v + r \cdot M$ が $R$ で割り切れるような $r$ を見つけることで、$(v + r \cdot M) / R$ はシフト演算で計算できる。
$v + r \cdot M \equiv 0 \pmod{R}$ を $r$ について解くと、

$$
r \equiv -v \cdot M^{-1} \pmod{R}
$$

この $r$ により $v + r \cdot M \equiv 0 \pmod{R}$ が成り立つ。従って、

$$
\text{reduce}(v) = (v + r \cdot M) / R
$$

は整数であり、$r \cdot M \equiv 0 \pmod{M}$ より、$\text{reduce}(v) \equiv v \cdot R^{-1} \pmod{M}$ が成り立つ。

## Reference
- [https://rsk0315.hatenablog.com/entry/2022/11/27/060616](https://rsk0315.hatenablog.com/entry/2022/11/27/060616)
- [https://ei1333.hateblo.jp/entry/2020/01/16/173956](https://ei1333.hateblo.jp/entry/2020/01/16/173956)
- [https://drken1215.hatenablog.com/entry/2023/05/23/233000](https://drken1215.hatenablog.com/entry/2023/05/23/233000)
- [https://andantesoft.hatenablog.com/entry/2025/05/21/164352](https://andantesoft.hatenablog.com/entry/2025/05/21/164352)
- [https://tex2e.github.io/blog/crypto/montgomery-mul](https://tex2e.github.io/blog/crypto/montgomery-mul)

- [https://zenn.dev/herumi/articles/finite-field-01-add](https://zenn.dev/herumi/articles/finite-field-01-add)
- [https://zenn.dev/herumi/articles/finite-field-02-sub](https://zenn.dev/herumi/articles/finite-field-02-sub)
- [https://zenn.dev/herumi/articles/finite-field-03-mul](https://zenn.dev/herumi/articles/finite-field-03-mul)
- [https://zenn.dev/herumi/articles/finite-field-04-mul](https://zenn.dev/herumi/articles/finite-field-04-mul)
