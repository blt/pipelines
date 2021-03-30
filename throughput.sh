#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
#set -o xtrace

__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${__dir}/target"
PERF_DIR="${TARGET_DIR}/perf"

cargo build --release
targets=("std-baseline" "baseline" "pipeline" "streamer")
mkdir -p "${PERF_DIR}"

for target in "${targets[@]}" ; do
    echo -n "${target}: "
    BIN="${TARGET_DIR}/release/${target}"
    yes "Alexey Fyodorovitch Karamazov was the third son of Fyodor Pavlovitch Karamazov, a landowner well known in our district in his own day, and still remembered among us owing to his gloomy and tragic death, which happened thirteen years ago, and which I shall describe in its proper place." | \
        head -n1000000 | \
        "${BIN}" | \
        pv --average-rate > /dev/null && true
    sleep 1
done
