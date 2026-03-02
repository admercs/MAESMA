import React from "react";
import {
  AbsoluteFill,
  interpolate,
  useCurrentFrame,
  spring,
  useVideoConfig,
} from "remotion";
import { BG, ACCENT, ACCENT2, TEXT, MUTED, fadeIn, slideUp } from "./shared";

export const TitleScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const titleOpacity = fadeIn(frame, 10);
  const titleY = slideUp(frame, 10, 25, 60);

  const subtitleOpacity = fadeIn(frame, 35);
  const subtitleY = slideUp(frame, 35, 20, 30);

  const taglineOpacity = fadeIn(frame, 55);

  const glowScale = spring({ frame: frame - 5, fps, config: { damping: 12 } });
  const glowOpacity = interpolate(frame, [5, 30], [0, 0.6], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

  return (
    <AbsoluteFill
      style={{
        backgroundColor: BG,
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      {/* Background glow */}
      <div
        style={{
          position: "absolute",
          width: 600,
          height: 600,
          borderRadius: "50%",
          background: `radial-gradient(circle, ${ACCENT}22 0%, transparent 70%)`,
          opacity: glowOpacity,
          transform: `scale(${glowScale})`,
        }}
      />

      {/* Title */}
      <div
        style={{
          opacity: titleOpacity,
          transform: `translateY(${titleY}px)`,
          textAlign: "center",
        }}
      >
        <div
          style={{
            fontSize: 72,
            fontWeight: 800,
            fontFamily: "system-ui, sans-serif",
            color: TEXT,
            letterSpacing: "-2px",
          }}
        >
          MAESMA
        </div>
      </div>

      {/* Subtitle */}
      <div
        style={{
          opacity: subtitleOpacity,
          transform: `translateY(${subtitleY}px)`,
          textAlign: "center",
          marginTop: 16,
        }}
      >
        <div
          style={{
            fontSize: 26,
            fontWeight: 400,
            fontFamily: "system-ui, sans-serif",
            color: ACCENT,
            letterSpacing: "4px",
            textTransform: "uppercase",
          }}
        >
          Modular Agentic Earth System
        </div>
        <div
          style={{
            fontSize: 26,
            fontWeight: 400,
            fontFamily: "system-ui, sans-serif",
            color: ACCENT,
            letterSpacing: "4px",
            textTransform: "uppercase",
            marginTop: 4,
          }}
        >
          Modeling Arena
        </div>
      </div>

      {/* Tagline */}
      <div
        style={{
          opacity: taglineOpacity,
          marginTop: 40,
          textAlign: "center",
          maxWidth: 700,
        }}
      >
        <div
          style={{
            fontSize: 18,
            fontFamily: "system-ui, sans-serif",
            color: MUTED,
            lineHeight: 1.6,
          }}
        >
          Autonomous AI discovers, assembles, and optimizes Earth system models
          — reasoning over a central Process Knowledgebase via a neural inference
          engine, driven by simulation errors.
        </div>
      </div>
    </AbsoluteFill>
  );
};
