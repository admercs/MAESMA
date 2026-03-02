import React from "react";
import {
  AbsoluteFill,
  interpolate,
  useCurrentFrame,
  spring,
  useVideoConfig,
} from "remotion";
import {
  BG,
  ACCENT,
  ACCENT2,
  ACCENT3,
  TEXT,
  MUTED,
  SURFACE,
  fadeIn,
  slideUp,
} from "./shared";

const NODE_COUNT = 18;

export const InferenceScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps, width, height } = useVideoConfig();

  const headerOpacity = fadeIn(frame, 5);
  const headerY = slideUp(frame, 5, 20, 30);

  /* Graph nodes positioned in a circle */
  const cx = width / 2;
  const cy = height / 2 + 30;
  const radius = 200;

  /* Pulsing "attention" connections */
  const pulsePhase = (frame / 30) * Math.PI * 2;

  return (
    <AbsoluteFill
      style={{
        backgroundColor: BG,
        justifyContent: "flex-start",
        alignItems: "center",
        paddingTop: 60,
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
        Neural Inference Engine
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
        Graph transformer reasons over the knowledgebase
      </div>

      {/* Graph visualization */}
      <svg width={width} height={height - 160} viewBox={`0 0 ${width} ${height - 160}`}>
        {/* Edges — animated attention lines */}
        {Array.from({ length: NODE_COUNT }).map((_, i) => {
          const angleI = (i / NODE_COUNT) * Math.PI * 2 - Math.PI / 2;
          const x1 = cx + Math.cos(angleI) * radius;
          const y1 = cy - 80 + Math.sin(angleI) * radius;

          // Connect to 3 neighbours
          return [1, 3, 7].map((offset) => {
            const j = (i + offset) % NODE_COUNT;
            const angleJ = (j / NODE_COUNT) * Math.PI * 2 - Math.PI / 2;
            const x2 = cx + Math.cos(angleJ) * radius;
            const y2 = cy - 80 + Math.sin(angleJ) * radius;

            const edgeDelay = 20 + i * 2;
            const edgeOpacity = fadeIn(frame, edgeDelay, 15);

            const pulse =
              0.15 +
              0.15 * Math.sin(pulsePhase + i * 0.5 + offset);

            return (
              <line
                key={`${i}-${j}`}
                x1={x1}
                y1={y1}
                x2={x2}
                y2={y2}
                stroke={ACCENT}
                strokeWidth={1.5}
                opacity={edgeOpacity * pulse}
              />
            );
          });
        })}

        {/* Nodes */}
        {Array.from({ length: NODE_COUNT }).map((_, i) => {
          const angle = (i / NODE_COUNT) * Math.PI * 2 - Math.PI / 2;
          const nx = cx + Math.cos(angle) * radius;
          const ny = cy - 80 + Math.sin(angle) * radius;

          const nodeDelay = 15 + i * 3;
          const nodeScale = spring({
            frame: frame - nodeDelay,
            fps,
            config: { damping: 12, stiffness: 140 },
          });
          const nodeOpacity = fadeIn(frame, nodeDelay, 10);

          const colors = [ACCENT, ACCENT2, ACCENT3];
          const color = colors[i % 3];

          const glow =
            0.4 + 0.3 * Math.sin(pulsePhase + i * 0.8);

          return (
            <g key={i}>
              {/* Glow ring */}
              <circle
                cx={nx}
                cy={ny}
                r={16}
                fill="none"
                stroke={color}
                strokeWidth={2}
                opacity={nodeOpacity * glow}
              />
              {/* Core node */}
              <circle
                cx={nx}
                cy={ny}
                r={8}
                fill={color}
                opacity={nodeOpacity * Math.min(nodeScale, 1)}
              />
            </g>
          );
        })}

        {/* Center label */}
        {(() => {
          const centerOpacity = fadeIn(frame, 60, 20);
          return (
            <g opacity={centerOpacity}>
              <circle
                cx={cx}
                cy={cy - 80}
                r={55}
                fill={`${SURFACE}`}
                stroke={ACCENT}
                strokeWidth={2}
                opacity={0.9}
              />
              <text
                x={cx}
                y={cy - 88}
                textAnchor="middle"
                fill={TEXT}
                fontSize={14}
                fontWeight={700}
                fontFamily="system-ui, sans-serif"
              >
                Graph
              </text>
              <text
                x={cx}
                y={cy - 72}
                textAnchor="middle"
                fill={TEXT}
                fontSize={14}
                fontWeight={700}
                fontFamily="system-ui, sans-serif"
              >
                Transformer
              </text>
            </g>
          );
        })()}
      </svg>

      {/* Bottom labels */}
      {(() => {
        const items = [
          { label: "Errors", icon: "⚠" },
          { label: "Uncertainty", icon: "◈" },
          { label: "Regime", icon: "◉" },
          { label: "Budget", icon: "⧫" },
        ];
        const labelsOpacity = fadeIn(frame, 80);
        return (
          <div
            style={{
              position: "absolute",
              bottom: 60,
              display: "flex",
              gap: 30,
              opacity: labelsOpacity,
            }}
          >
            <div
              style={{
                fontSize: 14,
                fontFamily: "system-ui, sans-serif",
                color: MUTED,
                marginRight: 8,
                alignSelf: "center",
              }}
            >
              Inputs →
            </div>
            {items.map((item, i) => (
              <div
                key={item.label}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 6,
                  opacity: fadeIn(frame, 85 + i * 6, 8),
                }}
              >
                <span style={{ fontSize: 18 }}>{item.icon}</span>
                <span
                  style={{
                    fontSize: 15,
                    fontFamily: "system-ui, sans-serif",
                    color: TEXT,
                    fontWeight: 500,
                  }}
                >
                  {item.label}
                </span>
              </div>
            ))}
            <div
              style={{
                fontSize: 14,
                fontFamily: "system-ui, sans-serif",
                color: MUTED,
                marginLeft: 8,
                alignSelf: "center",
              }}
            >
              → Proposals
            </div>
          </div>
        );
      })()}
    </AbsoluteFill>
  );
};
