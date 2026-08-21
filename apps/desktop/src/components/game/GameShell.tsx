import React from 'react';
import { LifeHeader } from './LifeHeader';
import { LifeNavigation, NavLens } from '../navigation/LifeNavigation';
import { WorldStage } from '../world/WorldStage';
import { ContextPanel, LivingStateDTO } from '../context/ContextPanel';
import { NaturalIntentBar } from '../interaction/NaturalIntentBar';
import { RelationshipPanel } from '../characters/RelationshipPanel';
import { MemoryTimeline } from '../context/MemoryTimeline';
import { TodaySceneDTO, LastStepResultDTO } from '../world/SceneRenderer';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { ContextProcessDTO } from '../context/ProcessTracker';
import { Feather, MapPin, Mail, Globe } from 'lucide-react';

interface GameShellProps {
  livingState: LivingStateDTO | null;
  todayScene: TodaySceneDTO | null;
  lastStepResult: LastStepResultDTO | null;
  npcs: ContextNpcDTO[];
  processes: ContextProcessDTO[];
  biographyText: string;
  activeLens: NavLens;
  onSelectLens: (lens: NavLens) => void;
  onSubmitIntent: (intentText: string) => void;
  isLoading: boolean;
  onReturnToMainMenu: () => void;
  devMode: boolean;
  onToggleDevMode: () => void;
}

export const GameShell: React.FC<GameShellProps> = ({
  livingState,
  todayScene,
  lastStepResult,
  npcs,
  processes,
  biographyText,
  activeLens,
  onSelectLens,
  onSubmitIntent,
  isLoading,
  onReturnToMainMenu,
  devMode,
  onToggleDevMode,
}) => {
  return (
    <div className="flex flex-col h-screen w-screen bg-[#07090e] text-slate-100 overflow-hidden font-sans select-none">
      {/* 1. Life Header */}
      <LifeHeader
        playerName={livingState?.player_name || 'Living Person'}
        age={livingState?.age || 0}
        lifeStage={livingState?.life_stage || 'Infancy'}
        timeFormatted={livingState?.time_formatted || ''}
        locationFormatted={livingState?.location_formatted || 'Living World'}
        weatherName="Harmattan Haze"
        currencySymbol={livingState?.currency_symbol || '₦'}
        cash={livingState?.cash || 0}
        onReturnToMainMenu={onReturnToMainMenu}
        devMode={devMode}
        onToggleDevMode={onToggleDevMode}
      />

      {/* 2. Middle Body: 3-Column Living World Layout */}
      <div className="flex flex-1 overflow-hidden relative">
        {/* Left Column: Permanent Life Lenses */}
        <LifeNavigation
          activeLens={activeLens}
          onSelectLens={onSelectLens}
        />

        {/* Center Stage: Active Lens Content */}
        <div className="flex-1 flex flex-col overflow-hidden relative bg-[#0a0c12]">
          {activeLens === 'life' && (
            <div className="flex-1 flex flex-col overflow-hidden">
              <WorldStage
                scene={todayScene}
                lastStepResult={lastStepResult}
                weatherName="Harmattan Haze"
              />
              {/* Natural Intention Bar */}
              <NaturalIntentBar
                onSubmitIntent={onSubmitIntent}
                suggestions={todayScene?.prompt_suggestions || []}
                isLoading={isLoading}
              />
            </div>
          )}

          {activeLens === 'people' && (
            <RelationshipPanel npcs={npcs} />
          )}

          {activeLens === 'journal' && (
            <MemoryTimeline
              timeFormatted={livingState?.time_formatted || ''}
              headline={todayScene?.headline}
              narrative={todayScene?.narrative}
            />
          )}

          {activeLens === 'biography' && (
            <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
              <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-8 space-y-6 shadow-md">
                <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
                  <Feather className="w-6 h-6 text-amber-400" />
                  <div>
                    <h2 className="text-2xl font-serif font-bold text-slate-100">My Story</h2>
                    <p className="text-xs font-serif italic text-amber-300/80">Autobiographical reflections on the path traveled</p>
                  </div>
                </div>
                <div className="prose prose-invert max-w-none text-slate-200 font-serif text-lg leading-relaxed whitespace-pre-wrap">
                  {biographyText || 'The first chapters of life are still unfolding...'}
                </div>
              </div>
            </main>
          )}

          {activeLens === 'places' && (
            <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
              <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
                <MapPin className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">Places & Horizon</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">The classrooms, grounds, streets, and institutions of your world</p>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
                  <h3 className="font-serif font-bold text-slate-100 text-base">District Primary & Secondary Academy</h3>
                  <p className="text-xs text-amber-300/80 font-mono">EDUCATIONAL FOUNDATION</p>
                  <p className="text-xs text-slate-300 font-serif leading-relaxed">
                    Classrooms and school courtyards where foundational arithmetic, literacy, and character discipline take root.
                  </p>
                </div>
                <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
                  <h3 className="font-serif font-bold text-slate-100 text-base">Community Sports & Athletics Ground</h3>
                  <p className="text-xs text-amber-300/80 font-mono">TRAINING & SCOUTING PITCH</p>
                  <p className="text-xs text-slate-300 font-serif leading-relaxed">
                    Youth athletic pitches hosting training drills, scrimmage matches, and grassroots talent scouts.
                  </p>
                </div>
              </div>
            </main>
          )}

          {activeLens === 'messages' && (
            <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
              <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
                <Mail className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">Letters & Inward Mail</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Official correspondence, examination slips, and notes</p>
                </div>
              </div>
              <div className="space-y-4">
                <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-6 space-y-3 shadow-sm">
                  <div className="flex justify-between items-start">
                    <div>
                      <h3 className="font-serif font-bold text-slate-100 text-base">National Examination Registry</h3>
                      <p className="text-xs text-amber-300/80 font-mono">OFFICIAL REGISTRATION NOTICE</p>
                    </div>
                    <span className="text-xs bg-amber-500/10 text-amber-300 px-3 py-1 rounded-full border border-amber-500/20 font-serif">
                      Official
                    </span>
                  </div>
                  <p className="text-sm text-slate-200 font-serif leading-relaxed italic">
                    "Candidate enrollment portal is open for qualifying students preparing for higher education admissions certification."
                  </p>
                </div>
              </div>
            </main>
          )}

          {activeLens === 'world' && (
            <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
              <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
                <Globe className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">Surrounding World & Era</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Climate, regional setting, and historical background</p>
                </div>
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
                  <p className="text-xs font-mono uppercase tracking-wider text-slate-500">Geographic Setting</p>
                  <p className="text-sm text-slate-200 font-serif leading-relaxed">
                    Located in {livingState?.location_formatted}. Daily rhythms balance family life, school terms, and community activities.
                  </p>
                </div>
                <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
                  <p className="text-xs font-mono uppercase tracking-wider text-slate-500">Living Environment</p>
                  <p className="text-sm text-slate-200 font-serif leading-relaxed">
                    A vibrant urban environment where discipline, curiosity, and family support shape long-term opportunities.
                  </p>
                </div>
              </div>
            </main>
          )}
        </div>

        {/* Right Column: Context Panel */}
        <ContextPanel
          state={livingState}
          npcs={npcs}
          processes={processes}
        />
      </div>
    </div>
  );
};
