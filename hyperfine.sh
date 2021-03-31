#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${__dir}/target"
PERF_DIR="${TARGET_DIR}/perf"
RESOURCE_LOG="${__dir}/resources/stable_big.log.gz"

if [ ! -f "${RESOURCE_LOG}" ]; then
    flog --number 1000000000 > "${__dir}/resources/tmp.log"
    gzip "${__dir}/resources/tmp.log"
    mv "${__dir}/resources/tmp.log.gz" "${RESOURCE_LOG}"
fi

cargo build --release
mkdir -p "${PERF_DIR}"

hyperfine \
    "zcat ${RESOURCE_LOG} | ${TARGET_DIR}/release/baseline > /dev/null" \
    "zcat ${RESOURCE_LOG} | ${TARGET_DIR}/release/std-baseline > /dev/null" \
    "zcat ${RESOURCE_LOG} | ${TARGET_DIR}/release/pipeline > /dev/null"
