# subgit

## コマンドの実行方法

cargo run (コマンド名) (引数)

## 実装コマンド

* init</br>
  subgitの初期化
* hash-object [file_path] </br>
  ファイルをハッシュ化してblobオブジェクトにする．オブジェクトのhashを表示する．
* cat-file [blob_hash]</br>
  ./.subgit/object内のblobオブジェクトのファイルの中身を見る
* update-index [file_path]</br>
  ファイルをステージングする(indexファイルに追加する)
* ls-files</br>
  ステージングしたファイルの中身を見る
* add [file_path]</br>
  gitのaddと大体同じ挙動．objectファイルを作成し，indexファイルに追加する．
* write-tree</br>
  treeオブジェクトを作り，hashを表示する．
* commit-tree [tree_hash] [message]</br>
  treeオブジェクトを子としたcommitオブジェクトを作り，hashを表示する．
* update-refs</br>
  refs/mainにcommitのhash値を書き込んで更新する
* commit [message] </br>
  gitのcommitと大体同じ挙動．tree，commitオブジェクトの作成とrefsの更新．
  
## 仕様

* indexファイルではファイルごとに作成時間，変更時間，モード，ファイルの大きさ，ハッシュ値，名前の大きさ，名前のみを格納しており，gitの全ての仕様に従わない
* ディレクトリの入れ子に対応していない(プロジェクトフォルダ直下のファイルしかaddとcommitできない)

## 参考

* Qiita Gitを作ってみる</br>
  理解編　<https://qiita.com/noshishi/items/60a6fe7c63097950911b> </br>
  開発編 <https://qiita.com/noshishi/items/823bc46d971ac1fe8215>

* Zenn Rustで自作Git作った話</br>
  <https://zenn.dev/garebare/articles/8bd5d66003dd6e>
