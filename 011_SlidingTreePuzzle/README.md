# AtCoder Heuristic Contest 011

2022.5/28 12:00 - 2022.6/5 19:00  
Public : 2022.6/5 19:00~
[AHC011](https://atcoder.jp/contests/ahc011/tasks/ahc011_a)  


## 最終結果
score : 599,884,256 (3000 testcase)  
ranking : 454 / 961 人  
performance : 1183 (緑)  
Rating : 1051 -> 1097 (緑)

<br>

## Results
[sliding-tree-puzzle/src/main.rs](sliding-tree-puzzle/src/main.rs) : 提出ファイルの差分がまとまったもの (1.rs ~ 6.rs)

### Submitted logs
50 testcase (Rust)

| file | score | time | 何をしたか |
| ---- | ---- | ---- | ---- |
| [1.rs](sliding-tree-puzzle/src/bin/1.rs) | 2,869,199 | 4ms | tターン何もしない |
| [2.rs](sliding-tree-puzzle/src/bin/2.rs) | 4,656,724 | 38ms | tターンrandomで判定に問題ないかcheck |
| [3.rs](sliding-tree-puzzle/src/bin/3.rs) | 8,257,596 | 38ms | プラモデル風に作ろうとするもバグが取れないまま一旦提出 |
| [4.rs](sliding-tree-puzzle/src/bin/4.rs) | 10,254,736 | 40ms | 3.rsのバグを修正。やっと少し狙い通りに動いていそう。 |
| [5.rs](sliding-tree-puzzle/src/bin/5.rs) | 7,781,948 | 42ms | 4.rsに最短距離の目的tileを採用を追加したがどこかがおかしい。 |
| [4.rs](sliding-tree-puzzle/src/bin/4.rs) | 9,737,079 | 44ms | 修正する時間がなく4.rsを最終提出として終了 |

<br>

### この方針での改善点
・簡単のため今いるマスの右下領域からしかタイルを探していないので、全体から最も近いタイルを持って来られると良い  
・簡単のため毎回目的のマスに戻ってからタイルとの交換に向かっているのでターン数に無駄がとても多い  
・どこかに恐らくバグがある

<br>

### ビジュアライザのPNG(seed=4)
![demo](sliding-tree-puzzle/vis.png)  
GIF(少し遅れて表示)  
![demo](sliding-tree-puzzle/vis.gif)