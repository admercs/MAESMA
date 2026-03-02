import React from "react";
import { AbsoluteFill, Sequence } from "remotion";
import { TitleScene } from "./TitleScene";
import { KnowledgebaseScene } from "./KnowledgebaseScene";
import { InferenceScene } from "./InferenceScene";
import { AgentSwarmScene } from "./AgentSwarmScene";
import { LoopScene } from "./LoopScene";
import { DiscoveryScene } from "./DiscoveryScene";
import { ALifeScene } from "./ALifeScene";
import { FullSystemScene } from "./FullSystemScene";
import { BG } from "./shared";

/**
 * MAESMA system animation — 8 scenes at 30fps, ~58 seconds total.
 *
 * Scene layout (frames @ 30fps):
 *   0–239    Title (8s)
 * 240–479    Process Knowledgebase (8s)
 * 480–719    Neural Inference Engine (8s)
 * 720–959    Agent Swarm (8s)
 * 960–1199   Autonomous Optimization Loop (8s)
 * 1200–1439  Process Discovery Pipeline (8s)
 * 1440–1679  ALife Process Lifecycle (8s)
 * 1680–1979  Full System Architecture (10s finale)
 *
 * Each scene is self-contained with its own fade-in animations.
 */
export const MAESMAAnimation: React.FC = () => {
  const SCENE_DUR = 240; // 8 seconds per scene
  const FINALE_DUR = 300; // 10 seconds for the finale

  return (
    <AbsoluteFill style={{ backgroundColor: BG }}>
      {/* Scene 1: Title */}
      <Sequence from={0} durationInFrames={SCENE_DUR}>
        <TitleScene />
      </Sequence>

      {/* Scene 2: Process Knowledgebase */}
      <Sequence from={SCENE_DUR} durationInFrames={SCENE_DUR}>
        <KnowledgebaseScene />
      </Sequence>

      {/* Scene 3: Neural Inference Engine */}
      <Sequence from={SCENE_DUR * 2} durationInFrames={SCENE_DUR}>
        <InferenceScene />
      </Sequence>

      {/* Scene 4: Agent Swarm */}
      <Sequence from={SCENE_DUR * 3} durationInFrames={SCENE_DUR}>
        <AgentSwarmScene />
      </Sequence>

      {/* Scene 5: Autonomous Optimization Loop */}
      <Sequence from={SCENE_DUR * 4} durationInFrames={SCENE_DUR}>
        <LoopScene />
      </Sequence>

      {/* Scene 6: Process Discovery Pipeline */}
      <Sequence from={SCENE_DUR * 5} durationInFrames={SCENE_DUR}>
        <DiscoveryScene />
      </Sequence>

      {/* Scene 7: ALife Process Lifecycle */}
      <Sequence from={SCENE_DUR * 6} durationInFrames={SCENE_DUR}>
        <ALifeScene />
      </Sequence>

      {/* Scene 8: Full System Architecture (finale) */}
      <Sequence from={SCENE_DUR * 7} durationInFrames={FINALE_DUR}>
        <FullSystemScene />
      </Sequence>
    </AbsoluteFill>
  );
};
