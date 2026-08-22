import React, { useState } from 'react';
import { PersistentWorldBar } from './PersistentWorldBar';
import { ExpandableNavigation } from '../navigation/ExpandableNavigation';
import { NavLens } from '../navigation/LifeNavigation';
import { LifeChronicle } from '../life/LifeChronicle';
import { DismissibleContextDrawer, ContextDrawerItem } from '../context/DismissibleContextDrawer';
import { ConversationModal } from '../characters/ConversationModal';
import { CalendarModal } from '../calendar/CalendarModal';
import { SimulatedPhoneModal } from '../devices/SimulatedPhoneModal';
import { SimulatedComputerModal } from '../devices/SimulatedComputerModal';
import { DocumentViewerModal, DocumentDTO } from '../documents/DocumentViewerModal';
import { DiegeticDeviceModal } from '../devices/DiegeticDeviceModal';
import { TravelPlannerModal } from '../travel/TravelPlannerModal';
import { UniversityApplicationModal } from '../education/UniversityApplicationModal';
import { CityMap } from '../world/CityMap';
import { MemoryTimeline } from '../context/MemoryTimeline';
import { TodaySceneDTO, LastStepResultDTO } from '../world/SceneRenderer';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { ContextProcessDTO } from '../context/ProcessTracker';
import { LivingStateDTO } from '../context/ContextPanel';
import { ChronicleEntryDTO, LetterNotificationDTO, PhoneMessageDTO, StructuredGameplayAction, WorldMapPlaceDTO } from '../../types/gameplay';
import { Feather, Mail, Globe, FileText } from 'lucide-react';

interface GameShellProps {
  livingState: LivingStateDTO | null;
  todayScene: TodaySceneDTO | null;
  lastStepResult: LastStepResultDTO | null;
  npcs: ContextNpcDTO[];
  documents: DocumentDTO[];
  phoneMessages: PhoneMessageDTO[];
  phoneContacts: ContextNpcDTO[];
  letters: LetterNotificationDTO[];
  worldMapPlaces: WorldMapPlaceDTO[];
  chronicleEntries: ChronicleEntryDTO[];
  processes?: ContextProcessDTO[];
  biographyText: string;
  activeLens: NavLens;
  onSelectLens: (lens: NavLens) => void;
  onSubmitIntent: (intentText: string) => void;
  onStructuredAction: (action: StructuredGameplayAction) => Promise<boolean>;
  onAdvanceExplicit?: (actionType: 'HOURS' | 'DAYS' | 'SLEEP' | 'ROUTINE', amount?: number) => void;
  onAgeUp: () => void;
  isLoading: boolean;
  onReturnToMainMenu: () => void;
  devMode?: boolean;
  onToggleDevMode?: () => void;
}

export const GameShell: React.FC<GameShellProps> = ({
  livingState,
  todayScene,
  lastStepResult,
  npcs,
  documents,
  phoneMessages,
  phoneContacts,
  letters,
  worldMapPlaces,
  chronicleEntries,
  processes = [],
  biographyText,
  activeLens,
  onSelectLens,
  onSubmitIntent,
  onStructuredAction,
  onAdvanceExplicit,
  onAgeUp,
  isLoading,
  onReturnToMainMenu,
  devMode: _devMode,
  onToggleDevMode: _onToggleDevMode,
}) => {
  const playerAge = livingState?.age || 0;
  const ownedCompanyName = documents.find((document) => document.document_type === 'COMPANY_INCORPORATION')?.fields['Company Name'];

  // Modals & Drawers State
  const [drawerItem, setDrawerItem] = useState<ContextDrawerItem>(null);
  const [conversationNpc, setConversationNpc] = useState<ContextNpcDTO | null>(null);
  const [isCalendarOpen, setIsCalendarOpen] = useState(false);
  const [isPhoneOpen, setIsPhoneOpen] = useState(false);
  const [isComputerOpen, setIsComputerOpen] = useState(false);
  const [isDocumentsOpen, setIsDocumentsOpen] = useState(false);
  const [isTravelOpen, setIsTravelOpen] = useState(false);
  const [isUniversityOpen, setIsUniversityOpen] = useState(false);
  const [activeDeviceType, setActiveDeviceType] = useState<'phone' | 'computer' | 'wallet' | 'documents' | 'mail' | null>(null);

  return (
    <div className="flex flex-col h-screen w-screen bg-[#07090e] text-slate-100 overflow-hidden font-sans select-none">
      {/* 1. Persistent World Bar */}
      <PersistentWorldBar
        characterName={livingState?.player_name || 'Living Person'}
        dateTimeFormatted={livingState?.time_formatted || ''}
        locationFormatted={livingState ? `${livingState.current_place_name} · ${livingState.location_formatted}` : 'Living World'}
        weatherName={livingState?.weather_name || 'Seasonal Weather'}
        unreadNotificationsCount={0}
        playerAge={playerAge}
        onOpenCalendar={() => setIsCalendarOpen(true)}
        onOpenPhone={() => setIsPhoneOpen(true)}
        onOpenMessages={() => onSelectLens('messages')}
        onOpenDocuments={() => setIsDocumentsOpen(true)}
        onOpenMenu={onReturnToMainMenu}
      />

      {/* 2. Middle Body: Expandable Navigation + Scene Workspace */}
      <div className="flex flex-1 overflow-hidden relative">
        {/* Compact Expandable Navigation Rail */}
        <ExpandableNavigation
          activeLens={activeLens}
          onSelectLens={onSelectLens}
          unreadCount={0}
        />

        {/* Central Workspace */}
        {activeLens === 'life' && (
          <LifeChronicle
            state={livingState!}
            scene={todayScene}
            lastStepResult={lastStepResult}
            entries={chronicleEntries}
            npcs={npcs}
            processes={processes}
            isLoading={isLoading}
            onAgeUp={onAgeUp}
            onOpenPeople={() => onSelectLens('people')}
            onOpenPlaces={() => onSelectLens('places')}
            onOpenBiography={() => onSelectLens('biography')}
            onOpenComputer={() => setIsComputerOpen(true)}
            onOpenDocuments={() => setIsDocumentsOpen(true)}
            onOpenTravel={() => setIsTravelOpen(true)}
            onOpenUniversity={() => setIsUniversityOpen(true)}
            onOpenPhone={() => setIsPhoneOpen(true)}
            onSelectNpc={(npc) => setDrawerItem({ type: 'npc', data: npc })}
          />
        )}

        {activeLens === 'people' && (
          <main className="flex-1 overflow-y-auto bg-[#07090e] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">People & Bonds</h2>
                <p className="text-xs font-serif italic text-amber-300/80">Click any person to converse or view relationship details</p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {npcs.map((npc) => (
                <button
                  type="button"
                  key={npc.id}
                  onClick={() => setDrawerItem({ type: 'npc', data: npc })}
                  className="w-full bg-[#0d1017] hover:bg-[#131722] border border-[#1b2234] hover:border-amber-500/50 p-5 rounded-2xl cursor-pointer space-y-2.5 text-left transition-all shadow-sm group"
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
                    <span>Click to inspect & converse</span>
                    <span className="text-slate-500">{npc.trust_description}</span>
                  </div>
                </button>
              ))}
            </div>
          </main>
        )}

        {activeLens === 'places' && (
          <CityMap
            cityName={livingState?.location_formatted || 'Living city'}
            currencySymbol={livingState?.currency_symbol || '₦'}
            currencyCode={livingState?.currency_code || 'NGN'}
            places={worldMapPlaces}
            isLoading={isLoading}
            onCommute={(placeId, transportMode) => onStructuredAction({ type: 'COMMUTE', placeId, transportMode })}
            onArrive={() => onSelectLens('life')}
          />
        )}

        {activeLens === 'journal' && (
          <MemoryTimeline
            timeFormatted={livingState?.time_formatted || ''}
            headline={todayScene?.headline}
            narrative={todayScene?.narrative}
          />
        )}

        {activeLens === 'biography' && (
          <main className="flex-1 overflow-y-auto bg-[#07090e] p-8 max-w-3xl mx-auto space-y-6 select-text">
            <div className="bg-[#0d1017] border border-[#1b2234] rounded-2xl p-8 space-y-6 shadow-md">
              <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
                <Feather className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">My Story</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Reflections on the lived path</p>
                </div>
              </div>
              <div className="prose prose-invert max-w-none text-slate-200 font-serif text-base md:text-lg leading-relaxed whitespace-pre-wrap">
                {biographyText || 'The first chapters of life are still unfolding...'}
              </div>
            </div>
          </main>
        )}

        {activeLens === 'messages' && (
          <main className="flex-1 overflow-y-auto bg-[#07090e] p-8 max-w-3xl mx-auto space-y-6 select-text">
            <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
              <div className="flex items-center gap-3">
                <Mail className="w-6 h-6 text-amber-400" />
                <div>
                  <h2 className="text-2xl font-serif font-bold text-slate-100">Letters & Official Documents</h2>
                  <p className="text-xs font-serif italic text-amber-300/80">Official records, birth certificates, and notices</p>
                </div>
              </div>
              <button
                type="button"
                onClick={() => setIsDocumentsOpen(true)}
                className="flex items-center gap-2 bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 border border-amber-500/40 px-3.5 py-1.5 rounded-xl text-xs font-serif transition-colors cursor-pointer"
              >
                <FileText className="w-4 h-4" />
                <span>Inspect Certificates</span>
              </button>
            </div>
            <div className="space-y-4">
              {letters.map((letter) => (
                <article key={letter.id} className="rounded-2xl border border-blue-500/25 bg-blue-500/5 p-6 space-y-3">
                  <div className="flex items-start justify-between gap-4">
                    <div>
                      <p className="text-[10px] font-mono uppercase text-blue-300">Official notice from {letter.sender}</p>
                      <h3 className="mt-1 font-serif font-bold text-slate-100">{letter.subject}</h3>
                    </div>
                    <span className="text-[10px] font-mono text-slate-500">{letter.date_received}</span>
                  </div>
                  <p className="text-xs font-serif leading-relaxed text-slate-300">{letter.body}</p>
                </article>
              ))}
              {documents.map((doc) => (
                <button
                  type="button"
                  key={doc.id}
                  onClick={() => setIsDocumentsOpen(true)}
                  className="w-full bg-[#0d1017] hover:bg-[#131722] border border-[#1b2234] hover:border-amber-500/40 rounded-2xl p-6 space-y-3 text-left shadow-sm cursor-pointer transition-colors"
                >
                  <div className="flex justify-between items-start">
                    <div>
                      <h3 className="font-serif font-bold text-slate-100 text-base">{doc.title}</h3>
                      <p className="text-xs text-amber-300/80 font-mono uppercase">{doc.issuing_authority}</p>
                    </div>
                    <span className="text-xs bg-amber-500/10 text-amber-300 px-3 py-1 rounded-full border border-amber-500/20 font-serif">
                      Reg: {doc.registration_number}
                    </span>
                  </div>
                  <p className="text-xs text-slate-300 font-serif leading-relaxed italic">
                    Click to view full verified registry record issued on {doc.issue_date}.
                  </p>
                </button>
              ))}
            </div>
          </main>
        )}

        {activeLens === 'world' && (
          <main className="flex-1 overflow-y-auto bg-[#07090e] p-8 max-w-3xl mx-auto space-y-6 select-text">
            <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
              <Globe className="w-6 h-6 text-amber-400" />
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Surrounding World & Era</h2>
                <p className="text-xs font-serif italic text-amber-300/80">Regional setting and institutions</p>
              </div>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-[#0d1017] border border-[#1b2234] rounded-2xl p-5 space-y-2 shadow-sm">
                <p className="text-xs font-mono uppercase tracking-wider text-slate-500">Geographic Setting</p>
                <p className="text-sm text-slate-200 font-serif leading-relaxed">
                  Currently residing in {livingState?.location_formatted}. Daily life balances family commitments, education, and career development.
                </p>
              </div>
              <div className="bg-[#0d1017] border border-[#1b2234] rounded-2xl p-5 space-y-2 shadow-sm">
                <p className="text-xs font-mono uppercase tracking-wider text-slate-500">Living Environment</p>
                <p className="text-sm text-slate-200 font-serif leading-relaxed">
                  A dynamic urban landscape where education, discipline, and community bonds open long-term opportunities.
                </p>
              </div>
            </div>
            <section className="rounded-2xl border border-[#1b2234] bg-[#0d1017] p-5 space-y-4">
              <div>
                <p className="text-xs font-mono uppercase tracking-wider text-slate-500">Active Processes</p>
                <p className="mt-1 text-xs font-serif text-slate-300">Applications, registrations, journeys, and other multi-step commitments persist here.</p>
              </div>
              {processes.length === 0 ? (
                <p className="rounded-xl border border-dashed border-[#27304a] p-4 text-xs text-slate-500">No tracked process has started yet.</p>
              ) : (
                <div className="space-y-3">
                  {processes.map((process) => (
                    <div key={process.id} className="rounded-xl border border-[#27304a] bg-[#121622] p-4 space-y-2">
                      <div className="flex items-start justify-between gap-4">
                        <div>
                          <p className="font-serif text-sm font-bold text-slate-100">{process.title}</p>
                          <p className="mt-1 text-[10px] font-mono text-amber-300">{process.status.replace(/_/g, ' ')}</p>
                        </div>
                        <span className="text-xs text-slate-400">{process.current_step}/{process.total_steps}</span>
                      </div>
                      <div className="h-1.5 overflow-hidden rounded-full bg-[#07090e]">
                        <div className="h-full rounded-full bg-amber-400" style={{ width: `${process.progress_percent}%` }} />
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </section>
          </main>
        )}
      </div>

      {/* 3. Dismissible Context Drawer */}
      <DismissibleContextDrawer
        item={drawerItem}
        onClose={() => setDrawerItem(null)}
        onExecuteAction={onSubmitIntent}
        onOpenConversation={(npc) => setConversationNpc(npc)}
        isLoading={isLoading}
      />

      {/* 4. In-Depth Conversation Modal */}
      {conversationNpc && (
        <ConversationModal
          npc={conversationNpc}
          onClose={() => setConversationNpc(null)}
          onSendMessage={(dialogue) => onStructuredAction({ type: 'CONVERSE', npcId: conversationNpc.id, dialogue })}
          isLoading={isLoading}
        />
      )}

      {/* 5. In-World Calendar Modal */}
      {isCalendarOpen && (
        <CalendarModal
          timeFormatted={livingState?.time_formatted || ''}
          playerAge={playerAge}
          onClose={() => setIsCalendarOpen(false)}
          onAdvanceTime={onSubmitIntent}
          onAdvanceExplicit={onAdvanceExplicit}
          isLoading={isLoading}
        />
      )}

      {/* 6. In-World Smartphone Modal */}
      {isPhoneOpen && (
        <SimulatedPhoneModal
          onClose={() => setIsPhoneOpen(false)}
          playerAge={playerAge}
          cash={livingState?.cash || 0}
          currencySymbol={livingState?.currency_symbol || '₦'}
          contacts={phoneContacts}
          messages={phoneMessages}
          onExecuteAction={onSubmitIntent}
          onStructuredAction={onStructuredAction}
          isLoading={isLoading}
        />
      )}

      {/* 7. In-World Computer Modal */}
      {isComputerOpen && (
        <SimulatedComputerModal
          onClose={() => setIsComputerOpen(false)}
          playerAge={playerAge}
          currencySymbol={livingState?.currency_symbol || '₦'}
          onExecuteAction={onSubmitIntent}
          onStructuredAction={onStructuredAction}
          isLoading={isLoading}
          ownedCompanyName={ownedCompanyName}
        />
      )}

      {/* 8. Inspectable Document Viewer Modal */}
      {isDocumentsOpen && (
        <DocumentViewerModal
          documents={documents}
          onClose={() => setIsDocumentsOpen(false)}
        />
      )}

      {isTravelOpen && (
        <TravelPlannerModal
          currentLocation={livingState?.location_formatted || 'Current location'}
          playerName={livingState?.player_name || 'Living Person'}
          currencySymbol={livingState?.currency_symbol || '₦'}
          currencyCode={livingState?.currency_code || 'NGN'}
          isLoading={isLoading}
          onClose={() => setIsTravelOpen(false)}
          onStructuredAction={onStructuredAction}
        />
      )}

      {isUniversityOpen && (
        <UniversityApplicationModal
          institution={livingState?.current_place_name || 'Metropolitan University'}
          currencySymbol={livingState?.currency_symbol || '₦'}
          isLoading={isLoading}
          onClose={() => setIsUniversityOpen(false)}
          onStructuredAction={onStructuredAction}
        />
      )}

      {/* 9. Additional Diegetic Device Modal */}
      {activeDeviceType && (
        <DiegeticDeviceModal
          deviceType={activeDeviceType}
          onClose={() => setActiveDeviceType(null)}
          cash={livingState?.cash || 0}
          currencySymbol={livingState?.currency_symbol || '₦'}
          onExecuteAction={onSubmitIntent}
          isLoading={isLoading}
        />
      )}
    </div>
  );
};
