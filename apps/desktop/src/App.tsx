import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { TodayView, TodaySceneDTO } from './components/TodayView';
import { LifeChronicle } from './components/LifeChronicle';
import { LifeJournalDrawer } from './components/LifeJournalDrawer';
import { FeedEvent } from './components/LifeFeed';
import { SidebarStateData } from './components/NowSidebar';
import { ActionPromptBar } from './components/ActionPromptBar';
import { CausalityInspector } from './components/CausalityInspector';
import { MainMenu, SaveMetadata } from './components/MainMenu';
import { CreationWizard, NewLifeFormState } from './components/creator/CreationWizard';

import './styles/globals.css';

export type AppMode = 'BOOTING' | 'MAIN_MENU' | 'CREATING_LIFE' | 'PLAYING' | 'SETTINGS' | 'ERROR';

export interface GameStateDTO {
  timeFormatted: string;
  year: number;
  month: number;
  day: number;
  age: number;
  isAlive: boolean;
  cash: number;
  location: string;
  playerName: string;
  activeInterest: string;
  eventCount: number;
  interests: string[];
  goals: string[];
  lifeStage: string;
  maritalStatus: string;
  jobTitle: string;
  monthlySalary: number;
  housingType: string;
  fitness: number;
  stress: number;
}

async function callTauriCommand<T>(cmd: string, args: Record<string, any> = {}): Promise<T | null> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    return await invoke<T>(cmd, args);
  } catch (err) {
    console.warn(`[Tauri Fallback]: ${cmd} fallback mode executed`, err);
    return null;
  }
}

export const App: React.FC = () => {
  const [appMode, setAppMode] = useState<AppMode>('BOOTING');
  const [devMode, setDevMode] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [inspectingEvent, setInspectingEvent] = useState<FeedEvent | null>(null);
  const [isJournalOpen, setIsJournalOpen] = useState(false);

  const [registries, setRegistries] = useState<any>(null);
  const [savesList, setSavesList] = useState<SaveMetadata[]>([]);
  const [activeGame, setActiveGame] = useState<GameStateDTO | null>(null);

  const [todayScene, setTodayScene] = useState<TodaySceneDTO | null>(null);
  const [sidebarData, setSidebarData] = useState<SidebarStateData | null>(null);
  const [events, setEvents] = useState<FeedEvent[]>([]);
  const [biographyText, setBiographyText] = useState<string>('');

  // Derived currency symbol
  const getCurrencySymbol = (locId?: string): string => {
    if (!locId) return '£';
    if (locId.includes('lagos') || locId.includes('abuja') || locId.includes('nigeria')) return '₦';
    if (locId.includes('new_york') || locId.includes('united_states') || locId.includes('usa')) return '$';
    if (locId.includes('madrid') || locId.includes('paris') || locId.includes('berlin')) return '€';
    return '£';
  };

  const refreshSavesList = async () => {
    const saves = await callTauriCommand<SaveMetadata[]>('list_saves');
    if (saves) {
      setSavesList(saves);
    } else {
      setSavesList([]);
    }
  };

  const refreshBiography = async () => {
    const bio = await callTauriCommand<string>('get_biography');
    if (bio) {
      setBiographyText(bio);
    }
  };

  useEffect(() => {
    const initBoot = async () => {
      const reg = await callTauriCommand<any>('get_registries');
      setRegistries(reg);

      await refreshSavesList();
      setAppMode('MAIN_MENU');
    };

    initBoot();
  }, []);

  const updateGameStateFromBackend = (dto: any, scene: any, _sits: any[], sbar: any) => {
    const gameDto: GameStateDTO = {
      timeFormatted: dto.time_formatted,
      year: dto.year,
      month: dto.month,
      day: dto.day,
      age: dto.age,
      isAlive: dto.is_alive,
      cash: dto.cash,
      location: dto.location,
      playerName: dto.player_name,
      activeInterest: dto.active_interest,
      eventCount: dto.event_count,
      interests: dto.interests,
      goals: dto.goals,
      lifeStage: dto.life_stage,
      maritalStatus: dto.marital_status,
      jobTitle: dto.job_title,
      monthlySalary: dto.monthly_salary,
      housingType: dto.housing_type,
      fitness: dto.fitness,
      stress: dto.stress,
    };

    setActiveGame(gameDto);
    setTodayScene(scene);
    setSidebarData(sbar);
  };

  const handleStartCustomLife = async (formState: NewLifeFormState) => {
    setIsLoading(true);

    const configPayload = {
      creation_mode: 'CUSTOM',
      starting_year: formState.startingYear,
      country_id: formState.countryId,
      location_id: formState.locationId,
      starting_age: formState.startingAge,
      first_name: formState.firstName,
      last_name: formState.lastName,
      sex: formState.sex,
      household_income_tier: formState.householdIncomeTier,
      traits: formState.traits,
      skills: formState.skills,
      interests: formState.interests,
      goals: formState.goals,
    };

    const res = await callTauriCommand<[any, any, any[], any]>('start_new_life', { config: configPayload, seed: Date.now() % 100000 });

    if (res && res[0]) {
      const [dto, scene, sits, sbar] = res;
      updateGameStateFromBackend(dto, scene, sits, sbar);

      const locName = dto.location.replace('city:real:', '').replace('city:sim:', '').replace('_', ' ');
      setEvents([
        {
          id: String(Date.now()),
          timestamp: dto.time_formatted,
          eventType: 'NEW_LIFE',
          summary: `You began your life as ${dto.player_name} (Age ${dto.age}) in ${locName.toUpperCase()}.`,
          causalityNote: 'New life initiated with realistic world conditions.',
        },
      ]);
      setAppMode('PLAYING');
      refreshBiography();
    } else {
      alert('Unable to initialize backend life engine. Please verify Tauri backend is running.');
      setAppMode('MAIN_MENU');
    }

    setIsLoading(false);
  };

  const handleContinueRecentSave = async () => {
    if (savesList.length === 0) return;
    await handleLoadSave(savesList[0].filename);
  };

  const handleLoadSave = async (filename: string) => {
    setIsLoading(true);
    const res = await callTauriCommand<[any, any, any[], any]>('load_game_state', { filename });

    if (res && res[0]) {
      const [dto, scene, sits, sbar] = res;
      updateGameStateFromBackend(dto, scene, sits, sbar);

      setEvents([
        {
          id: String(Date.now()),
          timestamp: dto.time_formatted,
          eventType: 'LOAD_TIMELINE',
          summary: `Loaded timeline save for ${dto.player_name} at Age ${dto.age}.`,
          causalityNote: 'Loaded from SQLite save state.',
        },
      ]);
      setAppMode('PLAYING');
      refreshBiography();
    } else {
      alert('Failed to load save file.');
    }
    setIsLoading(false);
  };

  const handleDeleteSave = async (filename: string) => {
    await callTauriCommand<boolean>('delete_save', { filename });
    await refreshSavesList();
  };

  const handleSelectChoice = async (choiceId: string) => {
    if (!activeGame) return;
    setIsLoading(true);

    const res = await callTauriCommand<[any, any, any, any[], any]>('resolve_situation_choice', {
      situation_id: 'today_scene_situation',
      choice_id: choiceId,
    });

    if (res && res[0] && res[1]) {
      const [dto, stepRes, scene, sits, sbar] = res;
      updateGameStateFromBackend(dto, scene, sits, sbar);

      const newEv: FeedEvent = {
        id: stepRes.event_record.id,
        timestamp: stepRes.event_record.timestamp,
        eventType: stepRes.event_record.event_type,
        summary: stepRes.narrative,
        causalityNote: stepRes.causality_note,
        success: stepRes.success,
      };

      setEvents((prev) => [newEv, ...prev]);
      refreshBiography();
    } else {
      alert('Unable to process choice.');
    }

    setIsLoading(false);
  };

  const handleAdvanceTime = async (days: number) => {
    if (!activeGame) return;
    setIsLoading(true);

    const res = await callTauriCommand<[any, any, any, any[], any]>('advance_time', { days });

    if (res && res[0] && res[1]) {
      const [dto, stepRes, scene, sits, sbar] = res;
      updateGameStateFromBackend(dto, scene, sits, sbar);

      const newEv: FeedEvent = {
        id: stepRes.event_record.id,
        timestamp: stepRes.event_record.timestamp,
        eventType: stepRes.event_record.event_type,
        summary: stepRes.narrative,
        causalityNote: stepRes.causality_note,
        success: stepRes.success,
      };

      setEvents((prev) => [newEv, ...prev]);
      refreshBiography();
    } else {
      alert('Unable to advance time.');
    }

    setIsLoading(false);
  };

  const handleSubmitAction = async (inputText: string) => {
    if (!activeGame) return;
    setIsLoading(true);

    const res = await callTauriCommand<[any, any, any, any[], any]>('submit_player_action', { input_text: inputText });

    if (res && res[0] && res[1]) {
      const [dto, stepRes, scene, sits, sbar] = res;
      updateGameStateFromBackend(dto, scene, sits, sbar);

      const newEv: FeedEvent = {
        id: stepRes.event_record.id,
        timestamp: stepRes.event_record.timestamp,
        eventType: stepRes.event_record.event_type,
        summary: stepRes.narrative,
        causalityNote: stepRes.causality_note,
        success: stepRes.success,
      };

      setEvents((prev) => [newEv, ...prev]);
      refreshBiography();
    } else {
      alert('Unable to process action.');
    }

    setIsLoading(false);
  };

  // Render App Mode State Machine
  if (appMode === 'CREATING_LIFE') {
    return (
      <CreationWizard
        registries={registries}
        onClose={() => setAppMode('MAIN_MENU')}
        onSubmitNewLife={handleStartCustomLife}
      />
    );
  }

  if (appMode === 'MAIN_MENU' || appMode === 'BOOTING' || !activeGame) {
    return (
      <MainMenu
        saves={savesList}
        onStartNewLife={() => setAppMode('CREATING_LIFE')}
        onContinueRecentSave={handleContinueRecentSave}
        onLoadSave={handleLoadSave}
        onDeleteSave={handleDeleteSave}
        onOpenSettings={() => alert('Settings mode enabled.')}
      />
    );
  }

  const actionSuggestions = todayScene?.choices.map((c) => c.label).slice(0, 3) || [];

  // PLAYING MODE — Living an alternate life in an immersive world
  return (
    <div style={{
      width: '100vw',
      height: '100vh',
      display: 'grid',
      gridTemplateRows: 'auto 1fr auto',
      gridTemplateColumns: '1fr',
      backgroundColor: 'var(--bg-app)',
      color: 'var(--text-primary)',
      overflow: 'hidden',
    }}>
      {/* Header */}
      <Header
        timeFormatted={activeGame.timeFormatted}
        age={activeGame.age}
        cash={activeGame.cash}
        location={activeGame.location}
        playerName={activeGame.playerName}
        currencySymbol={getCurrencySymbol(activeGame.location)}
        devMode={devMode}
        onToggleDevMode={() => setDevMode(!devMode)}
        onReturnToMainMenu={() => {
          refreshSavesList();
          setAppMode('MAIN_MENU');
        }}
      />

      {/* Main Living Experience Screen */}
      <main style={{
        overflowY: 'auto',
        display: 'flex',
        flexDirection: 'column',
        gap: '24px',
      }}>
        {todayScene && (
          <TodayView
            scene={todayScene}
            onSelectChoice={handleSelectChoice}
            onAdvanceTime={handleAdvanceTime}
            onOpenJournal={() => setIsJournalOpen(true)}
            isLoading={isLoading}
          />
        )}

        <LifeChronicle
          events={events}
          playerName={activeGame.playerName}
        />
      </main>

      {/* Bottom Action Prompt Bar */}
      <ActionPromptBar
        suggestions={actionSuggestions}
        onSubmitAction={handleSubmitAction}
        isLoading={isLoading}
      />

      {/* Slide-out Personal Journal Drawer */}
      <LifeJournalDrawer
        isOpen={isJournalOpen}
        onClose={() => setIsJournalOpen(false)}
        gameState={activeGame}
        sidebarData={sidebarData}
        biography={biographyText}
      />

      {/* Causality Inspector (Dev Mode only) */}
      {inspectingEvent && (
        <CausalityInspector
          event={inspectingEvent}
          onClose={() => setInspectingEvent(null)}
        />
      )}
    </div>
  );
};


