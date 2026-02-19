#!/usr/bin/env bash
set -euo pipefail

out_dir="${KANI_OUT_DIR:-kani-results}"
mkdir -p "$out_dir"
log="$out_dir/kani.log"
sarif="$out_dir/kani.sarif"
summary="$out_dir/summary.md"

if ./scripts/kani.sh 2>&1 | tee "$log"; then
  status="PASS"
else
  status="FAIL"
fi

cat > "$summary" <<SUMMARY
# Kani Summary

- crate: kamiyo-kani
- status: ${status}
- log: ${log}
SUMMARY

./scripts/kani-sarif.py "$summary" "$log" "$sarif"

[[ "$status" == "PASS" ]]
