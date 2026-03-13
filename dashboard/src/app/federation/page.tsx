"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface Peer {
  id: string;
  name: string;
  endpoint: string;
  status: string;
  trust_score: number;
  manifests_shared: number;
  skills_shared: number;
  last_sync: string | null;
}

export default function FederationPage() {
  const [peers, setPeers] = useState<Peer[]>([]);

  useEffect(() => {
    fetch("/api/v1/federation/peers")
      .then((r) => r.json())
      .then((data) => setPeers(data.peers || []))
      .catch(() => {});
  }, []);

  const connected = peers.filter((p) => p.status === "connected");

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Federation</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            A2A peer network and knowledge exchange
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-6">
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Total Peers
            </h3>
            <span className="text-3xl font-bold">{peers.length}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Connected
            </h3>
            <span className="text-3xl font-bold" style={{ color: "var(--accent-green)" }}>
              {connected.length}
            </span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Manifests Shared
            </h3>
            <span className="text-3xl font-bold">
              {peers.reduce((s, p) => s + p.manifests_shared, 0)}
            </span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Skills Shared
            </h3>
            <span className="text-3xl font-bold">
              {peers.reduce((s, p) => s + p.skills_shared, 0)}
            </span>
          </div>
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Peer Network
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Peer</th>
                  <th className="text-left py-2 px-3">Endpoint</th>
                  <th className="text-center py-2 px-3">Status</th>
                  <th className="text-right py-2 px-3">Trust</th>
                  <th className="text-right py-2 px-3">Manifests</th>
                  <th className="text-right py-2 px-3">Skills</th>
                  <th className="text-left py-2 px-3">Last Sync</th>
                </tr>
              </thead>
              <tbody>
                {peers.map((p) => (
                  <tr key={p.id} style={{ borderBottom: "1px solid var(--border)" }}>
                    <td className="py-2 px-3 font-medium">{p.name}</td>
                    <td className="py-2 px-3 font-mono text-xs">{p.endpoint}</td>
                    <td className="py-2 px-3 text-center">
                      <span
                        className="w-2 h-2 rounded-full inline-block"
                        style={{
                          background: p.status === "connected" ? "var(--accent-green)" : "var(--text-secondary)",
                        }}
                      />
                    </td>
                    <td className="py-2 px-3 text-right font-mono">
                      {(p.trust_score * 100).toFixed(0)}%
                    </td>
                    <td className="py-2 px-3 text-right">{p.manifests_shared}</td>
                    <td className="py-2 px-3 text-right">{p.skills_shared}</td>
                    <td className="py-2 px-3 text-xs" style={{ color: "var(--text-secondary)" }}>
                      {p.last_sync || "never"}
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
