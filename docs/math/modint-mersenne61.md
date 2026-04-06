---
title: Modint ($\mathbb{F}_{2^{61}-1}$)
documentation_of: //math/modint-mersenne61/src/lib.rs
---

メルセンヌ素数 $M = 2^{61}-1$ を法とする modint。

## 乗算 (Multiplication)

$a, b < 2^{61}-1$ について $a \cdot b \bmod (2^{61}-1)$ を 64bit の範囲で計算する。

$a, b$ をそれぞれ上位 30 ビットと下位 31 ビットに分割する。

$$
a = a_u \cdot 2^{31} + a_d, \; b = b_u \cdot 2^{31} + b_d \; (a_u, b_u < 2^{30}, \; a_d, b_d < 2^{31})
$$

このとき積は次のように展開される。

$$
a \cdot b = a_u b_u \cdot 2^{62} + (a_d b_u + a_u b_d) \cdot 2^{31} + a_d b_d
$$

中央項を $\mathit{mid} = a_d b_u + a_u b_d$ と置き、$\mathit{mid} \; (\mathit{mid} < 2^{62})$ を上位 32 ビットと下位 30 ビットに分割する。

$$
\mathit{mid} = m_u \cdot 2^{30} + m_d
$$

$2^{61} \equiv 1 \pmod{2^{61}-1}$ より、

$$
\begin{aligned}
a \cdot b
&\equiv a_u b_u \cdot 2^{62} + \mathit{mid} \cdot 2^{31} + a_d b_d \\
&\equiv 2 \, a_u b_u \cdot 2^{61} + (m_u \cdot 2^{30} + m_d) \cdot 2^{31} + a_d b_d \\
&\equiv 2 \, a_u b_u + m_u + m_d \cdot 2^{31} + a_d b_d \pmod{2^{61}-1}
\end{aligned}
$$

得られた値 $v$ について $v = v_u \cdot 2^{61} + v_d$ と分割すると、$2^{61} \equiv 1$ より

$$
v \equiv v_u + v_d \pmod{2^{61}-1}
$$

最後に結果が $2^{61}-1$ 以上であれば $2^{61}-1$ を引いて正規化する。

## Reference
- [https://qiita.com/keymoon/items/11fac5627672a6d6a9f6](https://qiita.com/keymoon/items/11fac5627672a6d6a9f6)
