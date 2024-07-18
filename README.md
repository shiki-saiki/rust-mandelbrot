# マンデルブロ集合

マンデルブロ集合の探索アプリです。

シェーダーを使って各ピクセルの値を計算しています。

参照: [Wikipedia](https://ja.wikipedia.org/wiki/%E3%83%9E%E3%83%B3%E3%83%87%E3%83%AB%E3%83%96%E3%83%AD%E9%9B%86%E5%90%88)

[Bevy](https://bevyengine.org/)で作りました。

製作期間: 約2週間

## セットアップ

```
git clone https://github.com/shiki-saiki/rust-mandelbrot.git
cd rust-mandelbrot
cargo run
```

## 操作方法

- W: ズームイン
- S: ズームアウト
- A: 計算回数減少
- D: 計算回数増加
- 矢印キー: 移動

![zoom_in.gif](/zoom_in.gif)
![iteration.gif](/iteration.gif)
