# estieプログラミングコンテスト2022 (AtCoder Heuristic Contest 014)

2022.9/17 15:00 - 2022.10/1 19:00  
Public : 2022.10/1 19:00~
[AHC014](https://atcoder.jp/contests/ahc014/tasks/ahc014_a)  


## 最終結果
score : ■ (2000 testcase)  
ranking : ■ / ■ 人  
performance : ■ (■)  
Rating : 1097 -> ■ (■)

<br>

## Results
[src/main.rs](src/main.rs) : 提出ファイルの差分がまとまったもの (1.rs ~ ■.rs)

### Submitted logs
50 testcase (Rust)

| file | score | time | 何をしたか |
| ---- | ---- | ---- | ---- |
| [1.rs](src/bin/1.rs) | 14,048,321 | 6ms | 一旦何もしない |
| [2.rs](src/bin/2.rs) | 33,271,877 | 512ms | 長方形の4点目として選べるp1を全探索し重みの高い順に採用する。4.0sec試す。 |
| [3.rs](src/bin/3.rs) | 33,271,877 | 497ms | 2.rsを4.9secでお試し。手元では僅かに上がるのにscore少しも変わらない謎。 |
| [4.rs](src/bin/4.rs) | 33,712,711 | 4904ms | 重みが高い順にその周で作れる1番小さい四角を採用。微増…。 |
| [5.rs](src/bin/5.rs) | 33,712,711 | 457ms | 内容特に変更なし、4.rsでもう置ける場所がなかったら終了。 |
| [6.rs](src/bin/6.rs) |  | ms |  |


<br>

### この方針での改善点


<br>

### ビジュアライザのPNG(seed=)
![demo](vis.png)  
