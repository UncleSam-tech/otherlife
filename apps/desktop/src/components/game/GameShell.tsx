import React from 'react';
import { LifeHeader } from './LifeHeader';
import { LifeNavigation, NavLens } from '../navigation/LifeNavigation';
import { WorldStage } from '../world/WorldStage';
import { ContextPanel, LivingStateDTO } from '../context/ContextPanel';
import { ActivityDrawer } from '../interaction/ActivityDrawer';
import { RelationshipPanel } from '../characters/RelationshipPanel';
import { MemoryTimeline } from '../context/MemoryTimeline';
import { TodaySceneDTO, LastStepResultDTO } from '../world/SceneRenderer';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { ContextProcessDTO } from '../context/ProcessTracker';
import { Feather, MapPin, Mail, Globe, Home, BookOpen, Activity, Building, Briefcase } from 'lucide-react';

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
  const playerAge = livingState?.age || 0;

  const renderPlacesByAge = () => {
    if (playerAge < 4) {
      // Infancy: Home & Nursery
      return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-amber-400">
              <Home className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">Family Home & Nursery</h3>
            </div>
            <p className="text-[10px] text-amber-300/80 font-mono">EARLY CHILDHOOD LIVING</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              Your safe, sunlit nursery and family living room where your parents care for you and morning tea brews.
            </p>
          </div>

          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-emerald-400">
              <Activity className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">Neighborhood Clinic & Pediatric Center</h3>
            </div>
            <p className="text-[10px] text-emerald-300/80 font-mono">HEALTH & VACCINATIONS</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              The local healthcare center where infant growth checks and routine vaccinations are administered.
            </p>
          </div>
        </div>
      );
    } else if (playerAge < 13) {
      // Childhood: Primary School & Courtyard
      return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-blue-400">
              <BookOpen className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">District Primary School</h3>
            </div>
            <p className="text-[10px] text-blue-300/80 font-mono">PRIMARY EDUCATION</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              Classrooms with green chalkboards where foundational arithmetic, reading, and moral discipline are taught.
            </p>
          </div>

          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-orange-400">
              <Activity className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">Neighborhood Sports Courtyard</h3>
            </div>
            <p className="text-[10px] text-orange-300/80 font-mono">COMMUNITY RECREATION</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              Open grass grounds where children play street football and neighborhood games after school.
            </p>
          </div>
        </div>
      );
    } else if (playerAge < 18) {
      // Adolescence: Secondary Academy & Sports Club
      return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-amber-400">
              <Building className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">Senior Secondary & Exam Academy</h3>
            </div>
            <p className="text-[10px] text-amber-300/80 font-mono">NATIONAL CERTIFICATION ACADEMY</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              Academic halls hosting intensive examination revisions, science laboratories, and library study desks.
            </p>
          </div>

          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-emerald-400">
              <Activity className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">Youth Sports Academy & Scouting Grounds</h3>
            </div>
            <p className="text-[10px] text-emerald-300/80 font-mono">SCOUTING & COMPETITIVE PITCH</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              Floodlit athletic fields where tactical training drills are contested under the eyes of talent scouts.
            </p>
          </div>
        </div>
      );
    } else {
      // Adulthood: Universities, Workplaces & Civic Institutions
      return (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-indigo-400">
              <Building className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">University & Higher Institute</h3>
            </div>
            <p className="text-[10px] text-indigo-300/80 font-mono">HIGHER EDUCATION & RESEARCH</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              Lecture auditoriums and faculties offering degree programs, research papers, and alumni networks.
            </p>
          </div>

          <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-5 space-y-2 shadow-sm">
            <div className="flex items-center gap-2 text-cyan-400">
              <Briefcase className="w-4 h-4" />
              <h3 className="font-serif font-bold text-slate-100 text-base">Commercial Central Business District</h3>
            </div>
            <p className="text-[10px] text-cyan-300/80 font-mono">FINANCIAL & CORPORATE PLAZAS</p>
            <p className="text-xs text-slate-300 font-serif leading-relaxed">
              Corporate headquarters, venture offices, commercial banks, and company registration registries.
            </p>
          </div>
        </div>
      );
    }
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-[#07090e] text-slate-100 overflow-hidden font-sans select-none">
      {/* 1. Life Header */}
      <LifeHeader
        playerName={livingState?.player_name || 'Living Person'}
        age={playerAge}
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
              {/* Structured Activities Menu Drawer (BitLife Style) */}
              <ActivityDrawer
                playerAge={playerAge}
                onSubmitIntent={onSubmitIntent}
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
                  <p className="text-xs font-serif italic text-amber-300/80">The locations and institutions currently present in your world</p>
                </div>
              </div>

              {renderPlacesByAge()}
            </main>
          )}

          {activeLens === 'messages' && (
            <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
              <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
                <Mail className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">Letters & Notices</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Official correspondence, certifications, and personal notes</p>
                </div>
              </div>
              <div className="space-y-4">
                {playerAge < 4 ? (
                  <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-6 space-y-3 shadow-sm">
                    <div className="flex justify-between items-start">
                      <div>
                        <h3 className="font-serif font-bold text-slate-100 text-base">Civic Registry of Births</h3>
                        <p className="text-xs text-amber-300/80 font-mono">OFFICIAL BIRTH RECORD</p>
                      </div>
                      <span className="text-xs bg-amber-500/10 text-amber-300 px-3 py-1 rounded-full border border-amber-500/20 font-serif">
                        Certificate
                      </span>
                    </div>
                    <p className="text-sm text-slate-200 font-serif leading-relaxed italic">
                      "Birth officially registered in the civic registry. Welcome to the living world."
                    </p>
                  </div>
                ) : (
                  <div className="bg-[#121622] border border-[#20273a] rounded-2xl p-6 space-y-3 shadow-sm">
                    <div className="flex justify-between items-start">
                      <div>
                        <h3 className="font-serif font-bold text-slate-100 text-base">National Examination Registry</h3>
                        <p className="text-xs text-amber-300/80 font-mono">OFFICIAL ADMISSIONS ENTRY</p>
                      </div>
                      <span className="text-xs bg-amber-500/10 text-amber-300 px-3 py-1 rounded-full border border-amber-500/20 font-serif">
                        Official
                      </span>
                    </div>
                    <p className="text-sm text-slate-200 font-serif leading-relaxed italic">
                      "Candidate registration portal is open for students preparing for higher certifications."
                    </p>
                  </div>
                )}
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
