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

const LAYERS = [
  { label: "Code", desc: "Runnable implementations", color: "#38bdf8" },
  { label: "Manifest", desc: "Machine-readable metadata", color: "#818cf8" },
  { label: "Ontology", desc: "Relations & regime tags", color: "#a78bfa" },
  { label: "Skill", desc: "Empirical performance records", color: "#34d399" },
  { label: "Provenance", desc: "Origin & lineage tracking", color: "#fb923c" },
];

export const KnowledgebaseScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const headerOpacity = fadeIn(frame, 5);
  const headerY = slideUp(frame, 5, 20, 30);

  return (
    <AbsoluteFill
      style={{
        backgroundColor: BG,
        justifyContent: "center",
        alignItems: "center",
        flexDirection: "column",
      }}
    >
      {/* Section header */}
      <div
        style={{
          opacity: headerOpacity,
          transform: `translateY(${headerY}px)`,
          fontSize: 42,
          fontWeight: 700,
          fontFamily: "system-ui, sans-serif",
          color: TEXT,
          marginBottom: 12,
        }}
      >
        Process Knowledgebase
      </div>
      <div
        style={{
          opacity: headerOpacity,
          fontSize: 16,
          fontFamily: "system-ui, sans-serif",
          color: MUTED,
          marginBottom: 40,
        }}
      >
        Central versioned store — the single source of truth
      </div>

      {/* KB container */}
      <div
        style={{
          position: "relative",
          width: 700,
          display: "flex",
          flexDirection: "column",
          gap: 8,
        }}
      >
        {LAYERS.map((layer, i) => {
          const delay = 20 + i * 12;
          const opacity = fadeIn(frame, delay, 12);
          const x = slideUp(frame, delay, 15, 50);
          const scale = spring({
            frame: frame - delay,
            fps,
            config: { damping: 14, stiffness: 120 },
          });

          return (
            <div
              key={layer.label}
              style={{
                opacity,
                transform: `translateY(${x}px) scale(${Math.min(scale, 1)})`,
                display: "flex",
                alignItems: "center",
                gap: 16,
                padding: "14px 24px",
                borderRadius: 12,
                background: SURFACE,
                border: `1px solid ${layer.color}44`,
              }}
            >
              {/* Colored dot */}
              <div
                style={{
                  width: 14,
                  height: 14,
                  borderRadius: "50%",
                  background: layer.color,
                  boxShadow: `0 0 12px ${layer.color}88`,
                  flexShrink: 0,
                }}
              />
              <div>
                <div
                  style={{
                    fontSize: 20,
                    fontWeight: 600,
                    color: TEXT,
                    fontFamily: "system-ui, sans-serif",
                  }}
                >
                  {layer.label}
                </div>
                <div
                  style={{
                    fontSize: 14,
                    color: MUTED,
                    fontFamily: "system-ui, sans-serif",
                  }}
                >
                  {layer.desc}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Operations label */}
      {(() => {
        const ops = ["Query", "Reason", "Deposit", "Update", "Federate"];
        const opsDelay = 85;
        const opsOpacity = fadeIn(frame, opsDelay);
        return (
          <div
            style={{
              opacity: opsOpacity,
              marginTop: 36,
              display: "flex",
              gap: 18,
            }}
          >
            {ops.map((op, i) => {
              const d = opsDelay + 4 + i * 6;
              const o = fadeIn(frame, d, 8);
              return (
                <div
                  key={op}
                  style={{
                    opacity: o,
                    padding: "8px 18px",
                    borderRadius: 20,
                    fontSize: 14,
                    fontWeight: 600,
                    fontFamily: "system-ui, sans-serif",
                    color: ACCENT,
                    border: `1px solid ${ACCENT}66`,
                    background: `${ACCENT}11`,
                  }}
                >
                  {op}
                </div>
              );
            })}
          </div>
        );
      })()}
    </AbsoluteFill>
  );
};
