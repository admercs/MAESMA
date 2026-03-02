import React from "react";
import {
  AbsoluteFill,
  interpolate,
  useCurrentFrame,
  spring,
  useVideoConfig,
} from "remotion";
import { BG, ACCENT, ACCENT2, ACCENT3, TEXT, MUTED, SURFACE, fadeIn, slideUp } from "./shared";

const AGENTS = [
  { name: "Intent & Scope", color: "#38bdf8" },
  { name: "KB Retrieval", color: "#38bdf8" },
  { name: "Model Assembly", color: "#38bdf8" },
  { name: "Closure Check", color: "#818cf8" },
  { name: "Data & Calib.", color: "#818cf8" },
  { name: "Runtime Sentinel", color: "#818cf8" },
  { name: "Benchmarking", color: "#34d399" },
  { name: "Model Selection", color: "#34d399" },
  { name: "Active Learning", color: "#34d399" },
  { name: "Skill Librarian", color: "#fb923c" },
  { name: "Optimizer", color: "#fb923c" },
  { name: "Data Scout", color: "#fb923c" },
  { name: "A2A Gateway", color: "#f472b6" },
  { name: "MSD Coupling", color: "#f472b6" },
  { name: "EESM Diags", color: "#f472b6" },
  { name: "Discovery", color: "#a78bfa" },
  { name: "Geoengineering", color: "#a78bfa" },
  { name: "Planetary Def.", color: "#a78bfa" },
  { name: "Trophic Dyn.", color: "#fbbf24" },
  { name: "Evolution", color: "#fbbf24" },
  { name: "Provenance", color: "#94a3b8" },
  { name: "Scenario Disc.", color: "#94a3b8" },
];

export const AgentSwarmScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps, width, height } = useVideoConfig();

  const headerOpacity = fadeIn(frame, 5);
  const headerY = slideUp(frame, 5, 20, 30);

  const cx = width / 2;
  const cy = height / 2 + 20;

  /* Slow orbit rotation */
  const orbitAngle = (frame / 300) * Math.PI * 2;

  return (
    <AbsoluteFill
      style={{
        backgroundColor: BG,
        justifyContent: "flex-start",
        alignItems: "center",
        paddingTop: 50,
      }}
    >
      {/* Header */}
      <div
        style={{
          opacity: headerOpacity,
          transform: `translateY(${headerY}px)`,
          fontSize: 42,
          fontWeight: 700,
          fontFamily: "system-ui, sans-serif",
          color: TEXT,
          marginBottom: 8,
        }}
      >
        Agent Swarm
      </div>
      <div
        style={{
          opacity: headerOpacity,
          fontSize: 16,
          fontFamily: "system-ui, sans-serif",
          color: MUTED,
          marginBottom: 10,
        }}
      >
        22 agents own the full model lifecycle
      </div>

      {/* Center KB icon */}
      {(() => {
        const coreOpacity = fadeIn(frame, 15, 20);
        const coreScale = spring({
          frame: frame - 15,
          fps,
          config: { damping: 10 },
        });
        return (
          <div
            style={{
              position: "absolute",
              left: cx - 50,
              top: cy - 50,
              width: 100,
              height: 100,
              borderRadius: "50%",
              background: `radial-gradient(circle, ${ACCENT}33, ${SURFACE})`,
              border: `2px solid ${ACCENT}88`,
              display: "flex",
              justifyContent: "center",
              alignItems: "center",
              flexDirection: "column",
              opacity: coreOpacity,
              transform: `scale(${Math.min(coreScale, 1)})`,
            }}
          >
            <div
              style={{
                fontSize: 12,
                fontWeight: 700,
                color: ACCENT,
                fontFamily: "system-ui, sans-serif",
              }}
            >
              Process
            </div>
            <div
              style={{
                fontSize: 12,
                fontWeight: 700,
                color: ACCENT,
                fontFamily: "system-ui, sans-serif",
              }}
            >
              KB
            </div>
          </div>
        );
      })()}

      {/* Agent nodes orbiting */}
      {AGENTS.map((agent, i) => {
        const angle =
          (i / AGENTS.length) * Math.PI * 2 - Math.PI / 2 + orbitAngle;

        /* Two rings */
        const ring = i < 11 ? 0 : 1;
        const r = ring === 0 ? 170 : 260;
        const ringCount = ring === 0 ? 11 : 11;
        const ringIndex = ring === 0 ? i : i - 11;
        const ringAngle =
          (ringIndex / ringCount) * Math.PI * 2 -
          Math.PI / 2 +
          orbitAngle * (ring === 0 ? 1 : -0.6);

        const ax = cx + Math.cos(ringAngle) * r;
        const ay = cy + Math.sin(ringAngle) * r;

        const delay = 20 + i * 3;
        const agentOpacity = fadeIn(frame, delay, 10);
        const agentScale = spring({
          frame: frame - delay,
          fps,
          config: { damping: 14, stiffness: 120 },
        });

        return (
          <div
            key={agent.name}
            style={{
              position: "absolute",
              left: ax - 42,
              top: ay - 18,
              width: 84,
              textAlign: "center",
              opacity: agentOpacity,
              transform: `scale(${Math.min(agentScale, 1)})`,
            }}
          >
            {/* Dot */}
            <div
              style={{
                width: 12,
                height: 12,
                borderRadius: "50%",
                background: agent.color,
                boxShadow: `0 0 10px ${agent.color}88`,
                margin: "0 auto 4px",
              }}
            />
            <div
              style={{
                fontSize: 10,
                fontWeight: 600,
                color: TEXT,
                fontFamily: "system-ui, sans-serif",
                whiteSpace: "nowrap",
              }}
            >
              {agent.name}
            </div>
          </div>
        );
      })}
    </AbsoluteFill>
  );
};
