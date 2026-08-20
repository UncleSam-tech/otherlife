import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { LifeNav } from './components/LifeNav';
import { LifeFeed, FeedEvent } from './components/LifeFeed';
import { NowSidebar, SidebarStateData } from './components/NowSidebar';
import { ActionPromptBar } from './components/ActionPromptBar';
import { CausalityInspector } from './components/CausalityInspector';
import { MainMenu, SaveMetadata } from './components/MainMenu';
import { CreationWizard, NewLifeFormState } from './components/creator/CreationWizard';

import { EducationView } from './components/views/EducationView';
import { CareerView } from './components/views/CareerView';
import { HealthView } from './components/views/HealthView';
import { FamilyRomanceView } from './components/views/FamilyRomanceView';
import { WorldNewsView } from './components/views/WorldNewsView';
import { BiographyView } from './components/views/BiographyView';
import { FootballView } from './components/views/FootballView';
import { PoliticsView } from './components/views/PoliticsView';

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
  const [activeTab, setActiveTab] = useState('overview');
  const [devMode, setDevMode] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [inspectingEvent, setInspectingEvent] = useState<FeedEvent | null>(null);

  const [registries, setRegistries] = useState<any>(null);
  const [savesList, setSavesList] = useState<SaveMetadata[]>([]);
  const [activeGame, setActiveGame] = useState<GameStateDTO | null>(null);

  const [suggestions, setSuggestions] = useState<string[]>([]);
  const [sidebarData, setSidebarData] = useState<SidebarStateData | null>(null);
  const [events, setEvents] = useState<FeedEvent[]>([]);

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

  useEffect(() => {
    const initBoot = async () => {
      const reg = await callTauriCommand<any>('get_registries');
      setRegistries(reg);

      await refreshSavesList();
      setAppMode('MAIN_MENU');
    };

    initBoot();
  }, []);

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

    const res = await callTauriCommand<[any, string[], any]>('start_new_life', { config: configPayload, seed: Date.now() % 100000 });

    if (res && res[0]) {
      const [dto, suggs, sbar] = res;
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
      setSuggestions(suggs);
      setSidebarData(sbar);
      setEvents([
        {
          id: String(Date.now()),
          timestamp: dto.time_formatted,
          eventType: 'NEW_LIFE',
          summary: `You began your life as ${dto.player_name} (Age ${dto.age}) in ${dto.location.replace('city:real:', '').toUpperCase()}.`,
          causalityNote: 'New Life created with custom parameters.',
        },
      ]);
      setAppMode('PLAYING');
    } else {
      // Backend error fallback
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
    const res = await callTauriCommand<[any, string[], any]>('load_game_state', { filename });

    if (res && res[0]) {
      const [dto, suggs, sbar] = res;
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
      setSuggestions(suggs);
      setSidebarData(sbar);
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
    } else {
      alert('Failed to load save file.');
    }
    setIsLoading(false);
  };

  const handleDeleteSave = async (filename: string) => {
    await callTauriCommand<boolean>('delete_save', { filename });
    await refreshSavesList();
  };

  const handleSubmitAction = async (inputText: string) => {
    if (!activeGame) return;
    setIsLoading(true);

    const res = await callTauriCommand<[any, any, string[], any]>('submit_player_action', { inputText });

    if (res && res[0] && res[1]) {
      const [dto, stepRes, suggs, sbar] = res;
      setActiveGame({
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
      });

      setSuggestions(suggs);
      setSidebarData(sbar);

      const newEv: FeedEvent = {
        id: stepRes.event_record.id,
        timestamp: stepRes.event_record.timestamp,
        eventType: stepRes.event_record.event_type,
        summary: stepRes.narrative,
        causalityNote: stepRes.causality_note,
        success: stepRes.success,
      };

      setEvents((prev) => [newEv, ...prev]);
    } else {
      alert('Unable to process action. Game state was not changed.');
    }

    setIsLoading(false);
  };

  const renderMainViewContent = () => {
    if (!activeGame) return null;
    const currency = getCurrencySymbol(activeGame.location);

    switch (activeTab) {
      case 'overview':
        return (
          <LifeFeed
            events={events}
            onInspectCausality={(ev) => setInspectingEvent(ev)}
            devMode={devMode}
          />
        );
      case 'education':
        return <EducationView gradeLevel={9} academicPerformance={65} qualifications={[]} onStudy={() => handleSubmitAction('Study academic concepts')} />;
      case 'career':
        return <CareerView jobTitle={activeGame.jobTitle} monthlySalary={activeGame.monthlySalary} onApplyJob={() => handleSubmitAction('Apply for new job')} onWorkShift={() => handleSubmitAction('Work shift')} />;
      case 'health':
        return <HealthView fitness={activeGame.fitness} stress={activeGame.stress} onSeekMedicalTreatment={() => handleSubmitAction('Seek medical care')} />;
      case 'family_romance':
        return <FamilyRomanceView maritalStatus={activeGame.maritalStatus} onDate={() => handleSubmitAction('Go on a date')} onMarry={() => handleSubmitAction('Marry partner')} onDivorce={() => handleSubmitAction('Divorce')} onHaveChild={() => handleSubmitAction('Have a child')} />;
      case 'football':
        return <FootballView weeklyWage={0} currencySymbol={currency} onTrain={() => handleSubmitAction('Attend casual football training')} onPlayMatch={() => handleSubmitAction('Play match')} />;
      case 'politics':
        return <PoliticsView partyName="Unaffiliated" officeTitle="No Active Office" isCampaigning={false} pollingPct={0} onLaunchCampaign={() => handleSubmitAction('Launch campaign')} onHoldRally={() => handleSubmitAction('Host campaign rally')} />;
      case 'world_news':
        return <WorldNewsView newsItems={[]} />;
      case 'biography':
        return <BiographyView biographyText="Life biography recording underway." />;
      default:
        return (
          <LifeFeed
            events={events}
            onInspectCausality={(ev) => setInspectingEvent(ev)}
            devMode={devMode}
          />
        );
    }
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

  // PLAYING MODE
  return (
    <div style={{
      width: '100vw',
      height: '100vh',
      display: 'grid',
      gridTemplateRows: 'auto auto 1fr auto',
      gridTemplateColumns: '1fr auto',
      backgroundColor: 'var(--bg-app)',
      color: 'var(--text-primary)',
      overflow: 'hidden',
    }}>
      <div style={{ gridColumn: '1 / -1' }}>
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
      </div>

      <div style={{ gridColumn: '1 / -1' }}>
        <LifeNav activeTab={activeTab} onSelectTab={setActiveTab} />
      </div>

      <div style={{ overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
        {renderMainViewContent()}
      </div>

      {sidebarData && (
        <div style={{ gridRow: '3 / 4', gridColumn: '2 / 3' }}>
          <NowSidebar sidebarData={sidebarData} devMode={devMode} />
        </div>
      )}

      <div style={{ gridColumn: '1 / -1' }}>
        <ActionPromptBar
          suggestions={suggestions}
          onSubmitAction={handleSubmitAction}
          isLoading={isLoading}
        />
      </div>

      {inspectingEvent && (
        <CausalityInspector
          event={inspectingEvent}
          onClose={() => setInspectingEvent(null)}
        />
      )}
    </div>
  );
};
