# 複数のCSVをひとつにつなげる

## 注意

Rust 勉強中の自習課題です。

クローン、使用は自己責任でお願い致します。

## 仕様

まずは素朴な実装から考える。

- コマンドライン引数につなげたいCSVのパスを並べる
- 出力は標準出力のみ

sample : tests/members.csv
```csv
id,name,age
10-1,John,30
10-2,Ken,33
15-0,Bob,18
n-09,Jen,48
```
sample : tests/members-2.csv
```csv
id,name,age
210-1,John,30
210-2,Ken,33
215-0,Bob,18
2n-09,Jen,48
```

```sh
cargo run -- tests/members*.csv >one.csv
```
このコマンドラインの出力は下記のようになる。

sample : one.csv
```csv
id,name,age
10-1,John,30
10-2,Ken,33
15-0,Bob,18
n-09,Jen,48
210-1,John,30
210-2,Ken,33
215-0,Bob,18
2n-09,Jen,48
```

ヘッダ行は先頭行にだけ登場する

## 参考

- [Crate csv](https://docs.rs/csv/latest/csv/)
- [Rust By Example 日本語版 - open](https://doc.rust-jp.rs/rust-by-example-ja/std_misc/file/open.html)

