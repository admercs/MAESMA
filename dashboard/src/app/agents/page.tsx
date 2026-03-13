"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface AgentInfo {
  id: string;
  name: string;
  role: string;
  layer: string;
  status: string;
  last_run: string | null;
}

const LAYER_COLORS: Record<string, string> = {
  strategic: "var(--accent-blue)",
  tactical: "var(--accent-green)",
  operational: "var(--accent-orange)",
};

export default function AgentsPage() {
  const [agents, setAgents] = useState<AgentInfo[]>([]);

  useEffect(() => {
    fetch("/api/v1/agents")
      .then((r) => r.json())
      .then((data) => setAgents(data.agents || []))
      .catch(() => {});
  }, []);

  const byLayer = (layer: string) => agents.filter((a) => a.layer === layer);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Agent Swarm</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            25-agent autonomous swarm across three control layers
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          {["strategic", "tactical", "operational"].map((layer) => (
            <div className="card" key={layer}>
              <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
                {layer}
              </h3>
              <div className="flex items-center gap-3">
                <span
                  className="w-3 h-3 rounded-full"
                  style={{ background: LAYER_COLORS[layer] || "var(--text-secondary)" }}
                />
                <span className="text-xl font-bold">{byLayer(layer).length} agents</span>
              </div>
            </div>
          ))}
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            All Agents
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Name</th>
                  <th className="text-left py-2 px-3">Role</th>
                  <th className="text-left py-2 px-3">Layer</th>
                  <th className="text-left py-2 px-3">Status</th>
                  <th className="text-left py-2 px-3">Last Run</th>
                </tr>
              </thead>
              <tbody>
                {agents.map((a) => (
                  <tr key={a.id} style={{ borderBottom: "1px solid var(--border)" }}>
                    <td className="py-2 px-3 font-medium">{a.name}</td>
                    <td className="py-2 px-3 font-mono text-xs">{a.role}</td>
                    <td className="py-2 px-3">
                      <span
                        className="px-2 py-0.5 rounded text-xs font-medium"
                        style={{
                          background: LAYER_COLORS[a.layer] || "var(--surface)",
                          color: "var(--bg)",
                        }}
                      >
                        {a.layer}
                      </span>
                    </td>
                    <td className="py-2 px-3">{a.status}</td>
                    <td className="py-2 px-3 text-xs" style={{ color: "var(--text-secondary)" }}>
                      {a.last_run || "—"}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </main>
    </div>
  );
}
