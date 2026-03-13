"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface SkillRecord {
  process_id: string;
  process_name: string;
  family: string;
  rung: string;
  rmse: number;
  kge: number;
  nse: number;
  bias: number;
  dataset: string;
  evaluated_at: string;
}

export default function SkillsPage() {
  const [skills, setSkills] = useState<SkillRecord[]>([]);

  useEffect(() => {
    fetch("/api/v1/skills")
      .then((r) => r.json())
      .then((data) => setSkills(data.records || []))
      .catch(() => {});
  }, []);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Skill Scores</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            Empirical performance records per process, region, and regime
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Total Records
            </h3>
            <span className="text-3xl font-bold">{skills.length}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Processes Evaluated
            </h3>
            <span className="text-3xl font-bold">
              {new Set(skills.map((s) => s.process_id)).size}
            </span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Avg KGE
            </h3>
            <span className="text-3xl font-bold">
              {skills.length > 0
                ? (skills.reduce((sum, s) => sum + s.kge, 0) / skills.length).toFixed(3)
                : "—"}
            </span>
          </div>
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Skill Records
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Process</th>
                  <th className="text-left py-2 px-3">Family</th>
                  <th className="text-left py-2 px-3">Rung</th>
                  <th className="text-right py-2 px-3">RMSE</th>
                  <th className="text-right py-2 px-3">KGE</th>
                  <th className="text-right py-2 px-3">NSE</th>
                  <th className="text-right py-2 px-3">Bias</th>
                  <th className="text-left py-2 px-3">Dataset</th>
                </tr>
              </thead>
              <tbody>
                {skills.map((s, i) => (
                  <tr key={i} style={{ borderBottom: "1px solid var(--border)" }}>
                    <td className="py-2 px-3 font-mono text-xs">{s.process_name}</td>
                    <td className="py-2 px-3">{s.family}</td>
                    <td className="py-2 px-3">{s.rung}</td>
                    <td className="py-2 px-3 text-right font-mono">{s.rmse.toFixed(4)}</td>
                    <td className="py-2 px-3 text-right font-mono">{s.kge.toFixed(4)}</td>
                    <td className="py-2 px-3 text-right font-mono">{s.nse.toFixed(4)}</td>
                    <td className="py-2 px-3 text-right font-mono">{s.bias.toFixed(4)}</td>
                    <td className="py-2 px-3 text-xs">{s.dataset}</td>
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
