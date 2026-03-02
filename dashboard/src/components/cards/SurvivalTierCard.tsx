"use client";

const TIERS = [
  { name: "Normal", count: 127, color: "var(--accent-green)", multiplier: "1.0×" },
  { name: "LowCompute", count: 34, color: "var(--accent-orange)", multiplier: "0.5×" },
  { name: "Critical", count: 8, color: "#ef4444", multiplier: "0.1×" },
  { name: "Archived", count: 19, color: "var(--text-secondary)", multiplier: "0×" },
];

export function SurvivalTierCard() {
  const total = TIERS.reduce((s, t) => s + t.count, 0);
  const alive = total - (TIERS.find((t) => t.name === "Archived")?.count ?? 0);

  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Survival Tiers — {alive} alive / {total} total
      </h3>

      {/* Stacked bar */}
      <div className="flex h-6 rounded overflow-hidden mb-4">
        {TIERS.map((tier) => (
          <div
            key={tier.name}
            style={{
              width: `${(tier.count / total) * 100}%`,
              background: tier.color,
              opacity: tier.name === "Archived" ? 0.4 : 1,
            }}
          />
        ))}
      </div>

      {/* Legend */}
      <div className="grid grid-cols-2 gap-2">
        {TIERS.map((tier) => (
          <div
            key={tier.name}
            className="flex items-center justify-between px-2 py-1.5 rounded text-xs"
            style={{ background: "rgba(255,255,255,0.03)" }}
          >
            <div className="flex items-center gap-2">
              <span
                className="w-2.5 h-2.5 rounded-full"
                style={{ background: tier.color }}
              />
              <span>{tier.name}</span>
            </div>
            <div className="flex gap-2" style={{ color: "var(--text-secondary)" }}>
              <span>{tier.count}</span>
              <span style={{ color: tier.color }}>{tier.multiplier}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
