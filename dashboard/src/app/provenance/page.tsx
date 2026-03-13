"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface ProvenanceRecord {
  id: string;
  timestamp: string;
  action: string;
  agent: string;
  target: string;
  rationale: string;
  parent_id: string | null;
}

export default function ProvenancePage() {
  const [records, setRecords] = useState<ProvenanceRecord[]>([]);

  useEffect(() => {
    fetch("/api/v1/provenance")
      .then((r) => r.json())
      .then((data) => setRecords(data.records || []))
      .catch(() => {});
  }, []);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Provenance</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            Full lineage tracking for every agent decision and model change
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Total Records
            </h3>
            <span className="text-3xl font-bold">{records.length}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Unique Agents
            </h3>
            <span className="text-3xl font-bold">
              {new Set(records.map((r) => r.agent)).size}
            </span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Action Types
            </h3>
            <span className="text-3xl font-bold">
              {new Set(records.map((r) => r.action)).size}
            </span>
          </div>
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Provenance Log
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Timestamp</th>
                  <th className="text-left py-2 px-3">Action</th>
                  <th className="text-left py-2 px-3">Agent</th>
                  <th className="text-left py-2 px-3">Target</th>
                  <th className="text-left py-2 px-3">Rationale</th>
                </tr>
              </thead>
              <tbody>
                {records.map((r) => (
                  <tr key={r.id} style={{ borderBottom: "1px solid var(--border)" }}>
                    <td className="py-2 px-3 font-mono text-xs" style={{ color: "var(--text-secondary)" }}>
                      {r.timestamp}
                    </td>
                    <td className="py-2 px-3">
                      <span className="px-2 py-0.5 rounded text-xs font-medium" style={{ background: "var(--surface)" }}>
                        {r.action}
                      </span>
                    </td>
                    <td className="py-2 px-3 font-medium">{r.agent}</td>
                    <td className="py-2 px-3 font-mono text-xs">{r.target}</td>
                    <td className="py-2 px-3 text-xs" style={{ color: "var(--text-secondary)" }}>
                      {r.rationale}
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
