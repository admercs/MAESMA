"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const navItems = [
  { href: "/", label: "Overview", icon: "◉" },
  { href: "/knowledgebase", label: "Knowledgebase", icon: "📚" },
  { href: "/sapg", label: "Process Graph", icon: "🔗" },
  { href: "/agents", label: "Agent Swarm", icon: "🤖" },
  { href: "/skills", label: "Skill Scores", icon: "📊" },
  { href: "/optimization", label: "Optimization", icon: "⚡" },
  { href: "/evolution", label: "Process Evolution", icon: "🧬" },
  { href: "/foundation-models", label: "Foundation Models", icon: "🌀" },
  { href: "/observations", label: "Observation Intel", icon: "🛰️" },
  { href: "/regimes", label: "Regime Map", icon: "🗺️" },
  { href: "/federation", label: "Federation", icon: "🌐" },
  { href: "/simulation", label: "Simulation", icon: "▶️" },
  { href: "/provenance", label: "Provenance", icon: "🔍" },
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <nav className="sidebar w-64 flex-shrink-0 flex flex-col h-screen">
      {/* Logo */}
      <div className="p-5 border-b" style={{ borderColor: "var(--border)" }}>
        <h2 className="text-lg font-bold tracking-tight">MAESMA</h2>
        <p className="text-xs" style={{ color: "var(--text-secondary)" }}>
          Earth System Modeling Arena
        </p>
      </div>

      {/* Navigation */}
      <div className="flex-1 overflow-auto py-4 px-3 space-y-1">
        {navItems.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            className={`sidebar-link ${pathname === item.href ? "active" : ""}`}
          >
            <span className="text-base">{item.icon}</span>
            <span className="text-sm">{item.label}</span>
          </Link>
        ))}
      </div>

      {/* Footer */}
      <div
        className="p-4 border-t text-xs"
        style={{ borderColor: "var(--border)", color: "var(--text-secondary)" }}
      >
        <div className="flex items-center gap-2">
          <span
            className="w-2 h-2 rounded-full"
            style={{ background: "var(--accent-green)" }}
          />
          API Connected
        </div>
      </div>
    </nav>
  );
}
