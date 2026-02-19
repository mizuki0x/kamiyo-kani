#!/usr/bin/env bash
set -euo pipefail

out_dir="${BENCH_OUT_DIR:-bench-results}"
mkdir -p "$out_dir"
log="$out_dir/agent-flow.log"
summary="$out_dir/summary.md"

start_ms=$(python3 - <<'PY'
import time
print(int(time.time() * 1000))
PY
)

if cargo kani -p kamiyo-kani --features solana-agent --harness agent::bench::verify_agent_flow_end_to_end --output-format terse >"$log" 2>&1; then
  status="PASS"
else
  status="FAIL"
fi

end_ms=$(python3 - <<'PY'
import time
print(int(time.time() * 1000))
PY
)

elapsed_ms=$((end_ms - start_ms))
threshold_ms=5000

if (( elapsed_ms < threshold_ms )); then
  target="HIT"
else
  target="MISS"
fi

cat > "$summary" <<SUMMARY
# Agent Flow Benchmark

- status: ${status}
- elapsed_ms: ${elapsed_ms}
- target_ms: ${threshold_ms}
- target_result: ${target}
- harness: agent::bench::verify_agent_flow_end_to_end
SUMMARY

echo "elapsed_ms=${elapsed_ms}" > "$out_dir/metrics.env"
echo "target_result=${target}" >> "$out_dir/metrics.env"

test "$status" = "PASS"
