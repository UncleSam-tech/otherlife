import React from 'react';
import { ActionPromptBar } from './ActionPromptBar';
import { IllustratedWorldLayer } from './IllustratedWorldLayer';
import { Sparkles, Compass } from 'lucide-react';

export interface TodaySceneDTO {
  greeting: string;
  date_formatted: string;
  location_formatted: string;
  age: number;
  life_stage: string;
  headline: string;
  narrative: string;
  circumstances: string[];
  prompt_suggestions: string[];
}

export interface LastStepResultDTO {
  success: boolean;
  narrative: string;
  causality_note: string;
  days_advanced: number;
}

interface CenterLivingStageProps {
  scene: TodaySceneDTO | null;
  lastStepResult: LastStepResultDTO | null;
  onSubmitIntent: (intent: string) => void;
  isLoading: boolean;
}

export const CenterLivingStage: React.FC<CenterLivingStageProps> = ({
  scene,
  lastStepResult,
  onSubmitIntent,
  isLoading,
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

  // Parse location and date into poetic headline
  const headlineParts = scene.headline.replace('Life in ', '').split('·');
  const chapterLocation = headlineParts[0]?.trim() || scene.location_formatted;

  return (
    <main className="flex-1 overflow-y-auto bg-[#0b0d13] px-6 lg:px-12 py-8 flex flex-col justify-between max-w-4xl mx-auto space-y-8 select-text">
      <div className="space-y-6">
        {/* 1. Illustrated 2D World Layer Canvas */}
        <IllustratedWorldLayer
          lifeStage={scene.life_stage}
          age={scene.age}
          locationFormatted={scene.location_formatted}
          timeFormatted={scene.date_formatted}
        />

        {/* 2. Cinematic Headline & Chapter Intro */}
        <div className="border-b border-slate-800/60 pb-4">
          <p className="text-xs uppercase tracking-widest text-amber-400/80 font-mono mb-1">
            {scene.life_stage} · Age {scene.age}
          </p>
          <h2 className="text-2xl lg:text-3xl font-serif font-bold text-slate-100 tracking-tight leading-snug">
            {scene.date_formatted} · {chapterLocation}
          </h2>
        </div>

        {/* 3. Consequence Passage (if previous intention was executed) */}
        {lastStepResult && (
          <div className="relative bg-gradient-to-r from-amber-950/20 via-slate-900/40 to-slate-900/20 border-l-2 border-amber-500/80 rounded-r-xl p-5 space-y-2.5 shadow-md">
            <div className="flex items-center gap-2 text-amber-300 text-xs font-mono uppercase tracking-wider">
              <Sparkles className="w-3.5 h-3.5 text-amber-400" />
              <span>How recent days unfolded ({lastStepResult.days_advanced} days elapsed)</span>
            </div>
            <p className="text-slate-200 font-serif text-base leading-relaxed italic">
              "{lastStepResult.narrative}"
            </p>
            <div className="pt-2 border-t border-slate-800/40 text-xs text-slate-400 font-sans">
              <span className="text-slate-500 font-serif">Milestone: </span>
              {lastStepResult.causality_note}
            </div>
          </div>
        )}

        {/* 4. Living Memoir Prose Scene */}
        <article className="space-y-4 pt-1">
          <p className="text-slate-200 font-serif text-lg lg:text-xl leading-relaxed tracking-normal font-normal">
            {scene.narrative}
          </p>

          {/* Environmental Sensory Observations */}
          {scene.circumstances.length > 0 && (
            <div className="pt-4 mt-4 border-t border-slate-800/40">
              <p className="text-xs font-serif italic text-amber-300/70 mb-2.5">
                You notice in your surroundings:
              </p>
              <div className="flex flex-wrap gap-2">
                {scene.circumstances.map((c, i) => (
                  <span
                    key={i}
                    className="text-xs font-sans bg-slate-900/80 text-slate-300 px-3.5 py-1.5 rounded-full border border-slate-800 shadow-sm"
                  >
                    • {c}
                  </span>
                ))}
              </div>
            </div>
          )}
        </article>
      </div>

      {/* 5. Natural Intention Prompt Bar */}
      <div className="pt-6 border-t border-slate-800/80">
        <ActionPromptBar
          onSubmitIntent={onSubmitIntent}
          suggestions={scene.prompt_suggestions}
          isLoading={isLoading}
        />
      </div>
    </main>
  );
};
