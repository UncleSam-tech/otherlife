import React from 'react';
import { ActionPromptBar } from './ActionPromptBar';
import { Sparkles, Calendar, MapPin, CheckCircle2 } from 'lucide-react';

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
      <div className="flex-1 flex items-center justify-center p-8 bg-slate-950">
        <p className="text-slate-500 font-mono">Loading living stage...</p>
      </div>
    );
  }

  return (
    <main className="flex-1 overflow-y-auto bg-slate-950 px-8 py-8 flex flex-col justify-between max-w-4xl mx-auto">
      <div className="space-y-6">
        {/* Stage Header Banner */}
        <div className="border-b border-slate-800/80 pb-5">
          <div className="flex items-center gap-4 text-xs font-mono text-emerald-400 mb-2">
            <span className="flex items-center gap-1.5 bg-emerald-500/10 px-2.5 py-1 rounded-md border border-emerald-500/20">
              <Calendar className="w-3.5 h-3.5" />
              {scene.date_formatted}
            </span>
            <span className="flex items-center gap-1.5 bg-slate-800/80 text-slate-300 px-2.5 py-1 rounded-md border border-slate-700/50">
              <MapPin className="w-3.5 h-3.5" />
              {scene.location_formatted}
            </span>
            <span className="bg-slate-800/80 text-slate-300 px-2.5 py-1 rounded-md border border-slate-700/50">
              AGE {scene.age} · {scene.life_stage.toUpperCase()}
            </span>
          </div>

          <h2 className="text-2xl font-serif font-bold text-slate-100 tracking-tight mt-3">
            {scene.headline}
          </h2>
        </div>

        {/* Previous Action Outcome Feedback (if any) */}
        {lastStepResult && (
          <div className="bg-slate-900/90 border border-emerald-500/30 rounded-xl p-5 shadow-lg space-y-2 animate-fadeIn">
            <div className="flex items-center gap-2 text-emerald-400 font-medium text-sm">
              <CheckCircle2 className="w-4 h-4" />
              <span>Consequence ({lastStepResult.days_advanced} days elapsed)</span>
            </div>
            <p className="text-slate-200 text-sm leading-relaxed">{lastStepResult.narrative}</p>
            <div className="pt-2 border-t border-slate-800/60 flex items-center gap-2 text-xs text-slate-400 font-mono">
              <Sparkles className="w-3.5 h-3.5 text-emerald-400" />
              <span>Causal origin: {lastStepResult.causality_note}</span>
            </div>
          </div>
        )}

        {/* Current Living Narrative */}
        <div className="bg-slate-900/50 border border-slate-800 rounded-xl p-6 backdrop-blur-sm space-y-4">
          <p className="text-slate-200 font-serif text-lg leading-relaxed">{scene.narrative}</p>

          {scene.circumstances.length > 0 && (
            <div className="pt-4 border-t border-slate-800/60">
              <p className="text-xs font-mono uppercase tracking-wider text-slate-500 mb-2">
                Current Circumstances & Active Dynamics
              </p>
              <div className="flex flex-wrap gap-2">
                {scene.circumstances.map((c, i) => (
                  <span
                    key={i}
                    className="text-xs bg-slate-800 text-slate-300 px-3 py-1 rounded-full border border-slate-700/60"
                  >
                    {c}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Intention Input Bar */}
      <div className="mt-8 pt-6 border-t border-slate-800/80">
        <ActionPromptBar
          onSubmitIntent={onSubmitIntent}
          suggestions={scene.prompt_suggestions}
          isLoading={isLoading}
        />
      </div>
    </main>
  );
};
