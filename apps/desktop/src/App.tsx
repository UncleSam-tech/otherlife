import React, { useState, useEffect } from 'react';
import { GameShell } from './components/game/GameShell';
import { MainMenu, SaveMetadata } from './components/MainMenu';
import { CausalityInspector } from './components/CausalityInspector';
import { LivingStateDTO } from './components/context/ContextPanel';
import { TodaySceneDTO, LastStepResultDTO } from './components/world/SceneRenderer';
import { ContextNpcDTO } from './components/characters/NPCDisplay';
import { ContextProcessDTO } from './components/context/ProcessTracker';
import { NavLens } from './components/navigation/LifeNavigation';
import { NewLifeCreatorConfig } from './components/creation/LifeCreator';
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

  const handleStartNewLife = async (config?: NewLifeCreatorConfig) => {
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
      <div className="flex items-center justify-center h-screen bg-[#07090e] text-amber-200 font-serif">
        <p className="italic text-lg">Opening the book of life...</p>
      </div>
    );
  }

  if (appMode === 'MAIN_MENU') {
    return (
      <MainMenu
        onStartNewLife={(cfg) => handleStartNewLife(cfg)}
        onContinueRecentSave={() => handleStartNewLife()}
        onLoadSave={(filename) => console.log('Load save:', filename)}
        onOpenSettings={() => {}}
        saves={savesList}
        onDeleteSave={async () => { await refreshSavesList(); }}
      />
    );
  }

  return (
    <div className="relative w-screen h-screen overflow-hidden">
      <GameShell
        livingState={livingState}
        todayScene={todayScene}
        lastStepResult={lastStepResult}
        npcs={npcs}
        processes={processes}
        biographyText={biographyText}
        activeLens={activeLens}
        onSelectLens={setActiveLens}
        onSubmitIntent={handleSubmitIntent}
        isLoading={isLoading}
        onReturnToMainMenu={() => setAppMode('MAIN_MENU')}
        devMode={devMode}
        onToggleDevMode={() => setDevMode(!devMode)}
      />

      {devMode && (
        <CausalityInspector
          event={null}
          onClose={() => setDevMode(false)}
        />
      )}
    </div>
  );
};
