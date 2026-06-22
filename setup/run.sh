#!/bin/sh
set -e

INPUT="${1:-setup/hello.deor}"
OUT_RS="/tmp/_deor_out.rs"
OUT_BIN="/tmp/_deor_run"

deor "$INPUT" "$OUT_RS"
rustc -O -A warnings "$OUT_RS" -o "$OUT_BIN"
"$OUT_BIN"
