---
title: Miller-Rabin (Miller-Rabin primality test)
documentation_of: //math/miller-rabin/src/lib.rs
---

Miller-Rabin 素数判定法。
与えられた数が素数かどうかを判定する乱択アルゴリズム。

## 原理

Fermat の小定理より、$n$ が素数で $a$ が $n$ の倍数でない整数ならば、

$$
a^{n-1} \equiv 1 \pmod{n}
$$

が成り立つ。$1$ を右辺に移項して因数分解する。$n - 1 = 2^s \cdot d$ ($d$ は奇数) と表すと、

$$
a^{n-1} - 1 = (a^d - 1)(a^d + 1)(a^{2d} + 1)(a^{4d} + 1) \cdots (a^{2^{s-1} \cdot d} + 1)
$$

この積が $n$ の倍数であるから、$n$ が素数ならば少なくとも 1 つの因数が $n$ の倍数となる。つまり、

- $a^d \equiv 1 \pmod{n}$
- $a^d \equiv -1 \pmod{n}$
- $a^{2d} \equiv -1 \pmod{n}$
- $a^{4d} \equiv -1 \pmod{n}$
- $\ldots$
- $a^{2^{s-1} \cdot d} \equiv -1 \pmod{n}$
<!--- ある $0 \leq r < s$ について $a^{2^r \cdot d} \equiv -1 \pmod{n}$-->

のいずれかが成り立つ。逆にいずれも成り立たないような $a$ が見つかれば $n$ は合成数である。
<!--このような $a$ を **witness** と呼ぶ。-->

$n$ が奇数の合成数のとき、$a$ をランダムに選ぶと $3/4$ 以上の確率で $n$ を合成数と判定できる。よって $k$ 個のランダムな $a$ でテストすれば、合成数を素数と誤判定する確率は $(1/4)^k$ 以下になる。

## 決定的判定

$n$ が十分小さいときは、$a$ を適切に選ぶことで決定的アルゴリズムとなる。
$k$ 個の素数を $a$ として用いた場合、次の範囲で決定的である ([A014233](https://oeis.org/A014233))。

| $a$ の組 | $n$ の範囲 |
|---|---|
| $\lbrace 2, 3 \rbrace$ | $n < 1{,}373{,}653$ |
| $\lbrace 2, 3, 5 \rbrace$ | $n < 25{,}326{,}001$ |
| $\lbrace 2, 3, 5, 7 \rbrace$ | $n < 3{,}215{,}031{,}751$ |
| $\lbrace 2, 3, 5, 7, 11 \rbrace$ | $n < 2{,}152{,}302{,}898{,}747$ |
| $\lbrace 2, 3, 5, 7, 11, 13, 17 \rbrace$ | $n < 341{,}550{,}071{,}728{,}321$ |
| $\lbrace 2, 3, 5, 7, 11, 13, 17, 19, 23 \rbrace$ | $n < 3{,}825{,}123{,}056{,}546{,}413{,}051$ |
| $\lbrace 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37 \rbrace$ | $n < 318{,}665{,}857{,}834{,}031{,}151{,}167{,}461$ ($> 2^{64}$) |

さらに効率的な $a$ の組として以下が知られている。

| $a$ の組 | $n$ の範囲 |
|---|---|
| $\lbrace 2, 7, 61 \rbrace$ | $n < 4{,}759{,}123{,}141$ |
| $\lbrace 2, 325, 9375, 28178, 450775, 9780504, 1795265022 \rbrace$ | $n < 2^{64}$ |

- Jonathan P. Sorenson and Jonathan Webster, ["Strong pseudoprimes to twelve prime bases"](https://doi.org/10.1090/mcom/3134), Mathematics of Computation vol. 86(304) 2017, pp. 985-1003.
<!--- Jonathan P. Sorenson and Jonathan Webster, ["Strong pseudoprimes to twelve prime bases"](https://doi.org/10.48550/arXiv.1509.00864), Mathematics of Computation vol. 86(304) 2017, pp. 985-1003.-->
- [https://miller-rabin.appspot.com/](https://web.archive.org/web/20260225175716/https://miller-rabin.appspot.com/)
- [https://oeis.org/A006945](https://oeis.org/A006945)
- [https://oeis.org/A014233](https://oeis.org/A014233)
- [https://t5k.org/prove/prove2_3.html](https://t5k.org/prove/prove2_3.html)
- [https://theswissbay.ch/pdf/Gentoomen Library/Cryptography/Handbook of Applied Cryptography - Alfred J. Menezes.pdf](https://theswissbay.ch/pdf/Gentoomen Library/Cryptography/Handbook of Applied Cryptography - Alfred J. Menezes.pdf)
- [https://www.kurims.kyoto-u.ac.jp/~kyodo/kokyuroku/contents/pdf/1955-17.pdf](https://www.kurims.kyoto-u.ac.jp/~kyodo/kokyuroku/contents/pdf/1955-17.pdf)

## Reference
- Gary L. Miller, ["Riemann's hypothesis and tests for primality"](https://doi.org/10.1016/S0022-0000(76)80043-8), Journal of Computer and System Sciences vol. 13(3) 1976, pp. 300-317.
- Michael O. Rabin, ["Probabilistic algorithm for testing primality"](https://doi.org/10.1016/0022-314X(80)90084-0), Journal of Number Theory vol. 12(1) 1980, pp. 128-138.
- [https://37zigen.com/miller-rabin/](https://web.archive.org/web/20251206060534/https://37zigen.com/miller-rabin/)
- [https://drken1215.hatenablog.com/entry/2023/05/23/233000](https://drken1215.hatenablog.com/entry/2023/05/23/233000)
- [https://qiita.com/srtk86/items/609737d50c9ef5f5dc59](https://qiita.com/srtk86/items/609737d50c9ef5f5dc59)
- [https://tex2e.github.io/blog/crypto/miller-rabin-test](https://tex2e.github.io/blog/crypto/miller-rabin-test)
