import React from 'react';
import { Users, Heart, Home, Clock } from 'lucide-react';

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

  // Convert internal energy/stress levels into human sensory descriptions
  const getVitalityProse = () => {
    if (state.energy_level > 70 && state.stress_level < 30) {
      return 'You wake feeling well-rested and energized, eager to explore the day.';
    } else if (state.stress_level > 50) {
      return 'You feel the weight of recent demands and mental fatigue.';
    } else {
      return 'You move through the day with steady, quiet focus.';
    }
  };

  // Convert household tier into human living reality description
  const getHouseholdProse = () => {
    const tier = state.household_tier.toUpperCase();
    if (tier.includes('WEALTHY') || tier.includes('UPPER')) {
      return 'Your family lives in an expansive, comfortable home with abundant books, electronics, and deep resources.';
    } else if (tier.includes('POVERTY') || tier.includes('WORKING')) {
      return 'Your household works hard every day. Resources are carefully managed, and every educational opportunity is treasured.';
    } else {
      return 'Your family lives comfortably. Books line the shelves, a computer sits in the study, and resources are set aside for your education.';
    }
  };

  // Map NPC relationship types to natural human titles
  const formatHumanBond = (role: string, name: string) => {
    const roleLower = role.toLowerCase();
    if (roleLower.includes('parent') || roleLower.includes('mother')) {
      return name.toLowerCase().includes('sarah') || name.toLowerCase().includes('fiona') || name.toLowerCase().includes('elena')
        ? 'Your Mother'
        : 'Your Father';
    }
    if (roleLower.includes('teacher')) return 'School Teacher & Mentor';
    if (roleLower.includes('coach')) return 'Sports Coach & Scout';
    if (roleLower.includes('friend') || roleLower.includes('classmate')) return 'Childhood Friend & Classmate';
    return role;
  };

  // Provide evocative personality summaries
  const getNpcPersonalityExcerpt = (name: string) => {
    const n = name.toLowerCase();
    if (n.includes('sarah') || n.includes('fiona') || n.includes('elena')) {
      return 'Patient, nurturing, and attentive to family wellbeing.';
    }
    if (n.includes('david') || n.includes('callum') || n.includes('marcus')) {
      return 'Disciplined, principled, and holds high standards.';
    }
    if (n.includes('adewale') || n.includes('macleod') || n.includes('hayes')) {
      return 'Inspirational mentor who encourages intellectual rigor.';
    }
    if (n.includes('ibrahim') || n.includes('gordon') || n.includes('miller')) {
      return 'Demands physical conditioning, stamina, and work rate.';
    }
    return 'Lively companion sharing daily adventures.';
  };

  return (
    <aside className="w-80 lg:w-96 bg-[#0e1118] border-l border-slate-800/80 p-6 flex flex-col gap-6 overflow-y-auto select-none font-sans">
      {/* 1. Personal State & Human Reality */}
      <div className="space-y-4">
        <h3 className="text-xs font-serif uppercase tracking-widest text-amber-400/80 flex items-center gap-2">
          <Heart className="w-3.5 h-3.5 text-amber-400" />
          <span>Personal Reality</span>
        </h3>

        <div className="bg-slate-900/70 border border-slate-800/80 rounded-2xl p-5 space-y-3.5 shadow-sm">
          <div>
            <p className="text-xs font-mono uppercase tracking-wider text-slate-500 mb-1">Vitality & Mind</p>
            <p className="text-sm text-slate-200 font-serif leading-relaxed italic">
              "{getVitalityProse()}"
            </p>
          </div>

          <div className="pt-3 border-t border-slate-800/60">
            <p className="text-xs font-mono uppercase tracking-wider text-slate-500 mb-1 flex items-center gap-1.5">
              <Home className="w-3.5 h-3.5 text-slate-400" />
              <span>Household Reality</span>
            </p>
            <p className="text-xs text-slate-300 font-sans leading-relaxed">
              {getHouseholdProse()}
            </p>
          </div>

          <div className="pt-3 border-t border-slate-800/60 flex justify-between items-center text-xs">
            <span className="text-slate-400">Personal Savings</span>
            <span className="font-serif font-bold text-amber-300">
              {state.currency_symbol}{state.cash.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })}
            </span>
          </div>
        </div>
      </div>

      {/* 2. People Around You (Living NPCs) */}
      <div className="space-y-4">
        <h3 className="text-xs font-serif uppercase tracking-widest text-amber-400/80 flex items-center gap-2">
          <Users className="w-3.5 h-3.5 text-amber-400" />
          <span>People Around You</span>
        </h3>

        <div className="space-y-3">
          {npcs.map((npc) => (
            <div
              key={npc.id}
              className="bg-slate-900/60 hover:bg-slate-900 border border-slate-800/80 hover:border-slate-700/80 rounded-xl p-4 space-y-2 transition-colors duration-200 shadow-sm"
            >
              <div className="flex justify-between items-baseline">
                <h4 className="font-serif font-semibold text-slate-100 text-sm">{npc.name}</h4>
                <span className="text-xs font-serif italic text-amber-300/80">
                  {formatHumanBond(npc.relationship_type, npc.name)}
                </span>
              </div>

              <p className="text-xs text-slate-300 leading-relaxed font-sans">
                <span className="text-slate-500">Activity: </span>
                {npc.current_activity}
              </p>

              <div className="pt-2 border-t border-slate-800/50 text-xs text-slate-400 font-serif italic">
                {getNpcPersonalityExcerpt(npc.name)}
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* 3. Active Life Undertakings */}
      {processes.length > 0 && (
        <div className="space-y-3 pt-2 border-t border-slate-800/60">
          <h3 className="text-xs font-serif uppercase tracking-widest text-amber-400/80 flex items-center gap-2">
            <Clock className="w-3.5 h-3.5 text-amber-400" />
            <span>Active Undertakings</span>
          </h3>

          <div className="space-y-2">
            {processes.map((proc) => (
              <div
                key={proc.id}
                className="bg-slate-900/50 border border-slate-800/70 rounded-xl p-3.5 space-y-1.5"
              >
                <div className="flex items-center justify-between text-xs">
                  <span className="font-serif font-medium text-slate-200">{proc.title}</span>
                  <span className="text-xs font-mono text-amber-400/90">{Math.round(proc.progress_percent)}%</span>
                </div>
                <div className="w-full bg-slate-800 rounded-full h-1 overflow-hidden">
                  <div
                    className="bg-amber-400 h-full rounded-full transition-all duration-300"
                    style={{ width: `${Math.min(100, Math.max(5, proc.progress_percent))}%` }}
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </aside>
  );
};
