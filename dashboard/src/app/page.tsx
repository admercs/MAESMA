import { Sidebar } from "@/components/Sidebar";
import { KbSummaryCard } from "@/components/cards/KbSummaryCard";
import { SapgGraphCard } from "@/components/cards/SapgGraphCard";
import { SkillEvolutionCard } from "@/components/cards/SkillEvolutionCard";
import { AgentStatusCard } from "@/components/cards/AgentStatusCard";
import { RegimeMapCard } from "@/components/cards/RegimeMapCard";
import { ParetoFrontCard } from "@/components/cards/ParetoFrontCard";
import { SurvivalTierCard } from "@/components/cards/SurvivalTierCard";
import { HeartbeatCard } from "@/components/cards/HeartbeatCard";

export default function Home() {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 overflow-auto p-6">
        <header className="mb-8">
          <h1 className="text-2xl font-bold">MAESMA Dashboard</h1>
          <p className="text-sm" style={{ color: "var(--text-secondary)" }}>
            Modular Agentic Earth System Modeling Arena
          </p>
        </header>

        <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
          {/* Row 1: Overview */}
          <KbSummaryCard />
          <AgentStatusCard />
          <ParetoFrontCard />

          {/* Row 2: ALife Process Lifecycle */}
          <SurvivalTierCard />
          <HeartbeatCard />
          <SkillEvolutionCard />

          {/* Row 3: Visualization */}
          <div className="lg:col-span-2">
            <SapgGraphCard />
          </div>

          {/* Row 4: Geospatial */}
          <div className="xl:col-span-3">
            <RegimeMapCard />
          </div>
        </div>
      </main>
    </div>
  );
}
