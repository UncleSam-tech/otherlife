import React, { useState, useEffect } from 'react';
import { GameShell } from './components/game/GameShell';
import { MainMenu, SaveMetadata } from './components/MainMenu';
import { CausalityInspector } from './components/CausalityInspector';
import { LivingStateDTO } from './components/context/ContextPanel';
import { TodaySceneDTO, LastStepResultDTO } from './components/world/SceneRenderer';
import { ContextNpcDTO } from './components/characters/NPCDisplay';
import { ContextProcessDTO } from './components/context/ProcessTracker';
import { DocumentDTO } from './components/documents/DocumentViewerModal';
import { NavLens } from './components/navigation/LifeNavigation';
import { NewLifeCreatorConfig } from './components/creation/LifeCreator';
import { ChronicleEntryDTO, LetterNotificationDTO, PhoneMessageDTO, StructuredGameplayAction, WorldMapPlaceDTO } from './types/gameplay';
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
  const [documents, setDocuments] = useState<DocumentDTO[]>([]);
  const [phoneMessages, setPhoneMessages] = useState<PhoneMessageDTO[]>([]);
  const [phoneContacts, setPhoneContacts] = useState<ContextNpcDTO[]>([]);
  const [letters, setLetters] = useState<LetterNotificationDTO[]>([]);
  const [lastStepResult, setLastStepResult] = useState<LastStepResultDTO | null>(null);
  const [biographyText, setBiographyText] = useState<string>('');
  const [worldMapPlaces, setWorldMapPlaces] = useState<WorldMapPlaceDTO[]>([]);
  const [chronicleEntries, setChronicleEntries] = useState<ChronicleEntryDTO[]>([]);

  const refreshSavesList = async () => {
    const saves = await callTauriCommand<SaveMetadata[]>('list_saves');
    setSavesList(saves || []);
  };

  const refreshBiography = async () => {
    const bio = await callTauriCommand<string>('get_biography');
    if (bio) setBiographyText(bio);
  };

  const refreshDocuments = async () => {
    const docs = await callTauriCommand<DocumentDTO[]>('get_documents');
    if (docs) setDocuments(docs);
  };

  const refreshPhoneMessages = async () => {
    const messages = await callTauriCommand<PhoneMessageDTO[]>('get_phone_messages');
    if (messages) setPhoneMessages(messages);
  };

  const refreshPhoneContacts = async () => {
    const contacts = await callTauriCommand<ContextNpcDTO[]>('get_phone_contacts');
    if (contacts) setPhoneContacts(contacts);
  };

  const refreshLetters = async () => {
    const inbox = await callTauriCommand<LetterNotificationDTO[]>('get_letters_inbox');
    if (inbox) setLetters(inbox);
  };

  const refreshWorldMap = async () => {
    const places = await callTauriCommand<WorldMapPlaceDTO[]>('get_world_map');
    if (places) setWorldMapPlaces(places);
  };

  const refreshChronicle = async () => {
    const entries = await callTauriCommand<ChronicleEntryDTO[]>('get_life_chronicle');
    if (entries) setChronicleEntries(entries);
  };

  const applyTurnResult = async (
    res: [LivingStateDTO, LastStepResultDTO, TodaySceneDTO, ContextNpcDTO[], ContextProcessDTO[]] | null
  ) => {
    if (!res) return false;
    setLivingState(res[0]);
    setLastStepResult(res[1]);
    setTodayScene(res[2]);
    setNpcs(res[3]);
    setProcesses(res[4]);
    await Promise.all([refreshBiography(), refreshDocuments(), refreshPhoneMessages(), refreshPhoneContacts(), refreshLetters(), refreshWorldMap(), refreshChronicle(), refreshSavesList()]);
    return res[1].success;
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
      creation_mode: 'ORGANIC_BIRTH',
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
      await Promise.all([refreshBiography(), refreshDocuments(), refreshPhoneMessages(), refreshPhoneContacts(), refreshLetters(), refreshWorldMap(), refreshChronicle(), refreshSavesList()]);
      setAppMode('PLAYING');
      setActiveLens('life');
    }
    setIsLoading(false);
  };

  const handleContinueRecentSave = async () => {
    setIsLoading(true);
    const res = await callTauriCommand<[LivingStateDTO, TodaySceneDTO, ContextNpcDTO[], ContextProcessDTO[]]>(
      'continue_recent_save'
    );

    if (res) {
      setLivingState(res[0]);
      setTodayScene(res[1]);
      setNpcs(res[2]);
      setProcesses(res[3]);
      setLastStepResult(null);
      await Promise.all([refreshBiography(), refreshDocuments(), refreshPhoneMessages(), refreshPhoneContacts(), refreshLetters(), refreshWorldMap(), refreshChronicle()]);
      setAppMode('PLAYING');
      setActiveLens('life');
    } else {
      // If no save file found, start fresh life
      await handleStartNewLife();
    }
    setIsLoading(false);
  };

  const handleLoadSave = async (filename: string) => {
    setIsLoading(true);
    const res = await callTauriCommand<[LivingStateDTO, TodaySceneDTO, ContextNpcDTO[], ContextProcessDTO[]]>(
      'load_game',
      { filename }
    );

    if (res) {
      setLivingState(res[0]);
      setTodayScene(res[1]);
      setNpcs(res[2]);
      setProcesses(res[3]);
      setLastStepResult(null);
      await Promise.all([refreshBiography(), refreshDocuments(), refreshPhoneMessages(), refreshPhoneContacts(), refreshLetters(), refreshWorldMap(), refreshChronicle()]);
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
      await refreshDocuments();
      await refreshPhoneContacts();
      await refreshWorldMap();
      await refreshChronicle();
      await refreshSavesList();
    }
    setIsLoading(false);
  };

  const handleAdvanceExplicit = async (actionType: 'HOURS' | 'DAYS' | 'SLEEP' | 'ROUTINE', amount?: number) => {
    if (isLoading) return;
    setIsLoading(true);

    const res = await callTauriCommand<[LivingStateDTO, LastStepResultDTO, TodaySceneDTO, ContextNpcDTO[], ContextProcessDTO[]]>(
      'advance_time_explicit',
      { actionType, amount }
    );

    if (res) {
      setLivingState(res[0]);
      setLastStepResult(res[1]);
      setTodayScene(res[2]);
      setNpcs(res[3]);
      setProcesses(res[4]);
      await refreshBiography();
      await refreshDocuments();
      await refreshPhoneContacts();
      await refreshWorldMap();
      await refreshChronicle();
      await refreshSavesList();
    }
    setIsLoading(false);
  };

  const handleAgeUp = async () => {
    if (isLoading) return;
    setIsLoading(true);
    const res = await callTauriCommand<[LivingStateDTO, LastStepResultDTO, TodaySceneDTO, ContextNpcDTO[], ContextProcessDTO[]]>('age_up_one_year');
    await applyTurnResult(res);
    setIsLoading(false);
  };

  const handleStructuredAction = async (action: StructuredGameplayAction): Promise<boolean> => {
    if (isLoading) return false;
    setIsLoading(true);

    let command = '';
    let args: Record<string, unknown> = {};
    switch (action.type) {
      case 'SEND_MESSAGE':
        command = 'send_phone_message';
        args = { recipientId: action.recipientId, text: action.text };
        break;
      case 'COMMUTE':
        command = 'commute_to_place';
        args = { placeId: action.placeId, transportMode: action.transportMode };
        break;
      case 'CONVERSE':
        command = 'converse_with_npc';
        args = { npcId: action.npcId, dialogue: action.dialogue };
        break;
      case 'BUSINESS_OPERATION':
        command = 'advance_company_operation';
        args = { companyName: action.companyName, operation: action.operation, plan: action.plan };
        break;
      case 'UNIVERSITY_APPLICATION':
        command = 'apply_to_university';
        args = {
          institution: action.institution,
          degreeProgram: action.degreeProgram,
          primaryCourse: action.primaryCourse,
          studyMode: action.studyMode,
          fundingPlan: action.fundingPlan,
        };
        break;
      case 'APPLY_FOR_JOB':
        command = 'apply_for_job';
        args = {
          jobId: action.jobId,
          companyId: action.companyId,
          title: action.title,
          companyName: action.companyName,
          resumeSummary: action.resumeSummary,
          coverLetter: action.coverLetter,
          availability: action.availability,
        };
        break;
      case 'REGISTER_COMPANY':
        command = 'register_company';
        args = {
          name: action.name,
          structure: action.structure,
          partners: action.partners,
          authorizedCapital: action.authorizedCapital,
          businessActivity: action.businessActivity,
          registeredAddress: action.registeredAddress,
        };
        break;
      case 'TRAVEL':
        command = 'travel_to_location';
        args = {
          destinationCityId: action.destinationCityId,
          transportMode: action.transportMode,
          stayDays: action.stayDays,
          operatorName: action.operatorName,
          serviceClass: action.serviceClass,
          fare: action.fare,
          accommodation: action.accommodation,
          departureTiming: action.departureTiming,
          journeyType: action.journeyType,
          immigrationPathway: action.immigrationPathway,
        };
        break;
    }

    const res = await callTauriCommand<[
      LivingStateDTO,
      LastStepResultDTO,
      TodaySceneDTO,
      ContextNpcDTO[],
      ContextProcessDTO[],
    ]>(command, args);
    const success = await applyTurnResult(res);
    setIsLoading(false);
    return success;
  };

  if (appMode === 'BOOTING') {
    return (
      <div className="flex items-center justify-center h-screen bg-[#07090e] text-amber-200 font-serif select-none">
        <p className="italic text-lg">Opening the book of life...</p>
      </div>
    );
  }

  if (appMode === 'MAIN_MENU') {
    return (
      <MainMenu
        onStartNewLife={(cfg) => handleStartNewLife(cfg)}
        onContinueRecentSave={handleContinueRecentSave}
        onLoadSave={handleLoadSave}
        onOpenSettings={() => {}}
        saves={savesList}
        onDeleteSave={async (fname) => {
          await callTauriCommand('delete_save', { filename: fname });
          await refreshSavesList();
        }}
      />
    );
  }

  return (
    <div className="relative w-screen h-screen overflow-hidden select-none font-sans">
      <GameShell
        livingState={livingState}
        todayScene={todayScene}
        lastStepResult={lastStepResult}
        npcs={npcs}
        documents={documents}
        phoneMessages={phoneMessages}
        phoneContacts={phoneContacts}
        letters={letters}
        worldMapPlaces={worldMapPlaces}
        chronicleEntries={chronicleEntries}
        processes={processes}
        biographyText={biographyText}
        activeLens={activeLens}
        onSelectLens={setActiveLens}
        onSubmitIntent={handleSubmitIntent}
        onStructuredAction={handleStructuredAction}
        onAdvanceExplicit={handleAdvanceExplicit}
        onAgeUp={handleAgeUp}
        isLoading={isLoading}
        onReturnToMainMenu={async () => {
          await refreshSavesList();
          setAppMode('MAIN_MENU');
        }}
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
