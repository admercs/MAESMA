"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface Regime {
  id: string;
  name: string;
  region: string;
  active: boolean;
  confidence: number;
  drivers: string[];
  detected_at: string;
}

export default function RegimesPage() {
  const [regimes, setRegimes] = useState<Regime[]>([]);

  useEffect(() => {
    fetch("/api/v1/regimes")
      .then((r) => r.json())
      .then((data) => setRegimes(data.regimes || []))
      .catch(() => {});
  }, []);

  const active = regimes.filter((r) => r.active);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Regime Map</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            Environmental regime detection and tracking
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Total Regimes
            </h3>
            <span className="text-3xl font-bold">{regimes.length}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Active
            </h3>
            <span className="text-3xl font-bold" style={{ color: "var(--accent-orange)" }}>
              {active.length}
            </span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Avg Confidence
            </h3>
            <span className="text-3xl font-bold">
              {regimes.length > 0
                ? (regimes.reduce((s, r) => s + r.confidence, 0) / regimes.length * 100).toFixed(0) + "%"
                : "—"}
            </span>
          </div>
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Detected Regimes
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Name</th>
                  <th className="text-left py-2 px-3">Region</th>
                  <th className="text-center py-2 px-3">Status</th>
                  <th className="text-right py-2 px-3">Confidence</th>
                  <th className="text-left py-2 px-3">Drivers</th>
                  <th className="text-left py-2 px-3">Detected</th>
                </tr>
              </thead>
              <tbody>
                {regimes.map((r) => (
                  <tr key={r.id} style={{ borderBottom: "1px solid var(--border)" }}>
                    <td className="py-2 px-3 font-medium">{r.name}</td>
                    <td className="py-2 px-3">{r.region}</td>
                    <td className="py-2 px-3 text-center">
                      <span
                        className="w-2 h-2 rounded-full inline-block"
                        style={{ background: r.active ? "var(--accent-orange)" : "var(--text-secondary)" }}
                      />
                    </td>
                    <td className="py-2 px-3 text-right font-mono">
                      {(r.confidence * 100).toFixed(0)}%
                    </td>
                    <td className="py-2 px-3 text-xs">{r.drivers.join(", ")}</td>
                    <td className="py-2 px-3 text-xs" style={{ color: "var(--text-secondary)" }}>
                      {r.detected_at}
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
