"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface SimStatus {
  running: boolean;
  step: number;
  time: number;
  status: string;
}

export default function SimulationPage() {
  const [sim, setSim] = useState<SimStatus>({
    running: false,
    step: 0,
    time: 0,
    status: "idle",
  });

  useEffect(() => {
    const poll = () => {
      fetch("/api/v1/simulation/status")
        .then((r) => r.json())
        .then(setSim)
        .catch(() => {});
    };
    poll();
    const interval = setInterval(poll, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Simulation</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            Runtime status and control
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Status
            </h3>
            <div className="flex items-center gap-3">
              <span
                className="w-3 h-3 rounded-full"
                style={{ background: sim.running ? "var(--accent-green)" : "var(--text-secondary)" }}
              />
              <span className="text-xl font-bold">{sim.status}</span>
            </div>
          </div>

          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Step
            </h3>
            <div className="text-3xl font-bold" style={{ color: "var(--accent-blue)" }}>
              {sim.step}
            </div>
          </div>

          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Simulation Time
            </h3>
            <div className="text-3xl font-bold" style={{ color: "var(--accent-orange)" }}>
              {(sim.time / 86400).toFixed(1)} days
            </div>
          </div>
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Instructions
          </h3>
          <div className="text-sm space-y-2" style={{ color: "var(--text-secondary)" }}>
            <p>Start a simulation from the CLI:</p>
            <pre className="px-3 py-2 rounded text-xs" style={{ background: "rgba(255,255,255,0.05)" }}>
              maesma run -c sapg.json -s 365
            </pre>
            <p>Start the API server:</p>
            <pre className="px-3 py-2 rounded text-xs" style={{ background: "rgba(255,255,255,0.05)" }}>
              maesma serve --port 3001
            </pre>
          </div>
        </div>
      </main>
    </div>
  );
}
