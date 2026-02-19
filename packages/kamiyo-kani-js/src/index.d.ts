export type KaniStatus = "PASS" | "FAIL" | "UNKNOWN";

export interface KaniSummary {
  status: KaniStatus;
  elapsedMs?: number;
  details: string;
}

export function parseKaniSummary(summary: string): KaniSummary;
