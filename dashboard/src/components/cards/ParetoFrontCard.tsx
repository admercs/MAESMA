"use client";

import dynamic from "next/dynamic";
import type { EChartsOption } from "echarts";

const ReactECharts = dynamic(() => import("echarts-for-react"), { ssr: false });

// Demo: Pareto front (skill vs cost)
const candidates = [
  { name: "Richards R1", kge: 0.82, cost: 1.2 },
  { name: "Bucket R0", kge: 0.55, cost: 0.1 },
  { name: "VarSat R2", kge: 0.91, cost: 8.5 },
  { name: "ParFlow R3", kge: 0.95, cost: 45.0 },
  { name: "Rothermel R1", kge: 0.75, cost: 2.1 },
  { name: "CFS FBP R1", kge: 0.78, cost: 1.8 },
  { name: "Balbi R2", kge: 0.88, cost: 12.0 },
  { name: "Stoch Fire R0", kge: 0.45, cost: 0.05 },
  { name: "CENTURY R1", kge: 0.80, cost: 0.8 },
  { name: "MIMICS R2", kge: 0.87, cost: 3.2 },
  { name: "Two-Stream R1", kge: 0.90, cost: 0.3 },
  { name: "Cohort R1", kge: 0.72, cost: 4.5 },
];

const option: EChartsOption = {
  backgroundColor: "transparent",
  tooltip: {
    trigger: "item",
    formatter: (params: any) =>
      `${params.data[2]}<br/>KGE: ${params.data[1]}<br/>Cost: ${params.data[0]} GFLOP`,
  },
  grid: { left: 60, right: 30, top: 20, bottom: 50 },
  xAxis: {
    type: "log",
    name: "Cost (GFLOP/step)",
    nameTextStyle: { color: "#94a3b8" },
    axisLabel: { color: "#94a3b8" },
    splitLine: { lineStyle: { color: "#2d3748" } },
  },
  yAxis: {
    type: "value",
    name: "KGE",
    min: 0.3,
    max: 1.0,
    nameTextStyle: { color: "#94a3b8" },
    axisLabel: { color: "#94a3b8" },
    splitLine: { lineStyle: { color: "#2d3748" } },
  },
  series: [
    {
      type: "scatter",
      symbolSize: 14,
      data: candidates.map((c) => [c.cost, c.kge, c.name]),
      label: {
        show: true,
        formatter: (params: any) => params.data[2],
        fontSize: 9,
        color: "#94a3b8",
        position: "right",
      },
      itemStyle: {
        color: (params: any) => {
          const kge = params.data[1];
          if (kge > 0.85) return "#10b981";
          if (kge > 0.7) return "#3b82f6";
          return "#f59e0b";
        },
      },
    },
  ],
};

export function ParetoFrontCard() {
  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Pareto Front (Skill vs Cost)
      </h3>
      <div style={{ height: 300 }}>
        <ReactECharts option={option} style={{ height: "100%" }} />
      </div>
    </div>
  );
}
