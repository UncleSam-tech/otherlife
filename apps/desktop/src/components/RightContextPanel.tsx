import React from 'react';
import { Users, Activity, Battery, Zap, Shield, Sparkles } from 'lucide-react';

export interface LivingStateDTO {
  player_name: string;
  age: number;
  life_stage: string;
  time_formatted: string;
  location_formatted: string;
  cash: number;
  currency_symbol: string;
  household_tier: string;
  energy_level: number;
  stress_level: number;
  fitness: number;
  occupation: string;
  active_processes_count: number;
  surrounding_npcs_count: number;
}

export interface ContextNpcDTO {
  id: string;
  name: string;
  relationship_type: string;
  trust_description: string;
  current_activity: string;
}

export interface ContextProcessDTO {
  id: string;
  title: string;
  progress_percent: number;
  status: string;
}

interface RightContextPanelProps {
  state: LivingStateDTO | null;
  npcs: ContextNpcDTO[];
  processes: ContextProcessDTO[];
}

export const RightContextPanel: React.FC<RightContextPanelProps> = ({
  state,
  npcs,
  processes,
}) => {
  if (!state) return null;

  return (
    <aside className="w-80 bg-slate-900 border-l border-slate-800 p-6 flex flex-col gap-6 overflow-y-auto select-none">
      {/* Human Biological & State Summary */}
      <div className="space-y-4">
        <h3 className="text-xs font-mono uppercase tracking-wider text-slate-400 flex items-center gap-2">
          <Activity className="w-4 h-4 text-emerald-400" />
          <span>Biological & Resource State</span>
        </h3>

        <div className="bg-slate-950/60 border border-slate-800 rounded-xl p-4 space-y-3">
          <div className="flex justify-between items-center text-sm">
            <span className="text-slate-400">Available Funds</span>
            <span className="font-mono font-semibold text-emerald-400">
              {state.currency_symbol}{state.cash.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })}
            </span>
          </div>

          <div className="flex justify-between items-center text-sm">
            <span className="text-slate-400">Household Wealth</span>
            <span className="text-xs bg-slate-800 text-slate-300 px-2 py-0.5 rounded font-mono">
              {state.household_tier}
            </span>
          </div>

          <div className="flex justify-between items-center text-sm">
            <span className="text-slate-400">Occupation / Role</span>
            <span className="text-xs text-slate-300 font-medium truncate max-w-[140px]">
              {state.occupation}
            </span>
          </div>

          {/* Energy & Stress Bars */}
          <div className="space-y-2 pt-2 border-t border-slate-800/80">
            <div>
              <div className="flex justify-between text-xs text-slate-400 mb-1">
                <span className="flex items-center gap-1">
                  <Battery className="w-3.5 h-3.5 text-emerald-400" />
                  Energy
                </span>
                <span className="font-mono">{Math.round(state.energy_level)}%</span>
              </div>
              <div className="w-full bg-slate-800 rounded-full h-1.5 overflow-hidden">
                <div
                  className="bg-emerald-400 h-full rounded-full transition-all duration-300"
                  style={{ width: `${Math.min(100, Math.max(0, state.energy_level))}%` }}
                />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-xs text-slate-400 mb-1">
                <span className="flex items-center gap-1">
                  <Zap className="w-3.5 h-3.5 text-amber-400" />
                  Stress
                </span>
                <span className="font-mono">{Math.round(state.stress_level)}%</span>
              </div>
              <div className="w-full bg-slate-800 rounded-full h-1.5 overflow-hidden">
                <div
                  className="bg-amber-400 h-full rounded-full transition-all duration-300"
                  style={{ width: `${Math.min(100, Math.max(0, state.stress_level))}%` }}
                />
              </div>
            </div>

            <div>
              <div className="flex justify-between text-xs text-slate-400 mb-1">
                <span className="flex items-center gap-1">
                  <Shield className="w-3.5 h-3.5 text-blue-400" />
                  Fitness
                </span>
                <span className="font-mono">{Math.round(state.fitness)}%</span>
              </div>
              <div className="w-full bg-slate-800 rounded-full h-1.5 overflow-hidden">
                <div
                  className="bg-blue-400 h-full rounded-full transition-all duration-300"
                  style={{ width: `${Math.min(100, Math.max(0, state.fitness))}%` }}
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Surrounding People / Autonomous NPCs */}
      <div className="space-y-3">
        <h3 className="text-xs font-mono uppercase tracking-wider text-slate-400 flex items-center gap-2">
          <Users className="w-4 h-4 text-emerald-400" />
          <span>People Around You ({npcs.length})</span>
        </h3>

        <div className="space-y-2.5">
          {npcs.map((npc) => (
            <div
              key={npc.id}
              className="bg-slate-950/60 border border-slate-800 rounded-xl p-3.5 space-y-1.5 hover:border-slate-700 transition-colors"
            >
              <div className="flex justify-between items-center">
                <h4 className="font-medium text-slate-200 text-sm">{npc.name}</h4>
                <span className="text-xs font-mono bg-slate-800 text-emerald-400 px-2 py-0.5 rounded">
                  {npc.relationship_type}
                </span>
              </div>
              <p className="text-xs text-slate-400">
                <span className="text-slate-500">Activity: </span>
                {npc.current_activity}
              </p>
              <p className="text-xs text-slate-500 font-mono">
                Trust: {npc.trust_description}
              </p>
            </div>
          ))}
        </div>
      </div>

      {/* Active Matters & Processes */}
      <div className="space-y-3">
        <h3 className="text-xs font-mono uppercase tracking-wider text-slate-400 flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-emerald-400" />
          <span>Active Matters & Processes ({processes.length})</span>
        </h3>

        {processes.length === 0 ? (
          <div className="bg-slate-950/40 border border-slate-800/80 rounded-xl p-4 text-center">
            <p className="text-xs text-slate-500 font-mono">No active multi-step applications</p>
          </div>
        ) : (
          <div className="space-y-2">
            {processes.map((proc) => (
              <div
                key={proc.id}
                className="bg-slate-950/60 border border-slate-800 rounded-xl p-3.5 space-y-2"
              >
                <div className="flex justify-between items-center">
                  <h4 className="text-xs font-medium text-slate-200">{proc.title}</h4>
                  <span className="text-xs font-mono text-emerald-400">{proc.status}</span>
                </div>
                <div className="w-full bg-slate-800 rounded-full h-1 overflow-hidden">
                  <div
                    className="bg-emerald-500 h-full rounded-full"
                    style={{ width: `${proc.progress_percent}%` }}
                  />
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </aside>
  );
};
