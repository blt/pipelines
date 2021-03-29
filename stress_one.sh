#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${__dir}/target"
PERF_DIR="${TARGET_DIR}/perf"
RESOURCE_SMALL_LOG="${__dir}/resources/stable_small.log.gz"

if [ ! -f "${RESOURCE_SMALL_LOG}" ]; then
    flog --number 10000000 > "${__dir}/resources/tmp.log"
    gzip "${__dir}/resources/tmp.log"
    mv "${__dir}/resources/tmp.log.gz" "${RESOURCE_SMALL_LOG}"
fi

cargo build --release
targets=("baseline" "std-baseline" "pipeline")
mkdir -p "${PERF_DIR}"

for target in "${targets[@]}" ; do
    BIN="${TARGET_DIR}/release/${target}"
    zcat "${RESOURCE_SMALL_LOG}" | perf stat bash -c "${BIN} > /dev/null"
done
