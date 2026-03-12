"use client";

import { useEffect, useState } from "react";

interface AgentInfo {
  role: string;
  description: string;
}

const ROLE_ICONS: Record<string, string> = {
  KbRetrieval: "📚", Assembly: "🔧", ClosureValidator: "✅",
  Benchmarking: "📏", Selection: "🎯", Optimizer: "⚡",
  Discovery: "🔬", DataScout: "📡", A2aGateway: "🌐",
  RegimeDetector: "🌍", ScaleNegotiator: "📐", Provenance: "📋",
  SalientDynamics: "🌊", Ensemble: "🎲", Diagnostics: "🩺",
  Sensitivity: "📊", Hypothesis: "💡", Geoengineering: "🏭",
  PlanetaryDefense: "🛡️", Trophic: "🦁", Evolution: "🧬",
  MetaLearner: "🧠", RuntimeSentinel: "🔒", FoundationModel: "🏗️",
  AutonomousObservation: "🛰️",
};

export function AgentStatusCard() {
  const [agents, setAgents] = useState<AgentInfo[]>([]);

  useEffect(() => {
    fetch("/api/v1/agents")
      .then((r) => r.json())
      .then((data) => setAgents(data.agents || []))
      .catch(() => {});
  }, []);

  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Agent Swarm ({agents.length} / 25)
      </h3>
      <div className="space-y-2 max-h-64 overflow-auto">
        {agents.map((agent) => (
          <div
            key={agent.role}
            className="flex items-center justify-between px-2 py-1.5 rounded"
            style={{ background: "rgba(255,255,255,0.03)" }}
          >
            <div className="flex items-center gap-2 text-sm">
              <span>{ROLE_ICONS[agent.role] || "🤖"}</span>
              <span>{agent.role}</span>
            </div>
            <div className="text-xs truncate max-w-[140px]" style={{ color: "var(--text-secondary)" }}>
              {agent.description}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
