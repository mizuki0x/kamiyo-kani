#!/usr/bin/env bash
set -euo pipefail

out_dir="${KANI_OUT_DIR:-kani-results}"
mkdir -p "$out_dir"
log="$out_dir/kani.log"

if ./scripts/kani.sh 2>&1 | tee "$log"; then
  status="PASS"
else
  status="FAIL"
fi

cat > "$out_dir/summary.md" <<SUMMARY
# Kani Summary

- crate: kamiyo-kani
- status: ${status}
- log: ${log}
SUMMARY

[[ "$status" == "PASS" ]]
