import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { LeftNav, NavLens } from './components/LeftNav';
import { CenterLivingStage, TodaySceneDTO, LastStepResultDTO } from './components/CenterLivingStage';
import { RightContextPanel, LivingStateDTO, ContextNpcDTO, ContextProcessDTO } from './components/RightContextPanel';
import { MainMenu, SaveMetadata } from './components/MainMenu';
import { CausalityInspector } from './components/CausalityInspector';
import { BookOpen, Feather, MapPin, Users, Mail, Globe } from 'lucide-react';
import './styles/globals.css';

export type AppMode = 'BOOTING' | 'MAIN_MENU' | 'PLAYING';

async function callTauriCommand<T>(cmd: string, args: Record<string, any> = {}): Promise<T | null> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[Tauri Command]: ${cmd} execution note:`, err);
    return null;
  }
}

export const App: React.FC = () => {
  const [appMode, setAppMode] = useState<AppMode>('BOOTING');
  const [devMode, setDevMode] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [activeLens, setActiveLens] = useState<NavLens>('life');

  const [savesList, setSavesList] = useState<SaveMetadata[]>([]);
  const [livingState, setLivingState] = useState<LivingStateDTO | null>(null);
  const [todayScene, setTodayScene] = useState<TodaySceneDTO | null>(null);
  const [npcs, setNpcs] = useState<ContextNpcDTO[]>([]);
  const [processes, setProcesses] = useState<ContextProcessDTO[]>([]);
  const [lastStepResult, setLastStepResult] = useState<LastStepResultDTO | null>(null);
  const [biographyText, setBiographyText] = useState<string>('');

  const refreshSavesList = async () => {
    const saves = await callTauriCommand<SaveMetadata[]>('list_saves');
    setSavesList(saves || []);
  };

  const refreshBiography = async () => {
    const bio = await callTauriCommand<string>('get_biography');
    if (bio) setBiographyText(bio);
  };

  useEffect(() => {
    const initBoot = async () => {
      await refreshSavesList();
      setAppMode('MAIN_MENU');
    };
    initBoot();
  }, []);

  const handleStartNewLife = async (config?: any) => {
    setIsLoading(true);
    const lifeConfig = config || {
      creation_mode: 'CUSTOM',
      starting_year: 2005,
      country_id: 'country:real:nigeria',
      location_id: 'city:real:abuja',
      starting_age: 0,
      first_name: 'Israel',
      last_name: 'Oyebamiji',
      sex: 'Male',
      household_income_tier: 'MIDDLE',
      traits: {},
      skills: {},
      interests: ['academics'],
      goals: ['excellence'],
    };

    const res = await callTauriCommand<[LivingStateDTO, TodaySceneDTO, ContextNpcDTO[], ContextProcessDTO[]]>(
      'start_new_life',
      { config: lifeConfig, seed: 42 }
    );

    if (res) {
      setLivingState(res[0]);
      setTodayScene(res[1]);
      setNpcs(res[2]);
      setProcesses(res[3]);
      setLastStepResult(null);
      await refreshBiography();
      setAppMode('PLAYING');
      setActiveLens('life');
    }
    setIsLoading(false);
  };

  const handleSubmitIntent = async (intentText: string) => {
    if (!intentText.trim() || isLoading) return;
    setIsLoading(true);

    const res = await callTauriCommand<[LivingStateDTO, LastStepResultDTO, TodaySceneDTO, ContextNpcDTO[], ContextProcessDTO[]]>(
      'submit_living_intent',
      { intentText }
    );

    if (res) {
      setLivingState(res[0]);
      setLastStepResult(res[1]);
      setTodayScene(res[2]);
      setNpcs(res[3]);
      setProcesses(res[4]);
      await refreshBiography();
    }
    setIsLoading(false);
  };

  if (appMode === 'BOOTING') {
    return (
      <div className="flex items-center justify-center h-screen bg-[#0b0d13] text-amber-200 font-serif">
        <p className="italic text-lg">Opening the book of life...</p>
      </div>
    );
  }

  if (appMode === 'MAIN_MENU') {
    return (
      <MainMenu
        onStartNewLife={() => handleStartNewLife()}
        onContinueRecentSave={() => handleStartNewLife()}
        onLoadSave={(filename) => console.log('Load save:', filename)}
        onOpenSettings={() => {}}
        saves={savesList}
        onDeleteSave={async () => { await refreshSavesList(); }}
      />
    );
  }

  return (
    <div className="flex flex-col h-screen w-screen bg-[#0b0d13] text-slate-100 overflow-hidden font-sans">
      <Header
        timeFormatted={livingState?.time_formatted || ''}
        age={livingState?.age || 0}
        cash={livingState?.cash || 0}
        location={livingState?.location_formatted || 'Abuja, Nigeria'}
        playerName={livingState?.player_name || 'Israel Oyebamiji'}
        currencySymbol={livingState?.currency_symbol || '₦'}
        devMode={devMode}
        onToggleDevMode={() => setDevMode(!devMode)}
        onReturnToMainMenu={() => setAppMode('MAIN_MENU')}
      />

      <div className="flex flex-1 overflow-hidden">
        {/* Left Column: Permanent Lenses */}
        <LeftNav activeLens={activeLens} onSelectLens={setActiveLens} />

        {/* Center Column: Active Stage or Lens View */}
        {activeLens === 'life' && (
          <CenterLivingStage
            scene={todayScene}
            lastStepResult={lastStepResult}
            onSubmitIntent={handleSubmitIntent}
            isLoading={isLoading}
          />
        )}

        {activeLens === 'biography' && (
          <main className="flex-1 overflow-y-auto bg-[#0b0d13] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-8 space-y-6 shadow-md">
              <div className="flex items-center gap-3 border-b border-slate-800/60 pb-4">
                <Feather className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">My Story</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Reflections on the path traveled so far</p>
                </div>
              </div>
              <div className="prose prose-invert max-w-none text-slate-200 font-serif text-lg leading-relaxed whitespace-pre-wrap">
                {biographyText || 'The first chapters of life are still being written...'}
              </div>
            </div>
          </main>
        )}

        {activeLens === 'journal' && (
          <main className="flex-1 overflow-y-auto bg-[#0b0d13] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-8 space-y-6 shadow-md">
              <div className="flex items-center gap-3 border-b border-slate-800/60 pb-4">
                <BookOpen className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">Life Chronicle</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Defining memories and turning points</p>
                </div>
              </div>
              <div className="space-y-6">
                <div className="relative pl-6 border-l-2 border-amber-500/80 space-y-2">
                  <div className="absolute -left-[9px] top-1.5 w-4 h-4 rounded-full bg-slate-950 border-2 border-amber-400 flex items-center justify-center">
                    <div className="w-1.5 h-1.5 rounded-full bg-amber-400" />
                  </div>
                  <span className="text-xs font-mono text-amber-400">{livingState?.time_formatted}</span>
                  <h4 className="font-serif font-bold text-slate-100 text-lg">{todayScene?.headline}</h4>
                  <p className="text-slate-300 font-serif text-base leading-relaxed italic">{todayScene?.narrative}</p>
                </div>
              </div>
            </div>
          </main>
        )}

        {activeLens === 'people' && (
          <main className="flex-1 overflow-y-auto bg-[#0b0d13] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="flex items-center gap-3 border-b border-slate-800/60 pb-4">
              <Users className="w-6 h-6 text-amber-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">People & Bonds</h2>
                <p className="text-xs font-serif italic text-amber-300/80">Family, mentors, companions, and those who share your life</p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {npcs.map((npc) => (
                <div key={npc.id} className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 space-y-3 shadow-sm">
                  <div className="flex justify-between items-baseline">
                    <h3 className="font-serif font-bold text-slate-100 text-base">{npc.name}</h3>
                    <span className="text-xs font-serif italic text-amber-300/90">{npc.relationship_type}</span>
                  </div>
                  <p className="text-xs text-slate-300 font-sans">
                    <strong className="text-slate-400">Rhythm: </strong> {npc.current_activity}
                  </p>
                  <div className="pt-2 border-t border-slate-800/60 text-xs text-slate-400 font-serif italic">
                    Bond: {npc.trust_description}
                  </div>
                </div>
              ))}
            </div>
          </main>
        )}

        {activeLens === 'places' && (
          <main className="flex-1 overflow-y-auto bg-[#0b0d13] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="flex items-center gap-3 border-b border-slate-800/60 pb-4">
              <MapPin className="w-6 h-6 text-amber-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Places & World</h2>
                <p className="text-xs font-serif italic text-amber-300/80">The schools, grounds, streets, and institutions where your story unfolds</p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 space-y-2.5 shadow-sm">
                <h3 className="font-serif font-bold text-slate-100 text-base">District Primary & Secondary Schools</h3>
                <p className="text-xs text-amber-300/80 font-mono">EDUCATIONAL FOUNDATION</p>
                <p className="text-xs text-slate-300 font-serif leading-relaxed">
                  The local classrooms and grounds where discipline, foundational arithmetic, reading, and early mentorship take root.
                </p>
              </div>
              <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 space-y-2.5 shadow-sm">
                <h3 className="font-serif font-bold text-slate-100 text-base">Community Sports & Athletics Ground</h3>
                <p className="text-xs text-amber-300/80 font-mono">TRAINING PITCH & RECREATION</p>
                <p className="text-xs text-slate-300 font-serif leading-relaxed">
                  Open community grounds where youth matches are contested, tactical drills are run, and talent scouts observe from the sidelines.
                </p>
              </div>
            </div>
          </main>
        )}

        {activeLens === 'messages' && (
          <main className="flex-1 overflow-y-auto bg-[#0b0d13] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="flex items-center gap-3 border-b border-slate-800/60 pb-4">
              <Mail className="w-6 h-6 text-amber-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Letters & Notices</h2>
                <p className="text-xs font-serif italic text-amber-300/80">Official correspondence, certifications, and personal notes</p>
              </div>
            </div>
            <div className="space-y-4">
              <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-6 space-y-3 shadow-sm">
                <div className="flex justify-between items-start">
                  <div>
                    <h3 className="font-serif font-bold text-slate-100 text-base">National Examination Registry</h3>
                    <p className="text-xs text-amber-300/80 font-mono">OFFICIAL ENTRY NOTIFICATION</p>
                  </div>
                  <span className="text-xs bg-amber-500/10 text-amber-300 px-3 py-1 rounded-full border border-amber-500/20 font-serif">
                    Official Notice
                  </span>
                </div>
                <p className="text-sm text-slate-200 font-serif leading-relaxed italic">
                  "Candidate enrollment portal is open. Qualifying students preparing for higher education admissions may register their entry."
                </p>
              </div>
            </div>
          </main>
        )}

        {activeLens === 'world' && (
          <main className="flex-1 overflow-y-auto bg-[#0b0d13] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="flex items-center gap-3 border-b border-slate-800/60 pb-4">
              <Globe className="w-6 h-6 text-amber-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Surroundings & Era</h2>
                <p className="text-xs font-serif italic text-amber-300/80">The climate, economic reality, and era surrounding your life</p>
              </div>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 space-y-3 shadow-sm">
                <p className="text-xs font-mono uppercase tracking-wider text-slate-500">Geographic Setting</p>
                <p className="text-sm text-slate-200 font-serif leading-relaxed">
                  Located in {livingState?.location_formatted}. The seasons shift between bright harmattan haze and seasonal rains, shaping the rhythm of outdoor markets, sports, and school terms.
                </p>
              </div>

              <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 space-y-3 shadow-sm">
                <p className="text-xs font-mono uppercase tracking-wider text-slate-500">Economic Climate</p>
                <p className="text-sm text-slate-200 font-serif leading-relaxed">
                  A dynamic urban economy where ambition, education, and family diligence open pathways to trade, engineering, and civic leadership.
                </p>
              </div>
            </div>
          </main>
        )}

        {/* Right Column: Context Panel */}
        <RightContextPanel
          state={livingState}
          npcs={npcs}
          processes={processes}
        />
      </div>

      {devMode && (
        <CausalityInspector
          event={null}
          onClose={() => setDevMode(false)}
        />
      )}
    </div>
  );
};
