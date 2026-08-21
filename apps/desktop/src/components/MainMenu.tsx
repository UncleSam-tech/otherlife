import React, { useState } from 'react';
import { Play, Sparkles, FolderOpen, Dices } from 'lucide-react';
import { LifeCreator, NewLifeCreatorConfig } from './creation/LifeCreator';

export interface SaveMetadata {
  id: string;
  filename: string;
  player_name: string;
  age: number;
  location: string;
  timestamp: string;
}

interface MainMenuProps {
  saves: SaveMetadata[];
  onStartNewLife: (config?: NewLifeCreatorConfig) => void;
  onContinueRecentSave: () => void;
  onLoadSave: (filename: string) => void;
  onDeleteSave: (filename: string) => void;
  onOpenSettings: () => void;
}

export const MainMenu: React.FC<MainMenuProps> = ({
  saves,
  onStartNewLife,
  onContinueRecentSave,
  onLoadSave,
  onDeleteSave: _onDeleteSave,
  onOpenSettings: _onOpenSettings,
}) => {
  const [showCreator, setShowCreator] = useState(false);
  const [showLoadModal, setShowLoadModal] = useState(false);
  const hasSaves = saves && saves.length > 0;
  const recentSave = hasSaves ? saves[0] : null;

  if (showCreator) {
    return (
      <div className="w-screen h-screen bg-[#07090e] flex items-center justify-center p-6 select-none">
        <LifeCreator
          onBeginLife={(config) => onStartNewLife(config)}
          onCancel={() => setShowCreator(false)}
        />
      </div>
    );
  }

  return (
    <div className="w-screen h-screen bg-[#07090e] text-slate-100 flex flex-col items-center justify-center relative overflow-hidden select-none font-sans">
      {/* Warm Ambient Radial Glow */}
      <div className="absolute top-1/3 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[700px] h-[500px] bg-gradient-to-b from-amber-500/10 via-amber-950/5 to-transparent rounded-full blur-3xl pointer-events-none" />

      <div className="flex flex-col items-center text-center z-10 max-w-xl w-11/12 space-y-8">
        {/* Brand & Subtitle */}
        <div className="space-y-3">
          <div className="inline-flex items-center gap-2 px-3.5 py-1 rounded-full bg-amber-500/10 border border-amber-500/30 text-amber-300 text-xs font-mono tracking-widest uppercase">
            <Sparkles className="w-3.5 h-3.5 text-amber-400" />
            <span>Living Human Simulator</span>
          </div>

          <h1 className="text-5xl md:text-6xl font-serif font-black tracking-tight text-slate-100">
            OTHERLIFE
          </h1>

          <p className="text-sm md:text-base font-serif italic text-slate-400 max-w-md mx-auto leading-relaxed">
            "Experience an entire human life through decisions, processes, relationships, opportunities, failures, and consequences."
          </p>
        </div>

        {/* Primary Action Buttons */}
        <div className="flex flex-col w-full max-w-sm gap-3 pt-2">
          {/* Continue Recent Save (if any) */}
          {recentSave && (
            <button
              type="button"
              onClick={onContinueRecentSave}
              className="w-full bg-[#121622] hover:bg-[#161c2b] border border-[#20273a] hover:border-amber-500/40 rounded-2xl p-4 flex items-center justify-between text-left transition-all duration-200 shadow-md group"
            >
              <div>
                <div className="text-xs font-mono uppercase tracking-wider text-amber-400">Continue Life</div>
                <div className="font-serif font-bold text-slate-100 text-sm mt-0.5">{recentSave.player_name}</div>
                <div className="text-xs text-slate-400 mt-0.5">Age {recentSave.age} · {recentSave.location}</div>
              </div>
              <Play className="w-5 h-5 text-amber-400 group-hover:translate-x-1 transition-transform" />
            </button>
          )}

          {/* Begin New Life (opens LifeCreator prologue) */}
          <button
            type="button"
            onClick={() => setShowCreator(true)}
            className="w-full bg-amber-500 hover:bg-amber-400 text-slate-950 font-serif font-bold py-4 px-6 rounded-2xl flex items-center justify-center gap-3 shadow-xl shadow-amber-500/20 transition-all duration-200 hover:scale-[1.02]"
          >
            <Sparkles className="w-4 h-4" />
            <span>Begin a New Life</span>
          </button>

          {/* Instant Fate Mode */}
          <button
            type="button"
            onClick={() => {
              const countries = ['country:real:nigeria', 'country:real:united_kingdom', 'country:real:united_states'];
              const cities = {
                'country:real:nigeria': ['city:real:lagos', 'city:real:abuja', 'city:real:ibadan', 'city:real:kano', 'city:real:enugu'],
                'country:real:united_kingdom': ['city:real:london', 'city:real:glasgow', 'city:real:manchester', 'city:real:edinburgh'],
                'country:real:united_states': ['city:real:new_york', 'city:real:san_francisco', 'city:real:los_angeles', 'city:real:chicago'],
              };
              const c = countries[Math.floor(Math.random() * countries.length)];
              const cityList = cities[c as keyof typeof cities];
              const city = cityList[Math.floor(Math.random() * cityList.length)];
              const tiers = ['WORKING_CLASS', 'MIDDLE', 'UPPER_MIDDLE'];
              const sexes = ['Male', 'Female'];
              const chosenSex = sexes[Math.floor(Math.random() * sexes.length)];
              
              onStartNewLife({
                creation_mode: 'CUSTOM',
                starting_year: 2005,
                country_id: c,
                location_id: city,
                starting_age: 0,
                birth_year: 2005,
                birth_month: 6,
                birth_day: 14,
                first_name: chosenSex === 'Female' ? 'Elena' : 'Tunde',
                last_name: 'Sterling',
                sex: chosenSex,
                household_income_tier: tiers[Math.floor(Math.random() * tiers.length)],
                traits: {},
                skills: {},
                interests: ['curiosity'],
                goals: ['discovery'],
              });
            }}
            className="w-full bg-[#121622] hover:bg-[#161c2b] border border-[#20273a] text-slate-300 hover:text-amber-200 py-3 rounded-2xl flex items-center justify-center gap-2 text-xs font-serif transition-all"
          >
            <Dices className="w-4 h-4 text-amber-400" />
            <span>Instant Fate Mode (Random Life)</span>
          </button>

          {/* Saved Timelines Button */}
          {hasSaves && (
            <button
              type="button"
              onClick={() => setShowLoadModal(true)}
              className="w-full bg-transparent hover:bg-slate-900/50 text-slate-400 hover:text-slate-200 py-2.5 rounded-xl flex items-center justify-center gap-2 text-xs font-serif transition-colors"
            >
              <FolderOpen className="w-3.5 h-3.5" />
              <span>Saved Timelines ({saves.length})</span>
            </button>
          )}
        </div>
      </div>

      {/* Load Save Modal */}
      {showLoadModal && (
        <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
          <div className="bg-[#0e1118] border border-[#1c2130] rounded-2xl max-w-md w-full p-6 space-y-4 shadow-2xl">
            <h3 className="font-serif font-bold text-lg text-slate-100">Saved Lives</h3>
            <div className="max-h-60 overflow-y-auto space-y-2">
              {saves.map((s) => (
                <div
                  key={s.id}
                  onClick={() => {
                    onLoadSave(s.filename);
                    setShowLoadModal(false);
                  }}
                  className="bg-[#121622] hover:bg-[#161c2b] border border-[#20273a] p-3 rounded-xl cursor-pointer flex justify-between items-center transition-colors"
                >
                  <div>
                    <div className="font-serif font-semibold text-sm text-slate-100">{s.player_name}</div>
                    <div className="text-xs text-slate-400">Age {s.age} · {s.location}</div>
                  </div>
                  <Play className="w-4 h-4 text-amber-400" />
                </div>
              ))}
            </div>
            <button
              type="button"
              onClick={() => setShowLoadModal(false)}
              className="w-full bg-[#121622] hover:bg-[#161c2b] text-slate-300 py-2 rounded-xl text-xs font-serif"
            >
              Close
            </button>
          </div>
        </div>
      )}
    </div>
  );
};
