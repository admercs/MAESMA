import React from "react";
import {
  AbsoluteFill,
  interpolate,
  useCurrentFrame,
  spring,
  useVideoConfig,
} from "remotion";
import { BG, ACCENT, ACCENT2, ACCENT3, TEXT, MUTED, SURFACE, fadeIn, slideUp } from "./shared";

const PIPELINE = [
  { step: "1", label: "Detect", desc: "Structured bias persisting across calibrations", icon: "⚠" },
  { step: "2", label: "Diagnose", desc: "Attribute to variables, regions, regimes", icon: "🔍" },
  { step: "3", label: "Hypothesize", desc: "Missing coupling / feedback / process", icon: "💡" },
  { step: "4", label: "Learn", desc: "Neural operator, symbolic regression, hybrid", icon: "🧠" },
  { step: "5", label: "Validate", desc: "Conservation, stability, generalization", icon: "✓" },
  { step: "6", label: "Deposit", desc: "Auto-manifest + deposit into Knowledgebase", icon: "📥" },
];

export const DiscoveryScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps, width, height } = useVideoConfig();

  const headerOpacity = fadeIn(frame, 5);
  const headerY = slideUp(frame, 5, 20, 30);

  /* Animate which pipeline step is highlighted */
  const activeStep = Math.min(
    Math.floor(interpolate(frame, [25, 115], [0, 6], {
      extrapolateLeft: "clamp",
      extrapolateRight: "clamp",
    })),
    5
  );

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
        Process Discovery
      </div>
      <div
        style={{
          opacity: headerOpacity,
          fontSize: 16,
          fontFamily: "system-ui, sans-serif",
          color: MUTED,
          marginBottom: 30,
        }}
      >
        Errors → Learn → Validate → Deposit into Knowledgebase
      </div>

      {/* Error signal visualization at top */}
      {(() => {
        const errorOpacity = fadeIn(frame, 15, 15);
        return (
          <div
            style={{
              opacity: errorOpacity,
              display: "flex",
              alignItems: "center",
              gap: 8,
              marginBottom: 24,
              padding: "10px 24px",
              borderRadius: 12,
              background: `${SURFACE}`,
              border: `1px solid #ef444466`,
            }}
          >
            <span style={{ fontSize: 20 }}>⚠</span>
            <div style={{ fontFamily: "system-ui, sans-serif", color: "#ef4444", fontSize: 14, fontWeight: 600 }}>
              Simulation Error Detected
            </div>
            <div style={{ fontFamily: "system-ui, sans-serif", color: MUTED, fontSize: 13, marginLeft: 12 }}>
              Streamflow timing bias — persistent across calibrations
            </div>
          </div>
        );
      })()}

      {/* Pipeline steps — vertical flow */}
      <div
        style={{
          display: "flex",
          flexDirection: "column",
          gap: 6,
          width: 600,
        }}
      >
        {PIPELINE.map((item, i) => {
          const delay = 25 + i * 15;
          const opacity = fadeIn(frame, delay, 12);
          const x = slideUp(frame, delay, 15, 30);
          const isActive = i <= activeStep;
          const isCurrentStep = i === activeStep;

          const colors = [
            "#ef4444", // detect - red
            "#fb923c", // diagnose - orange
            "#fbbf24", // hypothesize - yellow
            "#818cf8", // learn - indigo
            "#34d399", // validate - green
            "#38bdf8", // deposit - sky
          ];

          return (
            <div
              key={item.step}
              style={{
                opacity,
                transform: `translateY(${x}px)`,
                display: "flex",
                alignItems: "center",
                gap: 16,
                padding: "12px 20px",
                borderRadius: 10,
                background: isCurrentStep ? `${colors[i]}11` : SURFACE,
                border: `1px solid ${isActive ? colors[i] + "88" : colors[i] + "22"}`,
                transition: "all 0.3s",
              }}
            >
              {/* Step number */}
              <div
                style={{
                  width: 32,
                  height: 32,
                  borderRadius: "50%",
                  background: isActive ? colors[i] : `${colors[i]}33`,
                  display: "flex",
                  justifyContent: "center",
                  alignItems: "center",
                  fontSize: 14,
                  fontWeight: 700,
                  color: isActive ? BG : colors[i],
                  fontFamily: "system-ui, sans-serif",
                  flexShrink: 0,
                }}
              >
                {item.step}
              </div>

              {/* Label + desc */}
              <div style={{ flex: 1 }}>
                <div
                  style={{
                    fontSize: 18,
                    fontWeight: 600,
                    color: isActive ? TEXT : MUTED,
                    fontFamily: "system-ui, sans-serif",
                  }}
                >
                  {item.label}
                </div>
                <div
                  style={{
                    fontSize: 12,
                    color: MUTED,
                    fontFamily: "system-ui, sans-serif",
                    marginTop: 2,
                  }}
                >
                  {item.desc}
                </div>
              </div>

              {/* Arrow connector (except last) */}
              {i < PIPELINE.length - 1 && isActive && (
                <div
                  style={{
                    position: "absolute",
                    left: 34,
                    bottom: -8,
                    fontSize: 14,
                    color: colors[i],
                  }}
                >
                  ↓
                </div>
              )}
            </div>
          );
        })}
      </div>

      {/* KB deposit result */}
      {(() => {
        const depositOpacity = fadeIn(frame, 110, 15);
        return (
          <div
            style={{
              opacity: depositOpacity,
              marginTop: 20,
              display: "flex",
              alignItems: "center",
              gap: 8,
              padding: "10px 24px",
              borderRadius: 12,
              background: `${ACCENT3}11`,
              border: `1px solid ${ACCENT3}66`,
            }}
          >
            <span style={{ fontSize: 18, color: ACCENT3 }}>✓</span>
            <div style={{ fontFamily: "system-ui, sans-serif", color: ACCENT3, fontSize: 14, fontWeight: 600 }}>
              New process deposited into Knowledgebase
            </div>
            <div style={{ fontFamily: "system-ui, sans-serif", color: MUTED, fontSize: 13, marginLeft: 12 }}>
              lateral subsurface flow — origin: discovered — status: provisional
            </div>
          </div>
        );
      })()}
    </AbsoluteFill>
  );
};
