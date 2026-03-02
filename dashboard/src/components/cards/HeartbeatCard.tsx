"use client";

const CONSTITUTION_LAWS = [
  { id: 1, name: "Conservation", status: "satisfied", checks: 1842, violations: 0 },
  { id: 2, name: "Earned Existence", status: "satisfied", checks: 1842, violations: 3 },
  { id: 3, name: "Provenance", status: "satisfied", checks: 1842, violations: 0 },
];

const RECENT_EVENTS = [
  { type: "HeartbeatCheck", process: "H1-Richards-v3", tier: "Normal", time: "2s ago" },
  { type: "SurvivalTierChange", process: "B0-BigLeafC-v1", tier: "LowCompute → Critical", time: "18s ago" },
  { type: "Replication", process: "F1-Rothermel-v7 → v8", tier: "Mutation", time: "45s ago" },
  { type: "StagnationDetected", process: "E0-CohortMosaic-v2", tier: "Warning", time: "1m ago" },
  { type: "ProcessArchived", process: "A1-WRF-v1", tier: "Archived", time: "3m ago" },
];

function lawColor(status: string) {
  return status === "satisfied" ? "var(--accent-green)" : "#ef4444";
}

function eventIcon(type: string) {
  switch (type) {
    case "HeartbeatCheck": return "💓";
    case "SurvivalTierChange": return "📊";
    case "Replication": return "🧬";
    case "StagnationDetected": return "⏸️";
    case "ProcessArchived": return "📦";
    default: return "📋";
  }
}

export function HeartbeatCard() {
  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Heartbeat & Constitution
      </h3>

      {/* Constitutional invariants */}
      <div className="space-y-1.5 mb-4">
        {CONSTITUTION_LAWS.map((law) => (
          <div
            key={law.id}
            className="flex items-center justify-between px-2 py-1.5 rounded text-xs"
            style={{ background: "rgba(255,255,255,0.03)" }}
          >
            <div className="flex items-center gap-2">
              <span style={{ color: lawColor(law.status) }}>
                {law.status === "satisfied" ? "✓" : "✗"}
              </span>
              <span>
                Law {law.id}: {law.name}
              </span>
            </div>
            <span style={{ color: "var(--text-secondary)" }}>
              {law.violations}/{law.checks}
            </span>
          </div>
        ))}
      </div>

      {/* Recent ALife events */}
      <div className="text-xs font-semibold uppercase tracking-wider mb-2" style={{ color: "var(--text-secondary)" }}>
        Recent Events
      </div>
      <div className="space-y-1 max-h-36 overflow-auto">
        {RECENT_EVENTS.map((evt, i) => (
          <div
            key={i}
            className="flex items-center justify-between px-2 py-1 rounded text-xs"
            style={{ background: "rgba(255,255,255,0.03)" }}
          >
            <div className="flex items-center gap-2 truncate">
              <span>{eventIcon(evt.type)}</span>
              <span className="truncate">{evt.process}</span>
            </div>
            <span style={{ color: "var(--text-secondary)" }}>{evt.time}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
