"use client";

import dynamic from "next/dynamic";
import type { EChartsOption } from "echarts";

const ReactECharts = dynamic(() => import("echarts-for-react"), { ssr: false });

// Demo: skill evolution over benchmark iterations
const iterations = Array.from({ length: 20 }, (_, i) => i + 1);

const option: EChartsOption = {
  backgroundColor: "transparent",
  tooltip: { trigger: "axis" },
  legend: {
    data: ["KGE (Hydrology R1)", "KGE (Ecology R1)", "KGE (Fire R1)"],
    textStyle: { color: "#94a3b8", fontSize: 10 },
    bottom: 0,
  },
  grid: { left: 50, right: 20, top: 20, bottom: 60 },
  xAxis: {
    type: "category",
    data: iterations.map(String),
    name: "Benchmark Iteration",
    nameTextStyle: { color: "#94a3b8" },
    axisLabel: { color: "#94a3b8" },
  },
  yAxis: {
    type: "value",
    name: "KGE",
    min: 0,
    max: 1,
    nameTextStyle: { color: "#94a3b8" },
    axisLabel: { color: "#94a3b8" },
    splitLine: { lineStyle: { color: "#2d3748" } },
  },
  series: [
    {
      name: "KGE (Hydrology R1)",
      type: "line",
      smooth: true,
      data: iterations.map((i) => +(0.3 + 0.6 * (1 - Math.exp(-i / 5))).toFixed(3)),
      lineStyle: { color: "#3b82f6" },
      itemStyle: { color: "#3b82f6" },
    },
    {
      name: "KGE (Ecology R1)",
      type: "line",
      smooth: true,
      data: iterations.map((i) => +(0.25 + 0.5 * (1 - Math.exp(-i / 7))).toFixed(3)),
      lineStyle: { color: "#10b981" },
      itemStyle: { color: "#10b981" },
    },
    {
      name: "KGE (Fire R1)",
      type: "line",
      smooth: true,
      data: iterations.map((i) => +(0.2 + 0.55 * (1 - Math.exp(-i / 8))).toFixed(3)),
      lineStyle: { color: "#f59e0b" },
      itemStyle: { color: "#f59e0b" },
    },
  ],
};

export function SkillEvolutionCard() {
  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Skill Evolution
      </h3>
      <div style={{ height: 300 }}>
        <ReactECharts option={option} style={{ height: "100%" }} />
      </div>
    </div>
  );
}
