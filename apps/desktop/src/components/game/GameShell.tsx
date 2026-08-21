import React, { useState } from 'react';
import { LifeHeader } from './LifeHeader';
import { LifeNavigation, NavLens } from '../navigation/LifeNavigation';
import { WorldStage } from '../world/WorldStage';
import { ContextPanel, LivingStateDTO } from '../context/ContextPanel';
import { ActivityDrawer } from '../interaction/ActivityDrawer';
import { MemoryTimeline } from '../context/MemoryTimeline';
import { TodaySceneDTO, LastStepResultDTO } from '../world/SceneRenderer';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { ContextProcessDTO } from '../context/ProcessTracker';
import { PersonInteractionModal } from '../characters/PersonInteractionModal';
import { PlaceInteractionModal, PlaceLocationDTO } from '../world/PlaceInteractionModal';
import { Feather, Mail, Globe, Send } from 'lucide-react';

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
  const [selectedNpc, setSelectedNpc] = useState<ContextNpcDTO | null>(null);
  const [selectedPlace, setSelectedPlace] = useState<PlaceLocationDTO | null>(null);

  const getPlacesForAge = (): PlaceLocationDTO[] => {
    if (playerAge < 4) {
      return [
        {
          id: 'place_home',
          name: 'Family Home & Nursery',
          category: 'Home Living',
          desc: 'Your peaceful nursery and family living room with your crib, toys, and parents.',
          actions: [
            { id: 'home_rest', title: 'Rest in Crib', desc: 'Sleep and restore your energy peacefully.', intent: 'I rest in my crib and take a peaceful nap.' },
            { id: 'home_play', title: 'Play on the Living Room Rug', desc: 'Explore wooden blocks and toys.', intent: 'I play with toys on the living room rug near my family.' },
          ],
        },
        {
          id: 'place_clinic',
          name: 'Neighborhood Clinic & Pediatric Center',
          category: 'Health & Care',
          desc: 'Local clinic for child growth monitoring and vaccinations.',
          actions: [
            { id: 'clinic_check', title: 'Get Pediatric Health Checkup', desc: 'Visit with mother for health measurements.', intent: 'I visit the neighborhood clinic with my mother for routine health checkups and vaccination.' },
          ],
        },
      ];
    } else if (playerAge < 13) {
      return [
        {
          id: 'place_school',
          name: 'District Primary School',
          category: 'Primary Education',
          desc: 'Classrooms with green chalkboards where foundational arithmetic, reading, and discipline are taught.',
          actions: [
            { id: 'school_study', title: 'Attend Arithmetic & Reading Classes', desc: 'Work through problem sets with the class.', intent: 'I spend the afternoon doing arithmetic exercises and reading my schoolbooks carefully.' },
            { id: 'school_club', title: 'Participate in Science & Debate Club', desc: 'Engage with fellow curious students.', intent: 'I attend the school Science and Debate Club to learn with fellow curious students.' },
          ],
        },
        {
          id: 'place_courtyard',
          name: 'Neighborhood Sports Courtyard',
          category: 'Athletics & Recreation',
          desc: 'Open grass grounds where children play street football and athletic games.',
          actions: [
            { id: 'court_football', title: 'Join Football Drills', desc: 'Practice passing, ball control, and scrimmages.', intent: 'I join the youth football training session on the community field and practice ball control.' },
          ],
        },
      ];
    } else if (playerAge < 18) {
      return [
        {
          id: 'place_secondary',
          name: 'Senior Secondary & Exam Academy',
          category: 'Secondary Education',
          desc: 'Academic halls hosting national examination revisions (WAEC / JAMB / GCSE) and science labs.',
          actions: [
            { id: 'exam_revision', title: 'Study for National Certificate Examinations', desc: 'Revise chemistry, physics, and advanced mathematics question papers.', intent: 'I dedicate intensive evening study sessions to past examination papers in preparation for national certification.' },
            { id: 'library_session', title: 'Study in the Quiet Central Library', desc: 'Spend uninterrupted hours mastering curriculum topics.', intent: 'I spend Saturday mornings in the central library revising advanced curriculum subjects.' },
          ],
        },
        {
          id: 'place_sports_academy',
          name: 'Youth Sports Academy & Scouting Grounds',
          category: 'Athletics & Scouting',
          desc: 'Floodlit athletic fields where tactical training drills are contested under talent scouts.',
          actions: [
            { id: 'attend_trials', title: 'Attend Scouting Selection Trials', desc: 'Compete in scrimmages before talent scouts.', intent: 'I lace up my boots and attend competitive youth football trials before academy scouts.' },
          ],
        },
      ];
    } else {
      return [
        {
          id: 'place_university',
          name: 'University & Higher Institute',
          category: 'Higher Education',
          desc: 'Lecture auditoriums and faculties offering degree programs, research papers, and alumni networks.',
          actions: [
            { id: 'uni_classes', title: 'Attend University Lectures & Seminars', desc: 'Advance toward degree graduation and honors.', intent: 'I attend university lectures and seminars with full academic diligence.' },
          ],
        },
        {
          id: 'place_cbd',
          name: 'Commercial Central Business District',
          category: 'Finance & Enterprise',
          desc: 'Corporate offices, commercial banks, and company registration registries.',
          actions: [
            { id: 'register_business', title: 'Register a New Company / LLC', desc: 'Incorporate your business entity with commercial authorities.', intent: 'I formally incorporate a new limited liability company with commercial authorities.' },
            { id: 'apply_jobs', title: 'Apply for Professional Career Listings', desc: 'Submit CV for corporate and engineering roles.', intent: 'I submit formal applications for open professional positions aligned with my qualifications.' },
          ],
        },
      ];
    }
  };

  const currentPlaces = getPlacesForAge();

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
            <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
              <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">People & Bonds</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Click any person to converse, ask for support, or interact</p>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {npcs.map((npc) => (
                  <div
                    key={npc.id}
                    onClick={() => setSelectedNpc(npc)}
                    className="bg-[#121622] hover:bg-[#161c2b] border border-[#20273a] hover:border-amber-500/50 p-5 rounded-2xl cursor-pointer space-y-2.5 transition-all duration-200 shadow-sm group"
                  >
                    <div className="flex justify-between items-baseline">
                      <h4 className="font-serif font-bold text-slate-100 text-base group-hover:text-amber-200">{npc.name}</h4>
                      <span className="text-xs font-serif italic text-amber-300/90">{npc.relationship_type}</span>
                    </div>
                    <p className="text-xs text-slate-300 font-sans">
                      <span className="text-slate-500 font-serif">Currently: </span>
                      {npc.current_activity}
                    </p>
                    <div className="pt-2 border-t border-[#1c2234] text-[11px] text-amber-400/80 font-serif italic flex justify-between items-center">
                      <span>Click to interact</span>
                      <span className="text-slate-500">{npc.trust_description}</span>
                    </div>
                  </div>
                ))}
              </div>
            </main>
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
              <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">Places & Horizon</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Click any location to visit and take direct action</p>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {currentPlaces.map((pl) => (
                  <div
                    key={pl.id}
                    onClick={() => setSelectedPlace(pl)}
                    className="bg-[#121622] hover:bg-[#161c2b] border border-[#20273a] hover:border-amber-500/50 p-5 rounded-2xl cursor-pointer space-y-2.5 transition-all duration-200 shadow-sm group"
                  >
                    <div className="flex items-center justify-between">
                      <h3 className="font-serif font-bold text-slate-100 text-base group-hover:text-amber-200">{pl.name}</h3>
                      <Send className="w-3.5 h-3.5 text-slate-600 group-hover:text-amber-400 group-hover:translate-x-0.5 transition-all" />
                    </div>
                    <p className="text-[10px] text-amber-300/80 font-mono uppercase">{pl.category}</p>
                    <p className="text-xs text-slate-300 font-serif leading-relaxed">
                      {pl.desc}
                    </p>
                    <div className="pt-2 border-t border-[#1c2234] text-[11px] text-amber-400/80 font-serif italic">
                      Click to visit ({pl.actions.length} actions available)
                    </div>
                  </div>
                ))}
              </div>
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
          onSelectNpc={(npc) => setSelectedNpc(npc)}
        />
      </div>

      {/* Person Interaction Modal */}
      {selectedNpc && (
        <PersonInteractionModal
          npc={selectedNpc}
          playerAge={playerAge}
          onClose={() => setSelectedNpc(null)}
          onExecuteAction={onSubmitIntent}
          isLoading={isLoading}
        />
      )}

      {/* Place Interaction Modal */}
      {selectedPlace && (
        <PlaceInteractionModal
          place={selectedPlace}
          onClose={() => setSelectedPlace(null)}
          onExecuteAction={onSubmitIntent}
          isLoading={isLoading}
        />
      )}
    </div>
  );
};
