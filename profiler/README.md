[flamegraph](https://github.com/flamegraph-rs/flamegraph)を使う。

# 実行手順
プロジェクトルートディレクトリでコマンドを叩く。

## Dockerイメージのビルド
```sh
$ docker build -f profiler/Dockerfile  -t codingame-flamegraph:latest .
```

### hint
使わないライブラリは Cargo.toml からコメントアウトしておいた方が良さそう。

## 標準入力で直接やりとり
```sh
docker run \
    --security-opt seccomp=./profiler/my_seccomp.json \
    -w /app \
    codingame-flamegraph \
    bash -c "cargo flamegraph -- cargo run -q --release < out/output.txt &> /dev/null"
```

コンテナ内に生えた `flamegraph.svg` をホストマシンにコピーする。

```sh
container_id="$(docker ps -aq -n 1)"
docker cp ${container_id}:/app/flamegraph.svg /path/to/host/dir
```

# ログイン
```sh
docker run --rm --user "$(id -u)":"$(id -g)" \
    --security-opt seccomp=./profiler/my_seccomp.json \
    -w /app \
    -it codingame-flamegraph
```

# ブラウザ実行して入力データを取る
```sh
git ch for-profile
git rebase master
cd ..
./submit.sh
```

提出結果をブラウザアリーナで実行。
標準エラー出力結果を `profiler/in/input.txt` に貼り付け。

```sh
./tool/trim-input-from-runner-log.py "./profiler/in/input.txt" "./profiler/out/output.txt"
cat "./profiler/out/output.txt" | pbcopy
```

pbcopy の内容をプロファイラに食わせる。
