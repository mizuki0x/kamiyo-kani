export type KaniStatus = "PASS" | "FAIL" | "UNKNOWN";

export interface KaniSummary {
  status: KaniStatus;
  elapsedMs?: number;
  details: string;
}

// Lightweight parser for kani summary markdown artifacts.
export function parseKaniSummary(summary: string): KaniSummary {
  const statusMatch = summary.match(/status:\s*(\w+)/i);
  const elapsedMatch = summary.match(/elapsed_ms:\s*(\d+)/i);

  const status = (statusMatch?.[1]?.toUpperCase() as KaniStatus) ?? "UNKNOWN";
  const elapsedMs = elapsedMatch ? Number(elapsedMatch[1]) : undefined;

  return {
    status,
    elapsedMs,
    details: summary,
  };
}
