#!/usr/bin/env python3
import json
import re
import sys
from pathlib import Path


def read_status(summary_path: Path) -> str:
    if not summary_path.exists():
        return "UNKNOWN"
    text = summary_path.read_text(encoding="utf-8", errors="replace")
    m = re.search(r"status:\s*(\w+)", text)
    return m.group(1).upper() if m else "UNKNOWN"


def extract_failures(log_path: Path):
    if not log_path.exists():
        return []
    failures = []
    for line in log_path.read_text(encoding="utf-8", errors="replace").splitlines():
        if "FAILED" in line and "proof" in line.lower():
            failures.append(line.strip())
        elif "failed" in line.lower() and "harness" in line.lower():
            failures.append(line.strip())
    seen = set()
    uniq = []
    for f in failures:
        if f not in seen:
            uniq.append(f)
            seen.add(f)
    return uniq


def build_sarif(status: str, failures, log_path: Path):
    rules = [
        {
            "id": "kani.verification.failure",
            "name": "KaniVerificationFailure",
            "shortDescription": {"text": "Kani proof failed"},
            "fullDescription": {
                "text": "At least one proof harness failed during formal verification."
            },
            "helpUri": "https://github.com/model-checking/kani",
        }
    ]

    results = []
    if status == "FAIL":
        if not failures:
            failures = [f"Verification failed. See log: {log_path}"]
        for failure in failures:
            results.append(
                {
                    "ruleId": "kani.verification.failure",
                    "level": "error",
                    "message": {"text": failure},
                }
            )

    return {
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [
            {
                "tool": {
                    "driver": {
                        "name": "Kani",
                        "informationUri": "https://github.com/model-checking/kani",
                        "rules": rules,
                    }
                },
                "results": results,
            }
        ],
    }


def main():
    if len(sys.argv) != 4:
        print("usage: kani-sarif.py <summary.md> <kani.log> <output.sarif>", file=sys.stderr)
        return 2

    summary_path = Path(sys.argv[1])
    log_path = Path(sys.argv[2])
    output_path = Path(sys.argv[3])

    status = read_status(summary_path)
    failures = extract_failures(log_path)
    sarif = build_sarif(status, failures, log_path)

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(sarif, indent=2) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
