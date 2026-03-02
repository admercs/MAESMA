import React from "react";
import { Composition } from "remotion";
import { MAESMAAnimation } from "./MAESMAAnimation";

export const RemotionRoot: React.FC = () => {
  return (
    <>
      <Composition
        id="MAESMAAnimation"
        component={MAESMAAnimation}
        durationInFrames={1980}
        fps={30}
        width={1920}
        height={1080}
      />
    </>
  );
};
