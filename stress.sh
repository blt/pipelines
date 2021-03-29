#!/usr/bin/env bash

set -o errexit
set -o pipefail
set -o nounset
# set -o xtrace

__dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="${__dir}/target"
PERF_DIR="${TARGET_DIR}/perf"
RESOURCE_BIG_LOG="${__dir}/resources/stable_big.log.gz"

cargo build --release
targets=("baseline" "std-baseline" "pipeline")
mkdir -p "${PERF_DIR}"

if [ ! -f "${RESOURCE_BIG_LOG}" ]; then
    flog --number 1000000000 > "${__dir}/resources/tmp.log"
    gzip "${__dir}/resources/tmp.log"
    mv "${__dir}/resources/tmp.log.gz" "${RESOURCE_BIG_LOG}"
fi

for target in "${targets[@]}" ; do
    BIN="${TARGET_DIR}/release/${target}"
    perf stat --repeat=5 bash -c "zcat ${RESOURCE_BIG_LOG} | ${BIN} > /dev/null"
    zcat "${RESOURCE_BIG_LOG}" | perf record --output="${PERF_DIR}/${target}-perf.data" --call-graph dwarf "${BIN}" > /dev/null

    perf script --input="${PERF_DIR}/${target}-perf.data" | inferno-collapse-perf > "${PERF_DIR}/${target}-stacks.folded"
    inferno-flamegraph < "${PERF_DIR}/${target}-stacks.folded" > "${PERF_DIR}/${target}-flamegraph.svg"
done
