"use client";

import { useEffect, useRef } from "react";
import * as echarts from "echarts";

/** Mock data for ALife survival tiers */
const tierData = [
  { name: "Normal", value: 38, color: "#22c55e" },
  { name: "Low-Compute", value: 14, color: "#eab308" },
  { name: "Critical", value: 7, color: "#ef4444" },
  { name: "Archived", value: 5, color: "#6b7280" },
];

/** Mock generational fitness data */
const generationData = Array.from({ length: 50 }, (_, i) => ({
  gen: i + 1,
  bestFitness: Math.max(0.3, 0.95 - 0.6 * Math.exp(-i / 12) + Math.random() * 0.02),
  meanFitness: Math.max(0.2, 0.7 - 0.5 * Math.exp(-i / 15) + Math.random() * 0.04),
  paretoSize: Math.min(12, Math.floor(3 + i / 5 + Math.random() * 2)),
}));

function TierDonut() {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!ref.current) return;
    const chart = echarts.init(ref.current, "dark");
    chart.setOption({
      tooltip: { trigger: "item", formatter: "{b}: {c} ({d}%)" },
      series: [{
        type: "pie",
        radius: ["45%", "70%"],
        avoidLabelOverlap: true,
        itemStyle: { borderRadius: 6, borderColor: "#1a1a2e", borderWidth: 2 },
        label: { show: true, color: "#e0e0e0" },
        data: tierData.map((d) => ({
          name: d.name,
          value: d.value,
          itemStyle: { color: d.color },
        })),
      }],
    });
    const resize = () => chart.resize();
    window.addEventListener("resize", resize);
    return () => { window.removeEventListener("resize", resize); chart.dispose(); };
  }, []);
  return <div ref={ref} style={{ width: "100%", height: 280 }} />;
}

function FitnessChart() {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!ref.current) return;
    const chart = echarts.init(ref.current, "dark");
    chart.setOption({
      tooltip: { trigger: "axis" },
      legend: { data: ["Best Fitness", "Mean Fitness", "Pareto Front Size"], textStyle: { color: "#aaa" } },
      grid: { left: 50, right: 30, top: 50, bottom: 30 },
      xAxis: { type: "category", data: generationData.map((d) => d.gen), name: "Generation" },
      yAxis: [
        { type: "value", name: "Fitness", min: 0, max: 1 },
        { type: "value", name: "Pareto Size", min: 0 },
      ],
      series: [
        {
          name: "Best Fitness",
          type: "line",
          data: generationData.map((d) => d.bestFitness.toFixed(3)),
          smooth: true,
          lineStyle: { color: "#22c55e", width: 2 },
          itemStyle: { color: "#22c55e" },
        },
        {
          name: "Mean Fitness",
          type: "line",
          data: generationData.map((d) => d.meanFitness.toFixed(3)),
          smooth: true,
          lineStyle: { color: "#3b82f6", width: 2 },
          itemStyle: { color: "#3b82f6" },
        },
        {
          name: "Pareto Front Size",
          type: "bar",
          yAxisIndex: 1,
          data: generationData.map((d) => d.paretoSize),
          itemStyle: { color: "rgba(168, 85, 247, 0.4)" },
        },
      ],
    });
    const resize = () => chart.resize();
    window.addEventListener("resize", resize);
    return () => { window.removeEventListener("resize", resize); chart.dispose(); };
  }, []);
  return <div ref={ref} style={{ width: "100%", height: 320 }} />;
}

/** Recent evolutionary events */
const recentEvents = [
  { type: "Replication", process: "FNO-Hydro-v3", detail: "Offspring: FNO-Hydro-v3.1 (mutation)", time: "12s ago" },
  { type: "Tier Change", process: "Richards-ML-v2", detail: "Normal → Low-Compute (ρ=0.73)", time: "34s ago" },
  { type: "Speciation", process: "Rothermel-Boreal", detail: "Forked from Rothermel-Surface-v5", time: "2m ago" },
  { type: "Archived", process: "Bucket-Hydro-v1", detail: "Stagnation limit reached (20 gen)", time: "5m ago" },
  { type: "Constitution", process: "DeepONet-Fire-v2", detail: "Conservation violation — demoted", time: "8m ago" },
  { type: "Immigration", process: "GraphCast-TC-v1", detail: "Imported from Earth-2 catalog", time: "15m ago" },
];

const eventColors: Record<string, string> = {
  Replication: "#22c55e",
  "Tier Change": "#eab308",
  Speciation: "#a855f7",
  Archived: "#6b7280",
  Constitution: "#ef4444",
  Immigration: "#3b82f6",
};

export default function EvolutionPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Process Evolution</h1>
        <p className="text-sm mt-1" style={{ color: "var(--text-secondary)" }}>
          ALife-driven evolutionary optimization — survival tiers, fitness landscape, phylogenetic lineage
        </p>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {[
          { label: "Generation", value: "50", sub: "Current" },
          { label: "Alive", value: "59", sub: "of 64 total" },
          { label: "Best Fitness", value: "0.943", sub: "RMSE" },
          { label: "Heartbeat", value: "312", sub: "cycles" },
        ].map((s) => (
          <div key={s.label} className="card p-4">
            <div className="text-xs" style={{ color: "var(--text-secondary)" }}>{s.label}</div>
            <div className="text-2xl font-bold mt-1">{s.value}</div>
            <div className="text-xs" style={{ color: "var(--text-secondary)" }}>{s.sub}</div>
          </div>
        ))}
      </div>

      {/* Charts row */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="card p-4">
          <h3 className="text-sm font-semibold mb-2">Survival Tiers</h3>
          <TierDonut />
        </div>
        <div className="card p-4 lg:col-span-2">
          <h3 className="text-sm font-semibold mb-2">Fitness Over Generations</h3>
          <FitnessChart />
        </div>
      </div>

      {/* Constitution status */}
      <div className="card p-4">
        <h3 className="text-sm font-semibold mb-3">Constitutional Invariants</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {[
            { law: "1. Conserve", status: "Active", violations: 3, color: "#22c55e" },
            { law: "2. Earn Existence", status: "Active", violations: 5, color: "#22c55e" },
            { law: "3. Maintain Provenance", status: "Active", violations: 0, color: "#22c55e" },
          ].map((c) => (
            <div key={c.law} className="p-3 rounded-lg" style={{ background: "rgba(255,255,255,0.03)", border: "1px solid var(--border)" }}>
              <div className="flex items-center justify-between">
                <span className="text-sm font-medium">{c.law}</span>
                <span className="text-xs px-2 py-0.5 rounded" style={{ background: c.color + "22", color: c.color }}>{c.status}</span>
              </div>
              <div className="text-xs mt-1" style={{ color: "var(--text-secondary)" }}>
                {c.violations} total violations detected
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Recent events */}
      <div className="card p-4">
        <h3 className="text-sm font-semibold mb-3">Recent ALife Events</h3>
        <div className="space-y-2">
          {recentEvents.map((e, i) => (
            <div key={i} className="flex items-center gap-3 p-2 rounded" style={{ background: "rgba(255,255,255,0.02)" }}>
              <span
                className="text-xs px-2 py-0.5 rounded font-medium"
                style={{ background: (eventColors[e.type] || "#666") + "22", color: eventColors[e.type] || "#aaa", minWidth: 90, textAlign: "center" }}
              >
                {e.type}
              </span>
              <span className="text-sm font-medium" style={{ minWidth: 160 }}>{e.process}</span>
              <span className="text-xs flex-1" style={{ color: "var(--text-secondary)" }}>{e.detail}</span>
              <span className="text-xs" style={{ color: "var(--text-secondary)" }}>{e.time}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
