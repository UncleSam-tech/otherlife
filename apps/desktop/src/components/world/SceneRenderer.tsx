import React from 'react';
import { Sparkles, Eye } from 'lucide-react';

export interface TodaySceneDTO {
  greeting?: string;
  date_formatted?: string;
  location_formatted?: string;
  age?: number;
  life_stage?: string;
  headline: string;
  narrative: string;
  circumstances?: string[];
  prompt_suggestions?: string[];
  subtle_details?: string[];
  weather_name?: string;
  weather_description?: string;
  location_name?: string;
  present_people?: string[];
  environmental_objects?: string[];
  immediate_pressures?: string[];
}

export interface LastStepResultDTO {
  success: boolean;
  headline?: string;
  narrative: string;
  causality_note: string;
  days_advanced: number;
}

interface SceneRendererProps {
  scene: TodaySceneDTO;
  lastStepResult: LastStepResultDTO | null;
}

export const SceneRenderer: React.FC<SceneRendererProps> = ({
  scene,
  lastStepResult,
}) => {
  return (
    <div className="space-y-6">
      {/* Chapter Location & Time Header */}
      <div className="border-b border-slate-800/80 pb-3 flex items-baseline justify-between">
        <div>
          <span className="text-[11px] font-mono uppercase tracking-widest text-amber-400">
            {scene.life_stage} · Age {scene.age}
          </span>
          <h2 className="text-2xl lg:text-3xl font-serif font-bold text-slate-100 tracking-tight mt-0.5">
            {scene.date_formatted} · {scene.location_formatted}
          </h2>
        </div>
      </div>

      {/* Previous Consequence Transition */}
      {lastStepResult && (
        <div className="relative bg-gradient-to-r from-amber-950/25 via-slate-900/40 to-slate-900/20 border-l-2 border-amber-500/80 rounded-r-xl p-4 space-y-2 shadow-md">
          <div className="flex items-center gap-2 text-amber-300 text-xs font-mono uppercase tracking-wider">
            <Sparkles className="w-3.5 h-3.5 text-amber-400" />
            <span>How recent days unfolded ({lastStepResult.days_advanced} days elapsed)</span>
          </div>
          <p className="text-slate-200 font-serif text-base leading-relaxed italic">
            "{lastStepResult.narrative}"
          </p>
          <div className="pt-1.5 border-t border-slate-800/40 text-xs text-slate-400 font-sans">
            <span className="text-slate-500 font-serif">Milestone: </span>
            {lastStepResult.causality_note}
          </div>
        </div>
      )}

      {/* Living Memoir Prose */}
      <article className="space-y-4 pt-1">
        <p className="text-slate-200 font-serif text-lg lg:text-xl leading-relaxed tracking-normal font-normal">
          {scene.narrative}
        </p>

        {/* Sensory Environmental Observations */}
        {((scene.circumstances && scene.circumstances.length > 0) || (scene.environmental_objects && scene.environmental_objects.length > 0)) && (
          <div className="pt-4 border-t border-slate-800/60 space-y-2">
            <p className="text-xs font-serif italic text-amber-300/80 flex items-center gap-1.5">
              <Eye className="w-3.5 h-3.5 text-amber-400" />
              <span>You notice:</span>
            </p>
            <div className="flex flex-wrap gap-2">
              {(scene.circumstances || scene.environmental_objects || []).map((c, i) => (
                <span
                  key={i}
                  className="text-xs font-sans bg-slate-900/90 text-slate-300 px-3.5 py-1.5 rounded-full border border-slate-800 shadow-sm"
                >
                  • {c}
                </span>
              ))}
            </div>
          </div>
        )}
      </article>
    </div>
  );
};
