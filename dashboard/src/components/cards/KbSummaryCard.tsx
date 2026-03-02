"use client";

import { useEffect, useState } from "react";

interface KbStats {
  manifests: number;
  skill_records: number;
}

export function KbSummaryCard() {
  const [stats, setStats] = useState<KbStats>({
    manifests: 0,
    skill_records: 0,
  });

  useEffect(() => {
    fetch("/api/v1/kb/stats")
      .then((r) => r.json())
      .then(setStats)
      .catch(() => {});
  }, []);

  return (
    <div className="card">
      <h3 className="text-sm font-semibold mb-4 uppercase tracking-wider" style={{ color: "var(--text-secondary)" }}>
        Knowledgebase
      </h3>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <div className="text-3xl font-bold" style={{ color: "var(--accent-blue)" }}>
            {stats.manifests}
          </div>
          <div className="text-xs" style={{ color: "var(--text-secondary)" }}>
            Process Manifests
          </div>
        </div>
        <div>
          <div className="text-3xl font-bold" style={{ color: "var(--accent-green)" }}>
            {stats.skill_records}
          </div>
          <div className="text-xs" style={{ color: "var(--text-secondary)" }}>
            Skill Records
          </div>
        </div>
      </div>
      <div className="mt-4 text-xs" style={{ color: "var(--text-secondary)" }}>
        11 process families &middot; 4 fidelity rungs (R0–R3)
      </div>
    </div>
  );
}
