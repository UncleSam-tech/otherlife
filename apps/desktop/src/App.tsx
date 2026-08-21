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
      <div className="flex items-center justify-center h-screen bg-slate-950 text-slate-400 font-mono">
        <p>Booting OTHERLIFE Simulation Engine...</p>
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
    <div className="flex flex-col h-screen w-screen bg-slate-950 text-slate-100 overflow-hidden font-sans">
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
          <main className="flex-1 overflow-y-auto bg-slate-950 p-8 max-w-4xl mx-auto">
            <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-8 space-y-6">
              <div className="flex items-center gap-3 border-b border-slate-800 pb-4">
                <Feather className="w-6 h-6 text-emerald-400" />
                <h2 className="text-2xl font-serif font-bold text-slate-100">Autobiographical Life Memoir</h2>
              </div>
              <div className="prose prose-invert max-w-none text-slate-300 font-serif leading-relaxed whitespace-pre-wrap">
                {biographyText || 'Your life chronicle is currently being recorded as moments unfold.'}
              </div>
            </div>
          </main>
        )}

        {activeLens === 'journal' && (
          <main className="flex-1 overflow-y-auto bg-slate-950 p-8 max-w-4xl mx-auto">
            <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-8 space-y-6">
              <div className="flex items-center gap-3 border-b border-slate-800 pb-4">
                <BookOpen className="w-6 h-6 text-emerald-400" />
                <h2 className="text-2xl font-serif font-bold text-slate-100">Episodic Chronicle & Defining Moments</h2>
              </div>
              <p className="text-slate-400 text-sm">Every significant decision, consequence, and milestone recorded in time.</p>
              <div className="space-y-4">
                <div className="border-l-2 border-emerald-500 pl-4 py-1">
                  <span className="text-xs font-mono text-emerald-400">{livingState?.time_formatted}</span>
                  <h4 className="font-semibold text-slate-200 text-sm mt-1">{todayScene?.headline}</h4>
                  <p className="text-xs text-slate-400 mt-1">{todayScene?.narrative}</p>
                </div>
              </div>
            </div>
          </main>
        )}

        {activeLens === 'people' && (
          <main className="flex-1 overflow-y-auto bg-slate-950 p-8 max-w-4xl mx-auto space-y-6">
            <div className="flex items-center gap-3 border-b border-slate-800 pb-4">
              <Users className="w-6 h-6 text-emerald-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Social Network & Living NPCs</h2>
                <p className="text-xs text-slate-400">Autonomous people living independent lives in your world.</p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {npcs.map((npc) => (
                <div key={npc.id} className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-3">
                  <div className="flex justify-between items-start">
                    <div>
                      <h3 className="font-semibold text-slate-100">{npc.name}</h3>
                      <p className="text-xs font-mono text-emerald-400">{npc.relationship_type}</p>
                    </div>
                    <span className="text-xs bg-slate-800 text-slate-400 px-2 py-0.5 rounded font-mono">
                      {npc.trust_description}
                    </span>
                  </div>
                  <p className="text-xs text-slate-400">
                    <strong className="text-slate-300">Schedule:</strong> {npc.current_activity}
                  </p>
                </div>
              ))}
            </div>
          </main>
        )}

        {activeLens === 'places' && (
          <main className="flex-1 overflow-y-auto bg-slate-950 p-8 max-w-4xl mx-auto space-y-6">
            <div className="flex items-center gap-3 border-b border-slate-800 pb-4">
              <MapPin className="w-6 h-6 text-emerald-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Places & Institutions</h2>
                <p className="text-xs text-slate-400">Schools, clubs, civic centers, and municipal hubs in your region.</p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-2">
                <h3 className="font-semibold text-slate-100">Abuja Model Primary School</h3>
                <p className="text-xs text-emerald-400 font-mono">PRIMARY EDUCATION · GARKI</p>
                <p className="text-xs text-slate-400">Public academic institution providing foundational numeracy, English, and civic studies.</p>
              </div>
              <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-2">
                <h3 className="font-semibold text-slate-100">Area 10 Community Sports Ground</h3>
                <p className="text-xs text-emerald-400 font-mono">ATHLETICS & FOOTBALL PITCH · GARKI</p>
                <p className="text-xs text-slate-400">Open community grounds hosting youth training, weekend match fixtures, and grassroots scouts.</p>
              </div>
            </div>
          </main>
        )}

        {activeLens === 'messages' && (
          <main className="flex-1 overflow-y-auto bg-slate-950 p-8 max-w-4xl mx-auto space-y-6">
            <div className="flex items-center gap-3 border-b border-slate-800 pb-4">
              <Mail className="w-6 h-6 text-emerald-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Letters, School Notices & Mail</h2>
                <p className="text-xs text-slate-400">Official correspondence, examination slips, and family communications.</p>
              </div>
            </div>
            <div className="space-y-4">
              <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-3">
                <div className="flex justify-between items-start">
                  <div>
                    <h3 className="font-semibold text-slate-100">West African Examinations Council (WAEC)</h3>
                    <p className="text-xs text-emerald-400 font-mono">SENIOR SCHOOL CERTIFICATE EXAMINATION</p>
                  </div>
                  <span className="text-xs bg-emerald-500/10 text-emerald-400 px-2.5 py-0.5 rounded border border-emerald-500/20">
                    Official Notice
                  </span>
                </div>
                <p className="text-sm text-slate-300 font-serif leading-relaxed">
                  "Registration portal is now open for candidates presenting 5 credit passes including Mathematics and English language."
                </p>
              </div>

              <div className="bg-slate-900/60 border border-slate-800 rounded-xl p-5 space-y-3">
                <div className="flex justify-between items-start">
                  <div>
                    <h3 className="font-semibold text-slate-100">Abuja Model Primary School</h3>
                    <p className="text-xs text-emerald-400 font-mono">HEAD OF MATHEMATICS · MR. ADEWALE</p>
                  </div>
                  <span className="text-xs bg-slate-800 text-slate-400 px-2 py-0.5 rounded">
                    Archived
                  </span>
                </div>
                <p className="text-sm text-slate-300 font-serif leading-relaxed">
                  "Term academic commendation awarded for disciplined classroom arithmetic and problem set accuracy."
                </p>
              </div>
            </div>
          </main>
        )}

        {activeLens === 'world' && (
          <main className="flex-1 overflow-y-auto bg-slate-950 p-8 max-w-4xl mx-auto space-y-6">
            <div className="flex items-center gap-3 border-b border-slate-800 pb-4">
              <Globe className="w-6 h-6 text-emerald-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">World & Economic Indicators</h2>
                <p className="text-xs text-slate-400">Macro-economic trends, national news, and historical era.</p>
              </div>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-3">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-400">Location</span>
                  <span className="font-mono text-slate-200">Garki, Abuja, Nigeria</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-400">National Currency</span>
                  <span className="font-mono text-emerald-400">Nigerian Naira (₦)</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-400">Climate Zone</span>
                  <span className="font-mono text-slate-300">Tropical Savanna</span>
                </div>
              </div>

              <div className="bg-slate-900 border border-slate-800 rounded-xl p-5 space-y-3">
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-400">Annual Inflation Rate</span>
                  <span className="font-mono text-amber-400">11.2%</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-400">Power Grid Reliability</span>
                  <span className="font-mono text-slate-300">75% (Occasional outages)</span>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <span className="text-slate-400">Living Cost Index</span>
                  <span className="font-mono text-emerald-400">1.15 (Capital District)</span>
                </div>
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
