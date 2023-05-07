#!/bin/bash

set -eu

target_branch="topic-mcts-solver"

git ch for-profile
git rebase ${target_branch}

./tool/trim-input-from-runner-log.py "./profiler/in/input.txt" "./profiler/out/output.txt"

docker build -f profiler/Dockerfile  -t codingame-flamegraph:latest .
docker run \
    --security-opt seccomp=./profiler/my_seccomp.json \
    -w /app \
    codingame-flamegraph \
    bash -c "cargo flamegraph -- cargo run -q --release < out/output.txt &> /dev/null"

cp ./profiler/flamegraph.svg ./profiler/flamegraph.old.svg
docker cp $(docker ps -aq -n 1):/app/flamegraph.svg ./profiler
open ./profiler/flamegraph.svg

git ch ${target_branch}
