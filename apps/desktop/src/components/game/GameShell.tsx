import React, { useState } from 'react';
import { PersistentWorldBar } from './PersistentWorldBar';
import { ExpandableNavigation } from '../navigation/ExpandableNavigation';
import { NavLens } from '../navigation/LifeNavigation';
import { SceneWorkspace } from '../world/SceneWorkspace';
import { DismissibleContextDrawer, ContextDrawerItem } from '../context/DismissibleContextDrawer';
import { ConversationModal } from '../characters/ConversationModal';
import { CalendarModal } from '../calendar/CalendarModal';
import { SimulatedPhoneModal } from '../devices/SimulatedPhoneModal';
import { SimulatedComputerModal } from '../devices/SimulatedComputerModal';
import { DocumentViewerModal, DocumentDTO } from '../documents/DocumentViewerModal';
import { DiegeticDeviceModal } from '../devices/DiegeticDeviceModal';
import { TravelPlannerModal } from '../travel/TravelPlannerModal';
import { MemoryTimeline } from '../context/MemoryTimeline';
import { TodaySceneDTO, LastStepResultDTO } from '../world/SceneRenderer';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { ContextProcessDTO } from '../context/ProcessTracker';
import { PlaceLocationDTO } from '../world/PlaceInteractionModal';
import { LivingStateDTO } from '../context/ContextPanel';
import { LetterNotificationDTO, PhoneMessageDTO, StructuredGameplayAction } from '../../types/gameplay';
import { Feather, Mail, Globe, Send, FileText } from 'lucide-react';

interface GameShellProps {
  livingState: LivingStateDTO | null;
  todayScene: TodaySceneDTO | null;
  lastStepResult: LastStepResultDTO | null;
  npcs: ContextNpcDTO[];
  documents: DocumentDTO[];
  phoneMessages: PhoneMessageDTO[];
  letters: LetterNotificationDTO[];
  processes?: ContextProcessDTO[];
  biographyText: string;
  activeLens: NavLens;
  onSelectLens: (lens: NavLens) => void;
  onSubmitIntent: (intentText: string) => void;
  onStructuredAction: (action: StructuredGameplayAction) => Promise<boolean>;
  onAdvanceExplicit?: (actionType: 'HOURS' | 'DAYS' | 'SLEEP' | 'ROUTINE', amount?: number) => void;
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
  letters,
  processes = [],
  biographyText,
  activeLens,
  onSelectLens,
  onSubmitIntent,
  onStructuredAction,
  onAdvanceExplicit,
  isLoading,
  onReturnToMainMenu,
  devMode: _devMode,
  onToggleDevMode: _onToggleDevMode,
}) => {
  const playerAge = livingState?.age || 0;

  // Modals & Drawers State
  const [drawerItem, setDrawerItem] = useState<ContextDrawerItem>(null);
  const [conversationNpc, setConversationNpc] = useState<ContextNpcDTO | null>(null);
  const [isCalendarOpen, setIsCalendarOpen] = useState(false);
  const [isPhoneOpen, setIsPhoneOpen] = useState(false);
  const [isComputerOpen, setIsComputerOpen] = useState(false);
  const [isDocumentsOpen, setIsDocumentsOpen] = useState(false);
  const [isTravelOpen, setIsTravelOpen] = useState(false);
  const [activeDeviceType, setActiveDeviceType] = useState<'phone' | 'computer' | 'wallet' | 'documents' | 'mail' | null>(null);

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
          desc: 'Classrooms where foundational arithmetic, reading, and science are taught.',
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
        {
          id: 'place_travel',
          name: 'Transport Terminal & Travel Desk',
          category: 'Travel & Accommodation',
          desc: 'Compare transport, pay a fare, reserve a stay, receive an itinerary, and move to another city.',
          actions: [
            { id: 'book_travel', title: 'Plan a Journey', desc: 'Choose a city, transport, and length of stay.', intent: 'Open the structured travel planner.' },
          ],
        },
      ];
    }
  };

  const currentPlaces = getPlacesForAge();

  const handleSelectObject = (objName: string) => {
    const lower = objName.toLowerCase();
    if (lower.includes('phone') || lower.includes('mobile') || lower.includes('smartphone')) {
      setIsPhoneOpen(true);
    } else if (lower.includes('computer') || lower.includes('laptop') || lower.includes('pc') || lower.includes('desktop')) {
      setIsComputerOpen(true);
    } else if (lower.includes('certificate') || lower.includes('record') || lower.includes('document')) {
      setIsDocumentsOpen(true);
    } else {
      setDrawerItem({
        type: 'object',
        data: {
          name: objName,
          description: `An environmental object in the room: ${objName}.`,
          possibleActions: [
            `I inspect and examine the ${objName} closely.`,
            `I tidy up and arrange the ${objName} carefully.`,
          ],
        },
      });
    }
  };

  return (
    <div className="flex flex-col h-screen w-screen bg-[#07090e] text-slate-100 overflow-hidden font-sans select-none">
      {/* 1. Persistent World Bar */}
      <PersistentWorldBar
        characterName={livingState?.player_name || 'Living Person'}
        dateTimeFormatted={livingState?.time_formatted || ''}
        locationFormatted={livingState?.location_formatted || 'Living World'}
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
          <SceneWorkspace
            scene={todayScene}
            lastStepResult={lastStepResult}
            presentNpcs={npcs}
            playerAge={playerAge}
            weatherName={livingState?.weather_name || 'Seasonal Weather'}
            onSelectNpc={(npc) => setDrawerItem({ type: 'npc', data: npc })}
            onSelectObject={handleSelectObject}
            onSubmitIntent={onSubmitIntent}
            onOpenDevice={(dev) => {
              if (dev === 'phone') setIsPhoneOpen(true);
              else if (dev === 'computer') setIsComputerOpen(true);
              else if (dev === 'documents') setIsDocumentsOpen(true);
              else setActiveDeviceType(dev);
            }}
            isLoading={isLoading}
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
          <main className="flex-1 overflow-y-auto bg-[#07090e] p-8 max-w-4xl mx-auto space-y-6 select-text">
            <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
              <div>
                <h2 className="text-2xl font-serif font-bold text-slate-100">Places & Horizon</h2>
                <p className="text-xs font-serif italic text-amber-300/80">Click any location to inspect opportunities and actions</p>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {currentPlaces.map((pl) => (
                <button
                  type="button"
                  key={pl.id}
                  onClick={() => {
                    if (pl.id === 'place_travel') setIsTravelOpen(true);
                    else setDrawerItem({ type: 'place', data: pl });
                  }}
                  className="w-full bg-[#0d1017] hover:bg-[#131722] border border-[#1b2234] hover:border-amber-500/50 p-5 rounded-2xl cursor-pointer space-y-2.5 text-left transition-all shadow-sm group"
                >
                  <div className="flex items-center justify-between">
                    <h3 className="font-serif font-bold text-slate-100 text-base group-hover:text-amber-200">{pl.name}</h3>
                    <Send className="w-3.5 h-3.5 text-slate-600 group-hover:text-amber-400 group-hover:translate-x-0.5 transition-all" />
                  </div>
                  <p className="text-[10px] text-amber-300/80 font-mono uppercase">{pl.category}</p>
                  <p className="text-xs text-slate-300 font-serif leading-relaxed">{pl.desc}</p>
                  <div className="pt-2 border-t border-[#1c2234] text-[11px] text-amber-400/80 font-serif italic">
                    Click to visit ({pl.actions.length} actions available)
                  </div>
                </button>
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
          onSendMessage={onSubmitIntent}
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
          npcs={npcs}
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
          currencySymbol={livingState?.currency_symbol || '₦'}
          isLoading={isLoading}
          onClose={() => setIsTravelOpen(false)}
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
