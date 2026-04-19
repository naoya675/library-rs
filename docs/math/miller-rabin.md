---
title: Miller-Rabin
documentation_of: //math/miller-rabin/src/lib.rs
---

Miller-Rabin 素数判定法。指定した $a$ の組に対する決定的判定で素数性を調べる。
<!--Miller-Rabin 素数判定法。指定した witness に対する決定的判定で素数性を調べる。-->

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
最初の $k$ 個の素数を $a$ として用いた場合、次の範囲で決定的である ([A014233](https://oeis.org/A014233))。

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

- [https://miller-rabin.appspot.com/](https://web.archive.org/web/20260225175716/https://miller-rabin.appspot.com/)
- [https://oeis.org/A014233](https://oeis.org/A014233)
- [https://oeis.org/A006945](https://oeis.org/A006945)
- [https://arxiv.org/pdf/1509.00864](https://arxiv.org/pdf/1509.00864)
- [https://www.kurims.kyoto-u.ac.jp/~kyodo/kokyuroku/contents/pdf/1955-17.pdf](https://www.kurims.kyoto-u.ac.jp/~kyodo/kokyuroku/contents/pdf/1955-17.pdf)
- [https://theswissbay.ch/pdf/Gentoomen%20Library/Cryptography/Handbook%20of%20Applied%20Cryptography%20-%20Alfred%20J.%20Menezes.pdf](https://theswissbay.ch/pdf/Gentoomen%20Library/Cryptography/Handbook%20of%20Applied%20Cryptography%20-%20Alfred%20J.%20Menezes.pdf)

## Reference
- [https://37zigen.com/miller-rabin/](https://37zigen.com/miller-rabin/)
- [https://drken1215.hatenablog.com/entry/2023/05/23/233000](https://drken1215.hatenablog.com/entry/2023/05/23/233000)
- [https://tex2e.github.io/blog/crypto/miller-rabin-test](https://tex2e.github.io/blog/crypto/miller-rabin-test)
