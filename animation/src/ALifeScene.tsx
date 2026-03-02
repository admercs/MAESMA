import React from "react";
import { AbsoluteFill, useCurrentFrame, useVideoConfig, spring } from "remotion";
import { BG, ACCENT, ACCENT2, ACCENT3, TEXT, MUTED, SURFACE, fadeIn, slideUp } from "./shared";

/* ─── ALife Process Lifecycle Scene ─── */
const TIER_COLORS: Record<string, string> = {
  Normal: ACCENT3,     // emerald
  LowCompute: "#f59e0b", // amber
  Critical: "#ef4444",   // red
  Archived: MUTED,
};

const TIERS = [
  { name: "Normal", radius: 38, x: 320, y: 260 },
  { name: "LowCompute", radius: 30, x: 560, y: 260 },
  { name: "Critical", radius: 22, x: 760, y: 260 },
  { name: "Archived", radius: 16, x: 920, y: 260 },
];

const LAWS = [
  { id: 1, name: "Conservation", desc: "mass / energy / charge" },
  { id: 2, name: "Earned Existence", desc: "skill / cost > 0" },
  { id: 3, name: "Provenance", desc: "immutable lineage" },
];

const REPLICATION_METHODS = ["Mutation", "Crossover", "Immigration", "Speciation"];

export const ALifeScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const titleScale = spring({ frame, fps, from: 0, to: 1, config: { damping: 15 } });

  /* Heartbeat pulse — continuous sine wave */
  const pulse = Math.sin((frame / fps) * Math.PI * 2) * 0.15 + 1;

  return (
    <AbsoluteFill style={{ backgroundColor: BG }}>
      {/* Title */}
      <div
        style={{
          position: "absolute",
          top: 50,
          left: 100,
          transform: `scale(${titleScale})`,
          transformOrigin: "left center",
        }}
      >
        <div style={{ fontSize: 42, fontWeight: 700, color: TEXT, letterSpacing: -1 }}>
          ALife Process Lifecycle
        </div>
        <div style={{ fontSize: 18, color: MUTED, marginTop: 4 }}>
          Processes are alive — they compete, replicate, and evolve
        </div>
      </div>

      {/* ─── Survival Tiers ─── */}
      <div
        style={{
          position: "absolute",
          top: 140,
          left: 100,
          opacity: fadeIn(frame, 20),
          transform: `translateY(${slideUp(frame, 20)}px)`,
          fontSize: 14,
          fontWeight: 600,
          color: ACCENT,
          textTransform: "uppercase",
          letterSpacing: 2,
        }}
      >
        Survival Tiers
      </div>

      {/* Tier circles */}
      <svg
        style={{ position: "absolute", top: 140, left: 0 }}
        width={1100}
        height={200}
        viewBox="0 0 1100 200"
      >
        {/* Flow arrows */}
        {[0, 1, 2].map((i) => {
          const from = TIERS[i];
          const to = TIERS[i + 1];
          const arrowOpacity = fadeIn(frame, 50 + i * 12);
          return (
            <g key={`arrow-${i}`} opacity={arrowOpacity}>
              <line
                x1={from.x + from.radius + 10}
                y1={from.y - 80}
                x2={to.x - to.radius - 10}
                y2={to.y - 80}
                stroke={MUTED}
                strokeWidth={1.5}
                strokeDasharray="4,4"
                markerEnd="url(#arrowhead)"
              />
              <text
                x={(from.x + to.x) / 2}
                y={from.y - 92}
                fill={MUTED}
                fontSize={10}
                textAnchor="middle"
              >
                demote
              </text>
            </g>
          );
        })}
        <defs>
          <marker id="arrowhead" markerWidth="6" markerHeight="4" refX="5" refY="2" orient="auto">
            <polygon points="0 0, 6 2, 0 4" fill={MUTED} />
          </marker>
        </defs>

        {/* Tier circles */}
        {TIERS.map((tier, i) => {
          const tierOpacity = fadeIn(frame, 30 + i * 10);
          const tierY = tier.y - 80;
          const tierPulse = tier.name === "Normal" ? pulse : 1;
          return (
            <g key={tier.name} opacity={tierOpacity}>
              <circle
                cx={tier.x}
                cy={tierY}
                r={tier.radius * tierPulse}
                fill="none"
                stroke={TIER_COLORS[tier.name]}
                strokeWidth={2.5}
                opacity={tier.name === "Archived" ? 0.4 : 1}
              />
              <text
                x={tier.x}
                y={tierY + 4}
                fill={TIER_COLORS[tier.name]}
                fontSize={tier.radius > 25 ? 13 : 10}
                fontWeight={600}
                textAnchor="middle"
              >
                {tier.name}
              </text>
              <text
                x={tier.x}
                y={tierY + tier.radius + 18}
                fill={MUTED}
                fontSize={10}
                textAnchor="middle"
              >
                {tier.name === "Normal" ? "1.0× budget" : tier.name === "LowCompute" ? "0.5× budget" : tier.name === "Critical" ? "0.1× budget" : "0× budget"}
              </text>
            </g>
          );
        })}
      </svg>

      {/* ─── Constitutional Invariants ─── */}
      <div
        style={{
          position: "absolute",
          top: 370,
          left: 100,
          opacity: fadeIn(frame, 80),
          transform: `translateY(${slideUp(frame, 80)}px)`,
        }}
      >
        <div
          style={{
            fontSize: 14,
            fontWeight: 600,
            color: ACCENT2,
            textTransform: "uppercase",
            letterSpacing: 2,
            marginBottom: 12,
          }}
        >
          Constitutional Invariants
        </div>
        <div style={{ display: "flex", gap: 16 }}>
          {LAWS.map((law, i) => (
            <div
              key={law.id}
              style={{
                background: SURFACE,
                borderRadius: 8,
                padding: "12px 20px",
                border: `1px solid ${ACCENT2}33`,
                opacity: fadeIn(frame, 90 + i * 12),
                transform: `translateY(${slideUp(frame, 90 + i * 12)}px)`,
                width: 240,
              }}
            >
              <div style={{ color: ACCENT2, fontSize: 13, fontWeight: 600 }}>
                Law {law.id}: {law.name}
              </div>
              <div style={{ color: MUTED, fontSize: 11, marginTop: 4 }}>{law.desc}</div>
            </div>
          ))}
        </div>
      </div>

      {/* ─── Heartbeat Daemon ─── */}
      <div
        style={{
          position: "absolute",
          top: 370,
          right: 100,
          opacity: fadeIn(frame, 100),
          transform: `translateY(${slideUp(frame, 100)}px)`,
          textAlign: "center",
        }}
      >
        <div
          style={{
            fontSize: 14,
            fontWeight: 600,
            color: "#ef4444",
            textTransform: "uppercase",
            letterSpacing: 2,
            marginBottom: 12,
          }}
        >
          Heartbeat Daemon
        </div>
        <div
          style={{
            width: 80,
            height: 80,
            borderRadius: "50%",
            border: `3px solid #ef4444`,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            margin: "0 auto",
            transform: `scale(${pulse})`,
            boxShadow: `0 0 ${20 * (pulse - 0.85)}px #ef444466`,
          }}
        >
          <span style={{ fontSize: 32 }}>💓</span>
        </div>
        <div style={{ color: MUTED, fontSize: 11, marginTop: 8 }}>
          tick() → evaluate → promote/demote
        </div>
      </div>

      {/* ─── Replication & Phylogenetic Tree ─── */}
      <div
        style={{
          position: "absolute",
          bottom: 100,
          left: 100,
          opacity: fadeIn(frame, 130),
          transform: `translateY(${slideUp(frame, 130)}px)`,
        }}
      >
        <div
          style={{
            fontSize: 14,
            fontWeight: 600,
            color: ACCENT3,
            textTransform: "uppercase",
            letterSpacing: 2,
            marginBottom: 12,
          }}
        >
          Self-Replication Methods
        </div>
        <div style={{ display: "flex", gap: 12 }}>
          {REPLICATION_METHODS.map((method, i) => (
            <div
              key={method}
              style={{
                background: SURFACE,
                borderRadius: 6,
                padding: "8px 16px",
                border: `1px solid ${ACCENT3}33`,
                color: TEXT,
                fontSize: 12,
                fontWeight: 500,
                opacity: fadeIn(frame, 140 + i * 8),
                transform: `translateY(${slideUp(frame, 140 + i * 8)}px)`,
              }}
            >
              {method}
            </div>
          ))}
        </div>
      </div>

      {/* Phylogenetic tree mini-diagram */}
      <svg
        style={{
          position: "absolute",
          bottom: 60,
          right: 100,
          opacity: fadeIn(frame, 150),
        }}
        width={400}
        height={160}
        viewBox="0 0 400 160"
      >
        {/* Root */}
        <circle cx={30} cy={80} r={6} fill={ACCENT3} />
        <text x={30} y={72} fill={MUTED} fontSize={9} textAnchor="middle">root</text>

        {/* Gen 1 */}
        <line x1={36} y1={80} x2={100} y2={40} stroke={ACCENT3} strokeWidth={1.5} opacity={fadeIn(frame, 160)} />
        <line x1={36} y1={80} x2={100} y2={120} stroke={ACCENT3} strokeWidth={1.5} opacity={fadeIn(frame, 162)} />
        <circle cx={100} cy={40} r={5} fill={ACCENT} opacity={fadeIn(frame, 160)} />
        <circle cx={100} cy={120} r={5} fill={ACCENT} opacity={fadeIn(frame, 162)} />

        {/* Gen 2 */}
        <line x1={105} y1={40} x2={180} y2={20} stroke={ACCENT} strokeWidth={1.5} opacity={fadeIn(frame, 170)} />
        <line x1={105} y1={40} x2={180} y2={60} stroke={ACCENT} strokeWidth={1.5} opacity={fadeIn(frame, 172)} />
        <line x1={105} y1={120} x2={180} y2={100} stroke={ACCENT} strokeWidth={1.5} opacity={fadeIn(frame, 174)} />
        <line x1={105} y1={120} x2={180} y2={140} stroke={ACCENT} strokeWidth={1.5} opacity={fadeIn(frame, 176)} />
        <circle cx={180} cy={20} r={4} fill={ACCENT2} opacity={fadeIn(frame, 170)} />
        <circle cx={180} cy={60} r={4} fill={ACCENT2} opacity={fadeIn(frame, 172)} />
        <circle cx={180} cy={100} r={4} fill={ACCENT2} opacity={fadeIn(frame, 174)} />
        <circle cx={180} cy={140} r={4} fill={ACCENT2} opacity={fadeIn(frame, 176)} />

        {/* Gen 3 — some archived */}
        <line x1={184} y1={20} x2={260} y2={10} stroke={ACCENT2} strokeWidth={1} opacity={fadeIn(frame, 185)} />
        <line x1={184} y1={20} x2={260} y2={30} stroke={ACCENT2} strokeWidth={1} opacity={fadeIn(frame, 187)} />
        <line x1={184} y1={60} x2={260} y2={50} stroke={ACCENT2} strokeWidth={1} opacity={fadeIn(frame, 189)} />
        <line x1={184} y1={60} x2={260} y2={70} stroke={MUTED} strokeWidth={1} strokeDasharray="3,3" opacity={fadeIn(frame, 191)} />
        <line x1={184} y1={100} x2={260} y2={90} stroke={ACCENT2} strokeWidth={1} opacity={fadeIn(frame, 193)} />
        <line x1={184} y1={140} x2={260} y2={130} stroke={MUTED} strokeWidth={1} strokeDasharray="3,3" opacity={fadeIn(frame, 195)} />
        <line x1={184} y1={140} x2={260} y2={150} stroke={ACCENT2} strokeWidth={1} opacity={fadeIn(frame, 197)} />

        {/* Leaf nodes */}
        {[10, 30, 50, 90, 150].map((y, i) => (
          <circle key={y} cx={260} cy={y} r={3.5} fill={ACCENT3} opacity={fadeIn(frame, 185 + i * 2)} />
        ))}
        {/* Archived leaves (dashed) */}
        {[70, 130].map((y, i) => (
          <circle key={y} cx={260} cy={y} r={3.5} fill="none" stroke={MUTED} strokeWidth={1} strokeDasharray="2,2" opacity={fadeIn(frame, 191 + i * 4)} />
        ))}

        {/* Label */}
        <text x={300} y={80} fill={MUTED} fontSize={10}>Phylogenetic</text>
        <text x={300} y={93} fill={MUTED} fontSize={10}>Lineage Tree</text>
      </svg>

      {/* ─── Process Soul badge ─── */}
      <div
        style={{
          position: "absolute",
          bottom: 30,
          left: 100,
          opacity: fadeIn(frame, 180),
          display: "flex",
          alignItems: "center",
          gap: 8,
          color: MUTED,
          fontSize: 11,
        }}
      >
        <span style={{ color: ACCENT2 }}>ProcessSoul</span>
        <span>→ identity · strengths · weaknesses · niches · modification history · tier history</span>
      </div>
    </AbsoluteFill>
  );
};
