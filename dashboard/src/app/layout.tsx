import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "MAESMA Dashboard",
  description:
    "Agentic AI for Autonomous Earth System Observation, Model Discovery, and Simulation — Monitoring & Control",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="antialiased">{children}</body>
    </html>
  );
}
