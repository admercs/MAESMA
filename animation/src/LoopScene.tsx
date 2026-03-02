import React from "react";
import {
  AbsoluteFill,
  interpolate,
  useCurrentFrame,
  spring,
  useVideoConfig,
} from "remotion";
import { BG, ACCENT, ACCENT2, ACCENT3, TEXT, MUTED, SURFACE, fadeIn, slideUp } from "./shared";

const STEPS = [
  { num: "1", label: "Discover Data", desc: "Crawl STAC / CMR / CKAN catalogs", color: "#38bdf8" },
  { num: "2", label: "Assemble Models", desc: "Neural inference queries KB → build SAPG", color: "#818cf8" },
  { num: "3", label: "Benchmark & Score", desc: "Run against observations → skill scores", color: "#34d399" },
  { num: "4", label: "Select Optimal", desc: "Neural inference + Bayesian selection", color: "#fbbf24" },
  { num: "5", label: "Discover Processes", desc: "Residual → learn → validate → deposit into KB", color: "#fb923c" },
  { num: "6", label: "Update KB", desc: "Deposit skill records + refine ontology", color: "#f472b6" },
];

export const LoopScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps, width, height } = useVideoConfig();

  const headerOpacity = fadeIn(frame, 5);
  const headerY = slideUp(frame, 5, 20, 30);

  const cx = width / 2;
  const cy = height / 2 + 30;
  const radius = 195;

  /* Rotating highlight indicator */
  const cycleLen = 120; // frames per full cycle
  const activeIndex = Math.floor(((frame - 40) / 20) % 6);
  const cycleProgress = ((frame - 40) / cycleLen) * Math.PI * 2;

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
        Autonomous Optimization Loop
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
        Runs indefinitely — humans observe, never gate
      </div>

      {/* Circle of steps */}
      <svg width={width} height={height - 160} viewBox={`0 0 ${width} ${height - 160}`}>
        {/* Connecting arc */}
        {(() => {
          const arcOpacity = fadeIn(frame, 30, 20);
          return (
            <circle
              cx={cx}
              cy={cy - 80}
              r={radius}
              fill="none"
              stroke={`${ACCENT}22`}
              strokeWidth={2}
              strokeDasharray="8 4"
              opacity={arcOpacity}
            />
          );
        })()}

        {/* Rotating pulse on arc */}
        {frame > 40 && (
          <circle
            cx={cx + Math.cos(cycleProgress - Math.PI / 2) * radius}
            cy={cy - 80 + Math.sin(cycleProgress - Math.PI / 2) * radius}
            r={6}
            fill={ACCENT}
            opacity={0.8}
          >
          </circle>
        )}

        {/* Arrow segments between steps */}
        {STEPS.map((_, i) => {
          const a1 = (i / STEPS.length) * Math.PI * 2 - Math.PI / 2;
          const a2 = ((i + 1) / STEPS.length) * Math.PI * 2 - Math.PI / 2;
          const midA = (a1 + a2) / 2;

          const arrowX = cx + Math.cos(midA) * (radius + 2);
          const arrowY = cy - 80 + Math.sin(midA) * (radius + 2);

          const arrowOpacity = fadeIn(frame, 35 + i * 8, 10);

          return (
            <text
              key={`arrow-${i}`}
              x={arrowX}
              y={arrowY}
              textAnchor="middle"
              dominantBaseline="middle"
              fontSize={16}
              fill={ACCENT}
              opacity={arrowOpacity}
              transform={`rotate(${(midA * 180) / Math.PI + 90}, ${arrowX}, ${arrowY})`}
            >
              ▸
            </text>
          );
        })}

        {/* Step nodes */}
        {STEPS.map((step, i) => {
          const angle = (i / STEPS.length) * Math.PI * 2 - Math.PI / 2;
          const nx = cx + Math.cos(angle) * radius;
          const ny = cy - 80 + Math.sin(angle) * radius;

          const delay = 20 + i * 10;
          const opacity = fadeIn(frame, delay, 12);
          const scale = spring({
            frame: frame - delay,
            fps,
            config: { damping: 14 },
          });

          const isActive = frame > 40 && i === activeIndex;

          return (
            <g key={step.num} opacity={opacity}>
              {/* Glow when active */}
              {isActive && (
                <circle
                  cx={nx}
                  cy={ny}
                  r={52}
                  fill="none"
                  stroke={step.color}
                  strokeWidth={2}
                  opacity={0.6}
                />
              )}
              {/* Background circle */}
              <circle
                cx={nx}
                cy={ny}
                r={42}
                fill={SURFACE}
                stroke={step.color}
                strokeWidth={isActive ? 2.5 : 1.5}
                opacity={Math.min(scale, 1)}
              />
              {/* Number */}
              <text
                x={nx}
                y={ny - 10}
                textAnchor="middle"
                dominantBaseline="middle"
                fontSize={22}
                fontWeight={800}
                fill={step.color}
                fontFamily="system-ui, sans-serif"
              >
                {step.num}
              </text>
              {/* Label */}
              <text
                x={nx}
                y={ny + 10}
                textAnchor="middle"
                dominantBaseline="middle"
                fontSize={9}
                fontWeight={600}
                fill={TEXT}
                fontFamily="system-ui, sans-serif"
              >
                {step.label}
              </text>
            </g>
          );
        })}

        {/* Center "repeat forever" */}
        {(() => {
          const centerOpacity = fadeIn(frame, 75, 15);
          return (
            <g opacity={centerOpacity}>
              <text
                x={cx}
                y={cy - 88}
                textAnchor="middle"
                fontSize={14}
                fontWeight={600}
                fill={MUTED}
                fontFamily="system-ui, sans-serif"
              >
                ∞
              </text>
              <text
                x={cx}
                y={cy - 72}
                textAnchor="middle"
                fontSize={11}
                fill={MUTED}
                fontFamily="system-ui, sans-serif"
              >
                repeat forever
              </text>
            </g>
          );
        })()}
      </svg>
    </AbsoluteFill>
  );
};
