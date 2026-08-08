---
title: Wavelet Matrix
documentation_of: //data-structure/wavelet-matrix/src/lib.rs
---

A data structure that stores a sequence of non-negative integers in $[0, \sigma)$ as a stack of $\lceil \log \sigma \rceil$ bit vectors partitioned by bit position from MSB to LSB, where $\sigma$ is the alphabet size.
Supports `access`, `rank`, and `kth_smallest` queries in $O(\log \sigma)$ time and `select` in $O(\log \sigma \cdot \log n)$ time, where $n$ is the sequence length.

## Reference
- Francisco Claude, Gonzalo Navarro and Alberto Ordóñez, ["The wavelet matrix: An efficient wavelet tree for large alphabets"](https://doi.org/10.1016/j.is.2014.06.002), Information Systems vol. 47 2015, pp. 15-32.
<!--- Robinson Castro, Nico Lehmann, Jorge Pérez and Bernardo Subercaseaux, ["Wavelet Trees for Competitive Programming"](https://doi.org/10.15388/ioi.2016.02), Olympiads in Informatics vol. 10 2016, pp. 19-37.-->
- [https://info.atcoder.jp/entry/algorithm_lectures/wavelet_matrix_basic](https://info.atcoder.jp/entry/algorithm_lectures/wavelet_matrix_basic)
- [https://info.atcoder.jp/entry/algorithm_lectures/wavelet_matrix_advanced](https://info.atcoder.jp/entry/algorithm_lectures/wavelet_matrix_advanced)
<!--- [http://algoogle.hadrori.jp/algorithm/wavelet.html](https://web.archive.org/web/20210614003021/http://algoogle.hadrori.jp/algorithm/wavelet.html)-->
<!--- [https://blog.hamayanhamayan.com/entry/2017/06/13/103352](https://blog.hamayanhamayan.com/entry/2017/06/13/103352)-->
<!--- [https://echizen-tm.hatenadiary.org/entry/20120801/1343837130](https://echizen-tm.hatenadiary.org/entry/20120801/1343837130)-->
- [https://miti-7.hatenablog.com/entry/2018/04/28/152259](https://miti-7.hatenablog.com/entry/2018/04/28/152259)
- [https://miti-7.hatenablog.com/entry/2019/02/01/143655](https://miti-7.hatenablog.com/entry/2019/02/01/143655)
- [https://qiita.com/convexineq/items/70f36585ae76f70e3abf](https://qiita.com/convexineq/items/70f36585ae76f70e3abf)
- [https://qiita.com/vi_24E/items/85ce398bc8e4b4d28d56](https://qiita.com/vi_24E/items/85ce398bc8e4b4d28d56)
- [https://takeda25.hatenablog.jp/entry/20120804/1344066875](https://takeda25.hatenablog.jp/entry/20120804/1344066875)
- [https://takeda25.hatenablog.jp/entry/20120806/1344245743](https://takeda25.hatenablog.jp/entry/20120806/1344245743)
- [https://takeda25.hatenablog.jp/entry/20120807/1344306670](https://takeda25.hatenablog.jp/entry/20120807/1344306670)
- [https://takeda25.hatenablog.jp/entry/20120807/1344336237](https://takeda25.hatenablog.jp/entry/20120807/1344336237)
- [https://takeda25.hatenablog.jp/entry/20120820/1345455442](https://takeda25.hatenablog.jp/entry/20120820/1345455442)
- [https://takeda25.hatenablog.jp/entry/20130303/1362301095](https://takeda25.hatenablog.jp/entry/20130303/1362301095)
- [https://takeda25.hatenablog.jp/entry/20130505/1367730870](https://takeda25.hatenablog.jp/entry/20130505/1367730870)
<!--- [https://www.scribd.com/doc/102636443/Wavelet-Matrix](https://www.scribd.com/doc/102636443/Wavelet-Matrix)-->
<!--- [https://www.slideshare.net/slideshow/the-wavelet-matrix/62929724](https://www.slideshare.net/slideshow/the-wavelet-matrix/62929724)-->

- k-th Smallest
    - [https://rsk0315.hatenablog.com/entry/2022/01/09/152028](https://rsk0315.hatenablog.com/entry/2022/01/09/152028)
    - [https://sune2.hatenadiary.org/entry/20131216/1387197255](https://sune2.hatenadiary.org/entry/20131216/1387197255)
    - [https://atcoder.jp/contests/adt_hard_20240424_3/editorial/3438](https://atcoder.jp/contests/adt_hard_20240424_3/editorial/3438)

<!--- [https://github.com/MitI-7/WaveletMatrix](https://github.com/MitI-7/WaveletMatrix)-->
