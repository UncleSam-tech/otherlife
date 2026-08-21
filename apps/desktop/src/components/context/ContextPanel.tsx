import React from 'react';
import { Heart, Home, Users } from 'lucide-react';
import { NPCDisplay, ContextNpcDTO } from '../characters/NPCDisplay';
import { ProcessTracker, ContextProcessDTO } from './ProcessTracker';

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

interface ContextPanelProps {
  state: LivingStateDTO | null;
  npcs: ContextNpcDTO[];
  processes: ContextProcessDTO[];
  onSelectNpc?: (npc: ContextNpcDTO) => void;
}

export const ContextPanel: React.FC<ContextPanelProps> = ({
  state,
  npcs,
  processes,
  onSelectNpc,
}) => {
  if (!state) return null;

  const getVitalityProse = () => {
    if (state.energy_level > 70 && state.stress_level < 30) {
      return 'You wake feeling well-rested and energized, eager to explore the day.';
    } else if (state.stress_level > 50) {
      return 'You feel the weight of recent demands and mental fatigue.';
    } else {
      return 'You move through the day with steady, quiet focus.';
    }
  };

  const getHouseholdProse = () => {
    const tier = state.household_tier.toUpperCase();
    if (tier.includes('WEALTHY') || tier.includes('UPPER')) {
      return 'Your family lives in an expansive home with abundant books, electronics, and deep resources.';
    } else if (tier.includes('POVERTY') || tier.includes('WORKING')) {
      return 'Your household works hard every day. Resources are carefully budgeted, and every educational opportunity is treasured.';
    } else {
      return 'Your family lives comfortably. Books line the shelves, a computer sits in the study, and resources are set aside for your education.';
    }
  };

  return (
    <aside className="w-80 lg:w-88 bg-[#0a0c12] border-l border-[#1c2130] p-5 flex flex-col gap-5 overflow-y-auto select-none font-sans z-20">
      {/* 1. Personal State & Human Reality */}
      <div className="space-y-3">
        <h3 className="text-xs font-serif uppercase tracking-widest text-amber-400/80 flex items-center gap-2">
          <Heart className="w-3.5 h-3.5 text-amber-400" />
          <span>Personal Reality</span>
        </h3>

        <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-4 space-y-3 shadow-sm">
          <div>
            <p className="text-[10px] font-mono uppercase tracking-wider text-slate-500 mb-1">Vitality & Mind</p>
            <p className="text-xs text-slate-200 font-serif leading-relaxed italic">
              "{getVitalityProse()}"
            </p>
          </div>

          <div className="pt-2.5 border-t border-[#1c2234]">
            <p className="text-[10px] font-mono uppercase tracking-wider text-slate-500 mb-1 flex items-center gap-1.5">
              <Home className="w-3.5 h-3.5 text-slate-400" />
              <span>Household Reality</span>
            </p>
            <p className="text-xs text-slate-300 font-sans leading-relaxed">
              {getHouseholdProse()}
            </p>
          </div>

          <div className="pt-2.5 border-t border-[#1c2234] flex justify-between items-center text-xs">
            <span className="text-slate-400">Personal Savings</span>
            <span className="font-serif font-bold text-amber-300">
              {state.currency_symbol}{state.cash.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })}
            </span>
          </div>
        </div>
      </div>

      {/* 2. People Around You (Living NPCs) */}
      <div className="space-y-3">
        <h3 className="text-xs font-serif uppercase tracking-widest text-amber-400/80 flex items-center gap-2">
          <Users className="w-3.5 h-3.5 text-amber-400" />
          <span>People Around You</span>
        </h3>

        <div className="space-y-2.5">
          {npcs.map((npc) => (
            <NPCDisplay key={npc.id} npc={npc} onSelectNpc={onSelectNpc} />
          ))}
        </div>
      </div>

      {/* 3. Active Life Undertakings */}
      <ProcessTracker processes={processes} />
    </aside>
  );
};
