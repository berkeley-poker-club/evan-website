#!/bin/bash
set -e

OCF_USER="pokeratberkeley"
OCF_HOST="ssh.ocf.berkeley.edu"

SKIP_BENCHMARK=false
for arg in "$@"; do
    case $arg in
        --skip-benchmark|--force) SKIP_BENCHMARK=true ;;
    esac
done

npm run build

if [ "$SKIP_BENCHMARK" = false ]; then
    echo "Running benchmarks..."
    ./scripts/benchmark.sh
fi

rsync -avz --delete --exclude='.DS_Store' --exclude='*.map' dist/ $OCF_USER@$OCF_HOST:~/public_html/

echo "Done: https://poker.studentorg.berkeley.edu"
