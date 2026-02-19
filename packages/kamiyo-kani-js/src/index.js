/**
 * @typedef {"PASS" | "FAIL" | "UNKNOWN"} KaniStatus
 */

/**
 * @typedef {{ status: KaniStatus, elapsedMs?: number, details: string }} KaniSummary
 */

/**
 * Lightweight parser for Kani summary markdown artifacts.
 * @param {string} summary
 * @returns {KaniSummary}
 */
export function parseKaniSummary(summary) {
  const statusMatch = summary.match(/status:\s*(\w+)/i);
  const elapsedMatch = summary.match(/elapsed_ms:\s*(\d+)/i);

  const statusRaw = statusMatch?.[1]?.toUpperCase() ?? "UNKNOWN";
  const status =
    statusRaw === "PASS" || statusRaw === "FAIL" || statusRaw === "UNKNOWN"
      ? statusRaw
      : "UNKNOWN";

  return {
    status,
    elapsedMs: elapsedMatch ? Number(elapsedMatch[1]) : undefined,
    details: summary,
  };
}
