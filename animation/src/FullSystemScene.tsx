import React from "react";
import {
  AbsoluteFill,
  interpolate,
  useCurrentFrame,
  spring,
  useVideoConfig,
} from "remotion";
import { BG, ACCENT, ACCENT2, ACCENT3, TEXT, MUTED, SURFACE, fadeIn, slideUp } from "./shared";

/**
 * Full system overview — KB at center, inference engine, agents, simulation,
 * errors feeding back. A simplified visual of the Knowledgebase-Centric Architecture.
 */

const BOX_STYLE: React.CSSProperties = {
  padding: "14px 22px",
  borderRadius: 12,
  textAlign: "center",
  fontFamily: "system-ui, sans-serif",
  fontWeight: 600,
};

export const FullSystemScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps, width, height } = useVideoConfig();

  const headerOpacity = fadeIn(frame, 5);
  const headerY = slideUp(frame, 5, 20, 30);

  /* Pulse for data flow arrows */
  const pulse = 0.5 + 0.5 * Math.sin((frame / 20) * Math.PI * 2);

  const block = (
    label: string,
    color: string,
    delay: number,
    left: number,
    top: number,
    w: number,
    sub?: string
  ) => {
    const opacity = fadeIn(frame, delay, 15);
    const scale = spring({
      frame: frame - delay,
      fps,
      config: { damping: 14 },
    });
    return (
      <div
        style={{
          ...BOX_STYLE,
          position: "absolute",
          left,
          top,
          width: w,
          opacity,
          transform: `scale(${Math.min(scale, 1)})`,
          background: `${color}11`,
          border: `1.5px solid ${color}88`,
          color: TEXT,
          fontSize: 15,
        }}
      >
        {label}
        {sub && (
          <div style={{ fontSize: 11, fontWeight: 400, color: MUTED, marginTop: 4 }}>
            {sub}
          </div>
        )}
      </div>
    );
  };

  const arrow = (
    x1: number,
    y1: number,
    x2: number,
    y2: number,
    color: string,
    delay: number,
    label?: string
  ) => {
    const opacity = fadeIn(frame, delay, 12) * (0.6 + 0.4 * pulse);
    return (
      <g opacity={opacity}>
        <defs>
          <marker
            id={`arrowhead-${x1}-${y1}`}
            markerWidth="8"
            markerHeight="6"
            refX="8"
            refY="3"
            orient="auto"
          >
            <polygon points="0 0, 8 3, 0 6" fill={color} />
          </marker>
        </defs>
        <line
          x1={x1}
          y1={y1}
          x2={x2}
          y2={y2}
          stroke={color}
          strokeWidth={2}
          markerEnd={`url(#arrowhead-${x1}-${y1})`}
        />
        {label && (
          <text
            x={(x1 + x2) / 2 + 8}
            y={(y1 + y2) / 2 - 4}
            fill={MUTED}
            fontSize={10}
            fontFamily="system-ui, sans-serif"
          >
            {label}
          </text>
        )}
      </g>
    );
  };

  return (
    <AbsoluteFill
      style={{
        backgroundColor: BG,
        justifyContent: "flex-start",
        alignItems: "center",
        paddingTop: 40,
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
          marginBottom: 4,
        }}
      >
        MAESMA Architecture
      </div>
      <div
        style={{
          opacity: headerOpacity,
          fontSize: 16,
          fontFamily: "system-ui, sans-serif",
          color: MUTED,
          marginBottom: 20,
        }}
      >
        Knowledgebase-centric autonomous Earth system modeling
      </div>

      {/* Blocks positioned absolutely */}
      {/* Top: Observations + KB + Discovered */}
      {block("Observations", "#38bdf8", 15, 60, 140, 180, "STAC / CMR / NRT")}
      {block("Process Knowledgebase", ACCENT, 10, width / 2 - 130, 130, 260, "Code · Manifest · Ontology · Skill · Provenance")}
      {block("Discovered\nProcesses", ACCENT3, 18, width - 250, 140, 170, "ML / Symbolic / Hybrid")}

      {/* Middle: Neural Inference Engine */}
      {block("Neural Inference Engine", ACCENT2, 25, width / 2 - 130, 270, 260, "Graph transformer · error-driven proposals")}

      {/* Below: Agent Swarm + Simulation */}
      {block("Agent Swarm (22)", "#fb923c", 35, 150, 400, 220, "Assemble · Compile · Execute · Benchmark · Score")}
      {block("Simulation Runtime", ACCENT3, 38, width - 400, 400, 210, "Multi-GPU · Task scheduler")}

      {/* Bottom: Errors + Discovery */}
      {block("Errors & Uncertainties", "#ef4444", 50, width / 2 - 120, 530, 240, "Drive next inference cycle")}
      {block("Process Discovery", "#a78bfa", 55, 80, 530, 200, "Residual → Learn → Validate → Deposit")}

      {/* Arrows (SVG overlay) */}
      <svg
        width={width}
        height={height}
        style={{ position: "absolute", top: 0, left: 0, pointerEvents: "none" }}
      >
        {/* Observations → KB */}
        {arrow(240, 170, width / 2 - 130, 170, "#38bdf8", 22, "ingest")}
        {/* Discovered → KB */}
        {arrow(width - 250, 170, width / 2 + 130, 170, ACCENT3, 24, "deposit")}
        {/* KB → Neural Inference */}
        {arrow(width / 2, 200, width / 2, 270, ACCENT, 28, "query")}
        {/* Neural Inference → Agent Swarm */}
        {arrow(width / 2 - 60, 330, 300, 400, ACCENT2, 40, "proposals")}
        {/* Agent Swarm → Simulation */}
        {arrow(370, 430, width - 400, 430, "#fb923c", 42, "execute")}
        {/* Simulation → Errors */}
        {arrow(width - 290, 470, width / 2 + 100, 530, ACCENT3, 48, "evaluate")}
        {/* Errors → Neural Inference (feedback) */}
        {arrow(width / 2 + 80, 530, width / 2 + 100, 340, "#ef4444", 55, "feedback")}
        {/* Errors → Discovery */}
        {arrow(width / 2 - 120, 560, 280, 560, "#ef4444", 58, "residuals")}
        {/* Discovery → KB (deposit) */}
        {arrow(180, 530, width / 2 - 80, 200, "#a78bfa", 62, "deposit")}
      </svg>

      {/* Bottom tagline */}
      {(() => {
        const tagOpacity = fadeIn(frame, 75, 15);
        return (
          <div
            style={{
              position: "absolute",
              bottom: 30,
              opacity: tagOpacity,
              fontSize: 14,
              fontFamily: "system-ui, sans-serif",
              color: MUTED,
              textAlign: "center",
            }}
          >
            Every simulation error becomes a query against the knowledgebase — the system improves indefinitely.
          </div>
        );
      })()}
    </AbsoluteFill>
  );
};
