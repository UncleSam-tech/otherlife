import React, { Suspense, lazy } from 'react';
import {
  Activity,
  BookOpen,
  BriefcaseBusiness,
  Building2,
  FileText,
  GraduationCap,
  Heart,
  Map,
  Plane,
  Sparkles,
  Users,
} from 'lucide-react';
import { LivingStateDTO } from '../context/ContextPanel';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { ContextProcessDTO } from '../context/ProcessTracker';
import { LastStepResultDTO, TodaySceneDTO } from '../world/SceneRenderer';
import { ChronicleEntryDTO } from '../../types/gameplay';

const ThreeLifeScene = lazy(() => import('../world/ThreeLifeScene').then((module) => ({ default: module.ThreeLifeScene })));

interface LifeChronicleProps {
  state: LivingStateDTO;
  scene: TodaySceneDTO | null;
  lastStepResult: LastStepResultDTO | null;
  entries: ChronicleEntryDTO[];
  npcs: ContextNpcDTO[];
  processes: ContextProcessDTO[];
  isLoading: boolean;
  onAgeUp: () => void;
  onOpenPeople: () => void;
  onOpenPlaces: () => void;
  onOpenBiography: () => void;
  onOpenComputer: () => void;
  onOpenDocuments: () => void;
  onOpenTravel: () => void;
  onOpenUniversity: () => void;
  onOpenPhone: () => void;
  onSelectNpc: (npc: ContextNpcDTO) => void;
}

const StatBar = ({ label, value, tone }: { label: string; value: number; tone: string }) => (
  <div className="space-y-1.5">
    <div className="flex justify-between text-[10px] font-mono uppercase tracking-wider text-slate-400">
      <span>{label}</span><span>{Math.round(value)}%</span>
    </div>
    <div className="h-1.5 overflow-hidden rounded-full bg-slate-800">
      <div className={`h-full rounded-full ${tone}`} style={{ width: `${Math.max(0, Math.min(100, value))}%` }} />
    </div>
  </div>
);

export const LifeChronicle: React.FC<LifeChronicleProps> = ({
  state,
  scene,
  lastStepResult,
  entries,
  npcs,
  processes,
  isLoading,
  onAgeUp,
  onOpenPeople,
  onOpenPlaces,
  onOpenBiography,
  onOpenComputer,
  onOpenDocuments,
  onOpenTravel,
  onOpenUniversity,
  onOpenPhone,
  onSelectNpc,
}) => {
  const money = new Intl.NumberFormat(undefined, {
    style: 'currency',
    currency: state.currency_code,
    maximumFractionDigits: state.currency_code === 'NGN' ? 0 : 2,
  }).format(state.cash);

  const categories = [
    { label: 'Relationships', note: `${state.relationships_count} important people`, Icon: Users, onClick: onOpenPeople },
    { label: 'Occupation', note: state.occupation, Icon: BriefcaseBusiness, onClick: onOpenComputer },
    { label: 'Education', note: state.age < 18 ? 'School and future study' : 'Courses and qualifications', Icon: GraduationCap, onClick: onOpenUniversity },
    { label: 'Activities', note: 'Use the living city', Icon: Sparkles, onClick: onOpenPlaces },
    { label: 'Assets & records', note: `${money} available`, Icon: FileText, onClick: onOpenDocuments },
    { label: 'Travel & emigration', note: 'Trips, stays and residency', Icon: Plane, onClick: onOpenTravel },
    { label: 'Mind & body', note: 'Health, fitness and wellbeing', Icon: Heart, onClick: onOpenPlaces },
    { label: 'Life story', note: 'Review the full path', Icon: BookOpen, onClick: onOpenBiography },
  ];

  return (
    <main className="flex-1 overflow-y-auto bg-[#07090e] p-4 md:p-6 xl:p-8 select-text">
      <div className="mx-auto grid w-full max-w-[1500px] gap-5 xl:grid-cols-[300px_minmax(0,1fr)] 2xl:grid-cols-[320px_minmax(420px,1fr)_minmax(380px,0.85fr)]">
        <aside className="min-w-0 space-y-4">
          <section className="rounded-3xl border border-[#20283a] bg-[#0d111a] p-5 shadow-xl">
            <div className="flex items-start justify-between gap-3">
              <div>
                <p className="text-[10px] font-mono uppercase tracking-[0.2em] text-amber-400">Your life</p>
                <h1 className="mt-2 font-serif text-2xl font-bold text-white">{state.player_name}</h1>
                <p className="mt-1 text-sm text-slate-400">Age {state.age} · {state.life_stage}</p>
              </div>
              <div className="rounded-2xl border border-amber-400/25 bg-amber-400/10 px-3 py-2 text-right">
                <p className="text-[9px] font-mono uppercase text-amber-300">Cash</p>
                <p className="mt-1 text-sm font-bold text-amber-100">{money}</p>
              </div>
            </div>
            <div className="mt-5 space-y-3 border-t border-[#20283a] pt-4">
              <StatBar label="Health" value={state.health_level} tone="bg-emerald-400" />
              <StatBar label="Energy" value={state.energy_level} tone="bg-cyan-400" />
              <StatBar label="Fitness" value={state.fitness} tone="bg-violet-400" />
              <StatBar label="Confidence" value={state.confidence_level} tone="bg-amber-400" />
              <StatBar label="Calm" value={100 - state.stress_level} tone="bg-rose-400" />
            </div>
            <div className="mt-5 rounded-2xl bg-[#121827] p-4">
              <p className="text-[9px] font-mono uppercase tracking-wider text-slate-500">Current reality</p>
              <p className="mt-2 text-sm font-medium text-slate-100">{state.occupation}</p>
              <p className="mt-1 text-xs leading-relaxed text-slate-400">{state.current_place_name}, {state.location_formatted}</p>
            </div>
          </section>

          <section className="rounded-3xl border border-[#20283a] bg-[#0d111a] p-4">
            <p className="px-1 text-[10px] font-mono uppercase tracking-[0.18em] text-slate-500">Life activities</p>
            <div className="mt-3 grid gap-2 sm:grid-cols-2 xl:grid-cols-1">
              {categories.map(({ label, note, Icon, onClick }) => (
                <button key={label} type="button" onClick={onClick} className="group flex items-center gap-3 rounded-2xl border border-transparent bg-[#111725] p-3 text-left transition hover:border-amber-400/35 hover:bg-[#171f30] focus-visible:outline focus-visible:outline-2 focus-visible:outline-amber-400">
                  <span className="rounded-xl bg-slate-900 p-2 text-amber-300 group-hover:text-amber-200"><Icon className="h-4 w-4" /></span>
                  <span className="min-w-0"><span className="block text-xs font-semibold text-slate-100">{label}</span><span className="mt-0.5 block truncate text-[10px] text-slate-500">{note}</span></span>
                </button>
              ))}
            </div>
          </section>
        </aside>

        <section className="min-w-0 space-y-4">
          <div className="rounded-3xl border border-[#20283a] bg-[#0b0f17] p-5 shadow-xl">
            <div className="flex items-end justify-between gap-4 border-b border-[#20283a] pb-4">
              <div>
                <p className="text-[10px] font-mono uppercase tracking-[0.2em] text-cyan-300">Life chronicle</p>
                <h2 className="mt-2 font-serif text-xl font-bold text-white">Every year leaves consequences</h2>
              </div>
              <p className="text-right text-[10px] text-slate-500">{state.time_formatted}<br />{entries.length} recorded moments</p>
            </div>

            {lastStepResult ? (
              <div className={`mt-4 rounded-2xl border p-4 ${lastStepResult.success ? 'border-emerald-400/25 bg-emerald-400/5' : 'border-rose-400/25 bg-rose-400/5'}`}>
                <p className="text-[10px] font-mono uppercase text-slate-400">Latest consequence</p>
                <h3 className="mt-1 font-serif font-bold text-slate-100">{lastStepResult.headline || 'Your choice had consequences'}</h3>
                <p className="mt-2 text-xs leading-relaxed text-slate-300">{lastStepResult.narrative}</p>
              </div>
            ) : null}

            <div className="mt-4 max-h-[620px] space-y-0 overflow-y-auto pr-2">
              {entries.length ? entries.map((entry) => (
                <article key={entry.id} className="relative border-l border-[#29334a] pb-6 pl-6 last:pb-1">
                  <span className={`absolute -left-1.5 top-1 h-3 w-3 rounded-full border-2 border-[#0b0f17] ${entry.success ? 'bg-amber-400' : 'bg-rose-400'}`} />
                  <div className="flex items-center justify-between gap-3">
                    <p className="text-[10px] font-mono uppercase tracking-wider text-amber-300">Age {entry.age} · {entry.event_type.split('_').join(' ')}</p>
                    <p className="text-[9px] text-slate-600">{entry.date}</p>
                  </div>
                  <h3 className="mt-1 font-serif text-base font-bold text-slate-100">{entry.headline}</h3>
                  <p className="mt-1.5 text-xs leading-relaxed text-slate-400">{entry.narrative}</p>
                </article>
              )) : (
                <div className="py-12 text-center"><Activity className="mx-auto h-6 w-6 text-slate-600" /><p className="mt-3 text-sm text-slate-400">Your first chapter is ready to begin.</p></div>
              )}
            </div>
          </div>

          <button type="button" onClick={onAgeUp} disabled={isLoading} className="group flex w-full items-center justify-between rounded-3xl border border-amber-300/40 bg-gradient-to-r from-amber-400 to-orange-400 px-6 py-5 text-left text-slate-950 shadow-[0_18px_45px_rgba(251,191,36,0.16)] transition hover:brightness-105 disabled:cursor-wait disabled:opacity-60 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-amber-300">
            <span><span className="block text-[10px] font-mono font-bold uppercase tracking-[0.18em]">Advance your life</span><span className="mt-1 block font-serif text-xl font-black">{isLoading ? 'Resolving consequences…' : `Turn ${state.age + 1}`}</span></span>
            <span className="rounded-full bg-slate-950/15 px-5 py-3 text-sm font-black transition group-hover:translate-x-1">Age +1 year →</span>
          </button>
        </section>

        <aside className="min-w-0 space-y-4 xl:col-span-2 2xl:col-span-1">
          <div className="overflow-hidden rounded-3xl border border-[#26344a] bg-[#08111d]">
            <Suspense fallback={<div className="flex min-h-[480px] items-center justify-center text-sm text-slate-500">Preparing the current 3D moment…</div>}>
              <ThreeLifeScene
                age={state.age}
                location={state.location_formatted}
                placeId={state.current_place_id}
                weatherName={state.weather_name || 'Seasonal weather'}
                npcs={npcs}
                onOpenPhone={onOpenPhone}
                onOpenComputer={onOpenComputer}
                onOpenDocuments={onOpenDocuments}
                onOpenTravel={onOpenTravel}
                onSelectNpc={onSelectNpc}
              />
            </Suspense>
          </div>

          <section className="rounded-3xl border border-[#20283a] bg-[#0d111a] p-5">
            <div className="flex items-start justify-between gap-3">
              <div><p className="text-[10px] font-mono uppercase tracking-[0.18em] text-cyan-300">Current moment</p><h2 className="mt-2 font-serif text-lg font-bold text-white">{scene?.headline || state.current_place_name}</h2></div>
              <Map className="h-5 w-5 text-cyan-300" />
            </div>
            <p className="mt-3 text-xs leading-relaxed text-slate-400">{scene?.narrative || 'Your surroundings change with your place, age, weather, and decisions.'}</p>
            <button type="button" onClick={onOpenPlaces} className="mt-4 flex w-full items-center justify-center gap-2 rounded-xl border border-cyan-400/25 bg-cyan-400/5 py-3 text-xs font-bold text-cyan-200 hover:bg-cyan-400/10"><Building2 className="h-4 w-4" />Open city map</button>
          </section>

          {processes.length ? (
            <section className="rounded-3xl border border-[#20283a] bg-[#0d111a] p-5">
              <p className="text-[10px] font-mono uppercase tracking-[0.18em] text-violet-300">Ongoing commitments</p>
              <div className="mt-3 space-y-3">{processes.slice(0, 4).map((process) => <div key={process.id}><div className="flex justify-between gap-3 text-xs"><span className="truncate text-slate-200">{process.title}</span><span className="text-slate-500">{process.progress_percent}%</span></div><div className="mt-1.5 h-1 overflow-hidden rounded-full bg-slate-800"><div className="h-full bg-violet-400" style={{ width: `${process.progress_percent}%` }} /></div></div>)}</div>
            </section>
          ) : null}
        </aside>
      </div>
    </main>
  );
};
