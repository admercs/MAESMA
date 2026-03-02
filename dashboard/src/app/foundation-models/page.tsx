"use client";

import { useEffect, useRef } from "react";
import * as echarts from "echarts";

const models = [
  { name: "FourCastNet", arch: "SFNO", params: "450M", speed: "1200×", status: "Active", rmse: 0.42 },
  { name: "GraphCast", arch: "GNN", params: "37M", speed: "1000×", status: "Active", rmse: 0.38 },
  { name: "GenCast", arch: "Diffusion", params: "310M", speed: "800×", status: "Active", rmse: 0.35 },
  { name: "Pangu-Weather", arch: "ViT-3D", params: "256M", speed: "1100×", status: "Standby", rmse: 0.44 },
  { name: "CorrDiff", arch: "Diffusion", params: "128M", speed: "600×", status: "Active", rmse: 0.41 },
  { name: "GraphCast-TC", arch: "GNN (fine-tuned)", params: "42M", speed: "950×", status: "Evolved", rmse: 0.31 },
];

function SkillChart() {
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!ref.current) return;
    const chart = echarts.init(ref.current, "dark");
    chart.setOption({
      tooltip: { trigger: "axis" },
      grid: { left: 50, right: 20, top: 30, bottom: 40 },
      xAxis: {
        type: "category",
        data: models.map((m) => m.name),
        axisLabel: { rotate: 25, fontSize: 10 },
      },
      yAxis: { type: "value", name: "RMSE (K)", min: 0 },
      series: [{
        type: "bar",
        data: models.map((m) => ({
          value: m.rmse,
          itemStyle: {
            color: m.status === "Evolved" ? "#a855f7"
              : m.status === "Active" ? "#3b82f6"
                : "#6b7280",
          },
        })),
        barWidth: "50%",
      }],
    });
    const resize = () => chart.resize();
    window.addEventListener("resize", resize);
    return () => { window.removeEventListener("resize", resize); chart.dispose(); };
  }, []);
  return <div ref={ref} style={{ width: "100%", height: 260 }} />;
}

const statusColors: Record<string, string> = {
  Active: "#22c55e",
  Standby: "#eab308",
  Evolved: "#a855f7",
};

export default function FoundationModelsPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold">Foundation Models</h1>
        <p className="text-sm mt-1" style={{ color: "var(--text-secondary)" }}>
          NVIDIA Earth-2 / earth2studio — GPU-accelerated atmospheric state evolution
        </p>
      </div>

      {/* Stats row */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
        {[
          { label: "Active Models", value: "5", sub: "of 6 total" },
          { label: "Evolved Variants", value: "1", sub: "via ALife" },
          { label: "Best RMSE", value: "0.31 K", sub: "GraphCast-TC" },
          { label: "Speedup", value: "1000×", sub: "vs. dynamical" },
        ].map((s) => (
          <div key={s.label} className="card p-4">
            <div className="text-xs" style={{ color: "var(--text-secondary)" }}>{s.label}</div>
            <div className="text-2xl font-bold mt-1">{s.value}</div>
            <div className="text-xs" style={{ color: "var(--text-secondary)" }}>{s.sub}</div>
          </div>
        ))}
      </div>

      {/* Chart + table */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card p-4">
          <h3 className="text-sm font-semibold mb-2">Model Skill Comparison</h3>
          <SkillChart />
        </div>
        <div className="card p-4">
          <h3 className="text-sm font-semibold mb-3">Model Registry</h3>
          <div className="overflow-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="text-xs" style={{ color: "var(--text-secondary)" }}>
                  <th className="text-left py-2">Model</th>
                  <th className="text-left py-2">Architecture</th>
                  <th className="text-right py-2">Params</th>
                  <th className="text-right py-2">Speedup</th>
                  <th className="text-center py-2">Status</th>
                </tr>
              </thead>
              <tbody>
                {models.map((m) => (
                  <tr key={m.name} className="border-t" style={{ borderColor: "var(--border)" }}>
                    <td className="py-2 font-medium">{m.name}</td>
                    <td className="py-2" style={{ color: "var(--text-secondary)" }}>{m.arch}</td>
                    <td className="py-2 text-right">{m.params}</td>
                    <td className="py-2 text-right">{m.speed}</td>
                    <td className="py-2 text-center">
                      <span
                        className="text-xs px-2 py-0.5 rounded"
                        style={{
                          background: (statusColors[m.status] || "#666") + "22",
                          color: statusColors[m.status] || "#aaa",
                        }}
                      >
                        {m.status}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
