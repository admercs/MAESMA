"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface ParetoPoint {
  process_id: string;
  name: string;
  family: string;
  rung: string;
  rmse: number;
  cost: number;
  kge: number;
  dominated: boolean;
}

export default function OptimizationPage() {
  const [points, setPoints] = useState<ParetoPoint[]>([]);
  const [generation, setGeneration] = useState(0);

  useEffect(() => {
    fetch("/api/v1/optimization/pareto")
      .then((r) => r.json())
      .then((data) => {
        setPoints(data.points || []);
        setGeneration(data.generation || 0);
      })
      .catch(() => {});
  }, []);

  const frontier = points.filter((p) => !p.dominated);
  const dominated = points.filter((p) => p.dominated);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Optimization</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            Pareto frontier over skill vs. cost
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-6">
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Generation
            </h3>
            <span className="text-3xl font-bold">{generation}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Total Candidates
            </h3>
            <span className="text-3xl font-bold">{points.length}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Pareto Front
            </h3>
            <span className="text-3xl font-bold" style={{ color: "var(--accent-green)" }}>
              {frontier.length}
            </span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Dominated
            </h3>
            <span className="text-3xl font-bold" style={{ color: "var(--text-secondary)" }}>
              {dominated.length}
            </span>
          </div>
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Pareto Frontier
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Process</th>
                  <th className="text-left py-2 px-3">Family</th>
                  <th className="text-left py-2 px-3">Rung</th>
                  <th className="text-right py-2 px-3">RMSE ↓</th>
                  <th className="text-right py-2 px-3">Cost ↓</th>
                  <th className="text-right py-2 px-3">KGE ↑</th>
                  <th className="text-center py-2 px-3">Status</th>
                </tr>
              </thead>
              <tbody>
                {points.map((p, i) => (
                  <tr
                    key={i}
                    style={{
                      borderBottom: "1px solid var(--border)",
                      opacity: p.dominated ? 0.5 : 1,
                    }}
                  >
                    <td className="py-2 px-3 font-mono text-xs">{p.name}</td>
                    <td className="py-2 px-3">{p.family}</td>
                    <td className="py-2 px-3">{p.rung}</td>
                    <td className="py-2 px-3 text-right font-mono">{p.rmse.toFixed(4)}</td>
                    <td className="py-2 px-3 text-right font-mono">{p.cost.toFixed(2)}</td>
                    <td className="py-2 px-3 text-right font-mono">{p.kge.toFixed(4)}</td>
                    <td className="py-2 px-3 text-center">
                      {p.dominated ? (
                        <span style={{ color: "var(--text-secondary)" }}>dominated</span>
                      ) : (
                        <span style={{ color: "var(--accent-green)" }}>frontier</span>
                      )}
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
