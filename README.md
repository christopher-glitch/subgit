# subgit

## 実装コマンド

* init</br>
  subgitの初期化
* hash-object</br>
  ファイルをハッシュ化してblobオブジェクトにする
* cat-file</br>
  ./.subgit/object内のblobオブジェクトのファイルの中身を見る
* update-index</br>
  ファイルをステージングする(indexファイルに追加する)
* ls-files</br>
  ステージングしたファイルの中身を見る

## 仕様

* indexファイルではファイルごとに作成時間，変更時間，モード，ファイルの大きさ，ハッシュ値，名前の大きさ，名前のみを格納しており，gitの全ての仕様に従わない

## 参考

* Qiita Gitを作ってみる</br>
  理解編　<https://qiita.com/noshishi/items/60a6fe7c63097950911b> </br>
  開発編 <https://qiita.com/noshishi/items/823bc46d971ac1fe8215>

* Zenn Rustで自作Git作った話</br>
  <https://zenn.dev/garebare/articles/8bd5d66003dd6e>
