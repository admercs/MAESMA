import React from "react";
import {
  AbsoluteFill,
  interpolate,
  useCurrentFrame,
  spring,
  useVideoConfig,
} from "remotion";

/* ─── colour tokens ─── */
const BG = "#0b1120";
const ACCENT = "#38bdf8"; // sky-400
const ACCENT2 = "#818cf8"; // indigo-400
const ACCENT3 = "#34d399"; // emerald-400
const TEXT = "#e2e8f0";
const MUTED = "#94a3b8";
const SURFACE = "#1e293b";

/* ─── shared helpers ─── */
export const fadeIn = (frame: number, start: number, dur = 15) =>
  interpolate(frame, [start, start + dur], [0, 1], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

export const slideUp = (frame: number, start: number, dur = 20, px = 40) =>
  interpolate(frame, [start, start + dur], [px, 0], {
    extrapolateLeft: "clamp",
    extrapolateRight: "clamp",
  });

export { BG, ACCENT, ACCENT2, ACCENT3, TEXT, MUTED, SURFACE };
