"use client";

const AGENTS = [
  { name: "KB Retrieval", status: "idle", icon: "📚" },
  { name: "Assembly", status: "active", icon: "🔧" },
  { name: "Closure Validator", status: "idle", icon: "✅" },
  { name: "Benchmarking", status: "running", icon: "📏" },
  { name: "Selection", status: "idle", icon: "🎯" },
  { name: "Optimizer", status: "idle", icon: "⚡" },
  { name: "Discovery", status: "idle", icon: "🔬" },
  { name: "Data Scout", status: "idle", icon: "📡" },
  { name: "Salient Dynamics", status: "active", icon: "🌊" },
  { name: "Regime Detector", status: "idle", icon: "🌍" },
  { name: "Runtime Sentinel", status: "watching", icon: "🛡️" },
  { name: "Meta Learner", status: "idle", icon: "🧠" },
  { name: "Foundation Model", status: "active", icon: "🏗️" },
  { name: "Autonomous Observation", status: "watching", icon: "🛰️" },
  { name: "Process Evolution", status: "running", icon: "🧬" },
];

function statusColor(status: string) {
  switch (status) {
    case "active": return "var(--accent-green)";
    case "running": return "var(--accent-blue)";
    case "watching": return "var(--accent-orange)";
    default: return "var(--text-secondary)";
  }
}

export function AgentStatusCard() {
  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Agent Swarm ({AGENTS.length} / 25)
      </h3>
      <div className="space-y-2 max-h-64 overflow-auto">
        {AGENTS.map((agent) => (
          <div
            key={agent.name}
            className="flex items-center justify-between px-2 py-1.5 rounded"
            style={{ background: "rgba(255,255,255,0.03)" }}
          >
            <div className="flex items-center gap-2 text-sm">
              <span>{agent.icon}</span>
              <span>{agent.name}</span>
            </div>
            <div className="flex items-center gap-2 text-xs">
              <span
                className="w-2 h-2 rounded-full"
                style={{ background: statusColor(agent.status) }}
              />
              <span style={{ color: statusColor(agent.status) }}>
                {agent.status}
              </span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
