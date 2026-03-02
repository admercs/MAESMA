"use client";

import dynamic from "next/dynamic";
import type { EChartsOption } from "echarts";

const ReactECharts = dynamic(() => import("echarts-for-react"), { ssr: false });

const FAMILIES = [
  "Fire", "Hydrology", "Ecology", "Biogeochem", "Radiation",
  "Atmosphere", "Ocean", "Cryosphere", "Human", "Trophic", "Evolution",
];

// Demo data: SAPG as a force-directed graph
const nodes = FAMILIES.map((name, i) => ({
  id: String(i),
  name,
  symbolSize: 30 + Math.random() * 20,
  category: i,
}));

const edges = [
  { source: "0", target: "1" }, // Fire → Hydrology
  { source: "0", target: "2" }, // Fire → Ecology
  { source: "1", target: "2" }, // Hydrology → Ecology
  { source: "2", target: "3" }, // Ecology → Biogeochem
  { source: "3", target: "1" }, // Biogeochem → Hydrology
  { source: "4", target: "2" }, // Radiation → Ecology
  { source: "4", target: "5" }, // Radiation → Atmosphere
  { source: "5", target: "1" }, // Atmosphere → Hydrology
  { source: "5", target: "6" }, // Atmosphere → Ocean
  { source: "6", target: "7" }, // Ocean → Cryosphere
  { source: "7", target: "1" }, // Cryosphere → Hydrology
  { source: "8", target: "0" }, // Human → Fire
  { source: "8", target: "2" }, // Human → Ecology
  { source: "2", target: "9" }, // Ecology → Trophic
  { source: "9", target: "10" }, // Trophic → Evolution
];

const option: EChartsOption = {
  backgroundColor: "transparent",
  tooltip: { trigger: "item" },
  series: [
    {
      type: "graph",
      layout: "force",
      data: nodes,
      links: edges,
      roam: true,
      label: { show: true, fontSize: 10, color: "#e2e8f0" },
      lineStyle: { color: "#4a5568", curveness: 0.2 },
      itemStyle: { borderColor: "#2d3748", borderWidth: 1 },
      force: { repulsion: 200, edgeLength: 120 },
      categories: FAMILIES.map((name) => ({ name })),
    },
  ],
};

export function SapgGraphCard() {
  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Scale-Aware Process Graph (SAPG)
      </h3>
      <div style={{ height: 350 }}>
        <ReactECharts option={option} style={{ height: "100%" }} />
      </div>
    </div>
  );
}
