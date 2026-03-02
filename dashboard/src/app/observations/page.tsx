"use client";

import { useEffect, useRef } from "react";
import * as echarts from "echarts";

const classificationStats = [
  { label: "Cloud", accepted: 120, rejected: 890 },
  { label: "Clear Land", accepted: 1450, rejected: 45 },
  { label: "Ocean", accepted: 980, rejected: 320 },
  { label: "Snow/Ice", accepted: 310, rejected: 85 },
  { label: "Fire/Smoke", accepted: 67, rejected: 12 },
  { label: "Vegetation", accepted: 1120, rejected: 210 },
];

function FilterChart() {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!ref.current) return;
    const chart = echarts.init(ref.current, "dark");
    chart.setOption({
      tooltip: { trigger: "axis", axisPointer: { type: "shadow" } },
      legend: { data: ["Accepted", "Rejected"], textStyle: { color: "#aaa" } },
      grid: { left: 50, right: 20, top: 40, bottom: 40 },
      xAxis: {
        type: "category",
        data: classificationStats.map((d) => d.label),
        axisLabel: { rotate: 20, fontSize: 10 },
      },
      yAxis: { type: "value", name: "Scenes" },
      series: [
        {
          name: "Accepted",
          type: "bar",
          stack: "total",
          data: classificationStats.map((d) => d.accepted),
          itemStyle: { color: "#22c55e" },
        },
        {
          name: "Rejected",
          type: "bar",
          stack: "total",
          data: classificationStats.map((d) => d.rejected),
          itemStyle: { color: "#ef4444" },
        },
      ],
    });
    const resize = () => chart.resize();
    window.addEventListener("resize", resize);
    return () => { window.removeEventListener("resize", resize); chart.dispose(); };
  }, []);
  return <div ref={ref} style={{ width: "100%", height: 280 }} />;
}

const taskingRequests = [
  { region: "Western Pacific TC", sensor: "SAR", priority: "High", status: "Active", uncertainty: 0.89 },
  { region: "Amazon Deforestation", sensor: "MSI", priority: "High", status: "Queued", uncertainty: 0.76 },
  { region: "Arctic Sea Ice Edge", sensor: "Altimeter", priority: "Medium", status: "Active", uncertainty: 0.68 },
  { region: "Sahel Vegetation", sensor: "Hyperspectral", priority: "Medium", status: "Completed", uncertainty: 0.52 },
  { region: "Australian Bushfire", sensor: "SWIR", priority: "Critical", status: "Active", uncertainty: 0.94 },
];

const statusColors: Record<string, string> = {
  Active: "#22c55e",
  Queued: "#eab308",
  Completed: "#3b82f6",
};

const priorityColors: Record<string, string> = {
  Critical: "#ef4444",
  High: "#f97316",
  Medium: "#eab308",
};

export default function ObservationsPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Observation Intelligence</h1>
        <p className="text-sm mt-1" style={{ color: "var(--text-secondary)" }}>
          PhiSat-2 principles — edge-AI scene classification, on-board filtering, active observation tasking
        </p>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {[
          { label: "Scenes Processed", value: "5,609", sub: "last 24h" },
          { label: "Accepted", value: "4,047", sub: "72.1% pass rate" },
          { label: "Data Reduction", value: "27.9%", sub: "rejected at edge" },
          { label: "Active Taskings", value: "3", sub: "uncertainty-driven" },
        ].map((s) => (
          <div key={s.label} className="card p-4">
            <div className="text-xs" style={{ color: "var(--text-secondary)" }}>{s.label}</div>
            <div className="text-2xl font-bold mt-1">{s.value}</div>
            <div className="text-xs" style={{ color: "var(--text-secondary)" }}>{s.sub}</div>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Filter chart */}
        <div className="card p-4">
          <h3 className="text-sm font-semibold mb-2">Scene Classification &amp; Filtering</h3>
          <FilterChart />
        </div>

        {/* Active tasking requests */}
        <div className="card p-4">
          <h3 className="text-sm font-semibold mb-3">Active Observation Taskings</h3>
          <div className="space-y-2">
            {taskingRequests.map((t, i) => (
              <div key={i} className="flex items-center gap-3 p-2 rounded" style={{ background: "rgba(255,255,255,0.02)" }}>
                <span
                  className="text-xs px-2 py-0.5 rounded font-medium"
                  style={{
                    background: (priorityColors[t.priority] || "#666") + "22",
                    color: priorityColors[t.priority] || "#aaa",
                    minWidth: 60,
                    textAlign: "center",
                  }}
                >
                  {t.priority}
                </span>
                <span className="text-sm font-medium flex-1">{t.region}</span>
                <span className="text-xs" style={{ color: "var(--text-secondary)" }}>{t.sensor}</span>
                <span className="text-xs" style={{ color: "var(--text-secondary)" }}>
                  σ={t.uncertainty.toFixed(2)}
                </span>
                <span
                  className="text-xs px-2 py-0.5 rounded"
                  style={{
                    background: (statusColors[t.status] || "#666") + "22",
                    color: statusColors[t.status] || "#aaa",
                  }}
                >
                  {t.status}
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
