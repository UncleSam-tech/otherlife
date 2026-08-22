import React, { Suspense, lazy } from 'react';
import { EnvironmentLayer } from './EnvironmentLayer';
import { WeatherLayer } from './WeatherLayer';
import { TodaySceneDTO, LastStepResultDTO } from './SceneRenderer';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { IntentionComposer } from '../interaction/IntentionComposer';
import { Eye, Users, CheckCircle2, AlertCircle, BriefcaseBusiness, BusFront, Compass, GraduationCap } from 'lucide-react';

const ThreeLifeScene = lazy(() => import('./ThreeLifeScene').then((module) => ({ default: module.ThreeLifeScene })));

interface SceneWorkspaceProps {
  scene: TodaySceneDTO | null;
  lastStepResult: LastStepResultDTO | null;
  presentNpcs: ContextNpcDTO[];
  playerAge: number;
  weatherName: string;
  onSelectNpc: (npc: ContextNpcDTO) => void;
  onSelectObject: (objName: string) => void;
  onSubmitIntent: (intentText: string) => void;
  onOpenDevice: (deviceType: 'phone' | 'computer' | 'wallet' | 'documents' | 'mail') => void;
  onOpenTravel: () => void;
  currentPlaceId: string;
  currentPlaceName: string;
  onOpenUniversity: () => void;
  isLoading: boolean;
}

export const SceneWorkspace: React.FC<SceneWorkspaceProps> = ({
  scene,
  lastStepResult,
  presentNpcs,
  playerAge,
  weatherName,
  onSelectNpc,
  onSelectObject,
  onSubmitIntent,
  onOpenDevice,
  onOpenTravel,
  currentPlaceId,
  currentPlaceName,
  onOpenUniversity,
  isLoading,
}) => {
  if (!scene) {
    return (
      <main className="flex-1 flex items-center justify-center p-8 bg-[#090b10]">
        <div className="flex flex-col items-center gap-3">
          <Compass className="w-8 h-8 text-amber-400/70 animate-spin" />
          <p className="text-slate-400 font-serif italic text-sm">Opening your eyes to the world...</p>
        </div>
      </main>
    );
  }

  const suggestions = scene.subtle_details || [
    playerAge < 4
      ? 'Cuddle close to your mother on the sofa'
      : playerAge < 13
      ? 'Complete arithmetic homework at the desk'
      : 'Study past national examination papers',
    playerAge < 4 ? 'Try to stand and take first steps' : 'Spend time with family and discuss goals',
    'Rest and restore energy peacefully',
  ];

  return (
    <main className="flex-1 overflow-y-auto bg-[#07090e] px-4 md:px-8 lg:px-16 py-6 flex flex-col items-center select-text">
      {/* Maximum Content Width Container for Optimal Reading Comfort (~65-75ch) */}
      <div className="w-full max-w-6xl space-y-6">
        {/* 1. Playable 3D Environment with a lightweight 2D loading fallback */}
        <Suspense
          fallback={(
            <div className="relative w-full rounded-2xl overflow-hidden shadow-xl border border-[#1b2234]">
              <EnvironmentLayer
                lifeStage={scene.life_stage || 'Infancy'}
                age={scene.age || 0}
                locationFormatted={scene.location_formatted || scene.location_name || 'Living World'}
              />
              <WeatherLayer weatherName={weatherName} />
            </div>
          )}
        >
          <ThreeLifeScene
            age={scene.age || 0}
            location={scene.location_formatted || scene.location_name || 'Living World'}
            placeId={currentPlaceId}
            weatherName={weatherName}
            npcs={presentNpcs}
            onOpenPhone={() => onOpenDevice('phone')}
            onOpenComputer={() => onOpenDevice('computer')}
            onOpenDocuments={() => onOpenDevice('documents')}
            onOpenTravel={onOpenTravel}
            onSelectNpc={onSelectNpc}
          />
        </Suspense>

        {currentPlaceId !== 'place:home' ? (
          <section className="flex flex-col gap-3 rounded-2xl border border-cyan-400/20 bg-cyan-400/5 p-4 sm:flex-row sm:items-center sm:justify-between" aria-label="Actions available at this location">
            <div><p className="text-[10px] font-mono uppercase text-cyan-300">You entered this location</p><h3 className="mt-1 font-serif font-bold text-slate-100">{currentPlaceName}</h3><p className="mt-1 text-xs text-slate-400">Only people scheduled here and actions provided by this place are available.</p></div>
            {currentPlaceId === 'place:university' ? <button type="button" onClick={onOpenUniversity} className="flex items-center justify-center gap-2 rounded-xl bg-violet-400 px-4 py-3 text-xs font-bold text-slate-950"><GraduationCap className="h-4 w-4" />Browse programmes and apply</button> : null}
            {currentPlaceId === 'place:office' ? <button type="button" onClick={() => onOpenDevice('computer')} className="flex items-center justify-center gap-2 rounded-xl bg-blue-400 px-4 py-3 text-xs font-bold text-slate-950"><BriefcaseBusiness className="h-4 w-4" />Start work or open workstation</button> : null}
            {currentPlaceId === 'place:civic_center' ? <button type="button" onClick={() => onOpenDevice('computer')} className="flex items-center justify-center gap-2 rounded-xl bg-fuchsia-300 px-4 py-3 text-xs font-bold text-slate-950"><BriefcaseBusiness className="h-4 w-4" />Use registry and immigration services</button> : null}
            {currentPlaceId === 'place:transport_terminal' ? <button type="button" onClick={onOpenTravel} className="flex items-center justify-center gap-2 rounded-xl bg-cyan-400 px-4 py-3 text-xs font-bold text-slate-950"><BusFront className="h-4 w-4" />Plan an intercity journey</button> : null}
          </section>
        ) : null}

        <div className="mx-auto w-full max-w-3xl space-y-6">
        {/* 2. Scene Header & Situation Title */}
        <div className="space-y-1 border-b border-[#161a26] pb-3">
          <div className="flex items-center justify-between text-xs font-serif text-amber-400/90 tracking-wide">
            <span>{scene.location_name || scene.location_formatted || 'Living Room'}</span>
            <span>{scene.weather_name || weatherName}</span>
          </div>
          <h2 className="font-serif font-bold text-xl md:text-2xl text-slate-100 tracking-tight">
            {scene.headline || 'Current Situation'}
          </h2>
        </div>

        {/* 3. Recent Consequence / Episodic Memory Banner (if present) */}
        {lastStepResult && (
          <aside
            aria-label="Recent Consequence"
            className="p-4 rounded-xl bg-[#10141f] border border-amber-500/30 text-xs font-serif space-y-1 shadow-sm"
          >
            <div className="flex items-center gap-1.5 text-amber-400 font-semibold">
              {lastStepResult.success ? <CheckCircle2 className="w-3.5 h-3.5" /> : <AlertCircle className="w-3.5 h-3.5" />}
              <span>Recent Outcome</span>
            </div>
            <p className="text-slate-200 leading-relaxed">{lastStepResult.narrative}</p>
            {lastStepResult.causality_note && (
              <p className="text-[11px] text-amber-300/80 italic pt-1">{lastStepResult.causality_note}</p>
            )}
          </aside>
        )}

        {/* 4. Situation Narrative (Literary, Scannable Prose) */}
        <article className="space-y-4">
          <p className="text-slate-100 font-serif text-base md:text-lg leading-relaxed font-normal">
            {scene.narrative}
          </p>

          {/* Sensory Observations */}
          {((scene.circumstances && scene.circumstances.length > 0) ||
            (scene.environmental_objects && scene.environmental_objects.length > 0)) && (
            <div className="pt-3 border-t border-[#161a26] space-y-2">
              <p className="text-xs font-serif italic text-amber-300/90 flex items-center gap-1.5">
                <Eye className="w-3.5 h-3.5 text-amber-400" />
                <span>You notice:</span>
              </p>
              <div className="flex flex-wrap gap-2">
                {[...(scene.circumstances || []), ...(scene.environmental_objects || [])].map((c, i) => (
                  <button
                    key={i}
                    type="button"
                    onClick={() => onSelectObject(c)}
                    className="text-xs font-sans bg-[#0e121a] hover:bg-[#151c2a] text-slate-300 hover:text-amber-200 px-3 py-1 rounded-full border border-[#1b2234] hover:border-amber-500/40 transition-colors shadow-sm cursor-pointer"
                  >
                    • {c}
                  </button>
                ))}
              </div>
            </div>
          )}
        </article>

        {/* 5. People in the Scene */}
        {presentNpcs.length > 0 && (
          <section aria-label="People Present" className="space-y-2 pt-2">
            <div className="flex items-center gap-1.5 text-xs font-serif text-amber-400/90 tracking-wide">
              <Users className="w-3.5 h-3.5" />
              <span>People Present in the Scene</span>
            </div>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              {presentNpcs.map((npc) => (
                <button
                  type="button"
                  key={npc.id}
                  onClick={() => onSelectNpc(npc)}
                  className="w-full p-3 rounded-xl bg-[#0d1017] hover:bg-[#131722] border border-[#1b2234] hover:border-amber-500/50 cursor-pointer text-left transition-all flex items-center justify-between group shadow-sm"
                >
                  <div className="space-y-0.5">
                    <div className="flex items-baseline gap-2">
                      <span className="font-serif font-bold text-xs text-slate-100 group-hover:text-amber-200">
                        {npc.name}
                      </span>
                      <span className="text-[11px] font-serif text-amber-400/80 italic">{npc.relationship_type}</span>
                    </div>
                    <p className="text-[11px] text-slate-400 font-sans">{npc.current_activity}</p>
                  </div>
                  <span className="text-[10px] font-serif text-amber-300/80 group-hover:text-amber-300 opacity-0 group-hover:opacity-100 transition-opacity">
                    Converse →
                  </span>
                </button>
              ))}
            </div>
          </section>
        )}

        {/* 6. Intention Composer Situated in Scene Flow */}
        <section aria-label="Intention Composer" className="pt-2">
          <IntentionComposer
            playerAge={playerAge}
            suggestions={suggestions}
            onSubmitIntent={onSubmitIntent}
            onOpenDevice={onOpenDevice}
            isLoading={isLoading}
          />
        </section>
        </div>
      </div>
    </main>
  );
};
