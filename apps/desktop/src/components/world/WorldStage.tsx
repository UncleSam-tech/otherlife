import React from 'react';
import { EnvironmentLayer } from './EnvironmentLayer';
import { WeatherLayer } from './WeatherLayer';
import { SceneRenderer, TodaySceneDTO, LastStepResultDTO } from './SceneRenderer';
import { Compass } from 'lucide-react';

interface WorldStageProps {
  scene: TodaySceneDTO | null;
  lastStepResult: LastStepResultDTO | null;
  weatherName?: string;
}

export const WorldStage: React.FC<WorldStageProps> = ({
  scene,
  lastStepResult,
  weatherName = 'Harmattan Haze',
}) => {
  if (!scene) {
    return (
      <div className="flex-1 flex items-center justify-center p-8 bg-[#0b0d13]">
        <div className="flex flex-col items-center gap-3">
          <Compass className="w-8 h-8 text-amber-400/60 animate-spin" />
          <p className="text-slate-400 font-serif italic text-sm">Opening your eyes to the world...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto bg-[#0a0c12] px-6 lg:px-12 py-6 flex flex-col space-y-6 select-text max-w-4xl mx-auto">
      {/* 2D Illustrated Environment Layer with Dynamic Weather */}
      <div className="relative w-full rounded-2xl overflow-hidden shadow-2xl border border-amber-500/20">
        <EnvironmentLayer
          lifeStage={scene.life_stage}
          age={scene.age}
          locationFormatted={scene.location_formatted}
        />
        <WeatherLayer weatherName={weatherName} />
      </div>

      {/* Literary Living Scene Renderer */}
      <SceneRenderer
        scene={scene}
        lastStepResult={lastStepResult}
      />
    </div>
  );
};
