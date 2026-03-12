"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface Manifest {
  id: string;
  name: string;
  family: string;
}

export default function KnowledgebasePage() {
  const [manifests, setManifests] = useState<Manifest[]>([]);
  const [stats, setStats] = useState({ manifests: 0, skill_records: 0, families: 0 });
  const [filter, setFilter] = useState("");

  useEffect(() => {
    fetch("/api/v1/kb/manifests")
      .then((r) => r.json())
      .then((data) => setManifests(data.manifests || []))
      .catch(() => {});
    fetch("/api/v1/kb/stats")
      .then((r) => r.json())
      .then(setStats)
      .catch(() => {});
  }, []);

  const filtered = filter
    ? manifests.filter((m) => m.family.toLowerCase().includes(filter.toLowerCase()))
    : manifests;

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Process Knowledgebase</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            {stats.manifests} manifests &middot; {stats.skill_records} skill records &middot; {stats.families} families
          </p>
        </header>

        <div className="mb-4">
          <input
            type="text"
            placeholder="Filter by family..."
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            className="px-3 py-2 rounded text-sm w-64"
            style={{
              background: "var(--card-bg)",
              border: "1px solid var(--border)",
              color: "var(--text-primary)",
            }}
          />
        </div>

        <div className="card">
          <table className="w-full text-sm">
            <thead>
              <tr style={{ color: "var(--text-secondary)" }}>
                <th className="text-left pb-3 font-semibold">ID</th>
                <th className="text-left pb-3 font-semibold">Name</th>
                <th className="text-left pb-3 font-semibold">Family</th>
              </tr>
            </thead>
            <tbody>
              {filtered.map((m) => (
                <tr key={m.id} className="border-t" style={{ borderColor: "var(--border)" }}>
                  <td className="py-2 font-mono text-xs" style={{ color: "var(--text-secondary)" }}>
                    {m.id.substring(0, 8)}…
                  </td>
                  <td className="py-2">{m.name}</td>
                  <td className="py-2">
                    <span
                      className="px-2 py-0.5 rounded text-xs"
                      style={{ background: "var(--accent-blue)", color: "#fff", opacity: 0.9 }}
                    >
                      {m.family}
                    </span>
                  </td>
                </tr>
              ))}
              {filtered.length === 0 && (
                <tr>
                  <td colSpan={3} className="py-8 text-center" style={{ color: "var(--text-secondary)" }}>
                    {manifests.length === 0 ? "Loading..." : "No manifests match filter"}
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </main>
    </div>
  );
}
