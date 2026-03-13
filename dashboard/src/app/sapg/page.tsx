"use client";

import { useEffect, useState } from "react";
import { Sidebar } from "@/components/Sidebar";

interface SapgNode {
  id: string;
  name: string;
  family: string;
  rung: string;
}

interface SapgEdge {
  source: string;
  target: string;
  variables: string[];
}

export default function SapgPage() {
  const [nodes, setNodes] = useState<SapgNode[]>([]);
  const [edges, setEdges] = useState<SapgEdge[]>([]);

  useEffect(() => {
    fetch("/api/v1/sapg")
      .then((r) => r.json())
      .then((data) => {
        setNodes(data.nodes || []);
        setEdges(data.edges || []);
      })
      .catch(() => {});
  }, []);

  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">Surrogate-Augmented Process Graph</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            Live view of the active SAPG topology
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Nodes
            </h3>
            <span className="text-3xl font-bold">{nodes.length}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Edges
            </h3>
            <span className="text-3xl font-bold">{edges.length}</span>
          </div>
          <div className="card">
            <h3 className="text-sm font-semibold mb-2 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
              Families
            </h3>
            <span className="text-3xl font-bold">
              {new Set(nodes.map((n) => n.family)).size}
            </span>
          </div>
        </div>

        <div className="card">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Process Nodes
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Name</th>
                  <th className="text-left py-2 px-3">Family</th>
                  <th className="text-left py-2 px-3">Rung</th>
                </tr>
              </thead>
              <tbody>
                {nodes.map((n) => (
                  <tr key={n.id} style={{ borderBottom: "1px solid var(--border)" }}>
                    <td className="py-2 px-3 font-mono text-xs">{n.name}</td>
                    <td className="py-2 px-3">{n.family}</td>
                    <td className="py-2 px-3">{n.rung}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        <div className="card mt-6">
          <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
            Coupling Edges
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr style={{ borderBottom: "1px solid var(--border)" }}>
                  <th className="text-left py-2 px-3">Source</th>
                  <th className="text-left py-2 px-3">Target</th>
                  <th className="text-left py-2 px-3">Variables</th>
                </tr>
              </thead>
              <tbody>
                {edges.map((e, i) => (
                  <tr key={i} style={{ borderBottom: "1px solid var(--border)" }}>
                    <td className="py-2 px-3 font-mono text-xs">{e.source}</td>
                    <td className="py-2 px-3 font-mono text-xs">{e.target}</td>
                    <td className="py-2 px-3">{e.variables.join(", ")}</td>
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
