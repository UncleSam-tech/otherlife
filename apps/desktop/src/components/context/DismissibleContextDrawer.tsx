import React from 'react';
import { X, User, MapPin, Package, Clock, ArrowRight } from 'lucide-react';
import { ContextNpcDTO } from '../characters/NPCDisplay';
import { ContextProcessDTO } from './ProcessTracker';
import { PlaceLocationDTO } from '../world/PlaceInteractionModal';

export type ContextDrawerItem =
  | { type: 'npc'; data: ContextNpcDTO }
  | { type: 'place'; data: PlaceLocationDTO }
  | { type: 'process'; data: ContextProcessDTO }
  | { type: 'object'; data: { name: string; description: string; possibleActions: string[] } }
  | null;

interface DismissibleContextDrawerProps {
  item: ContextDrawerItem;
  onClose: () => void;
  onExecuteAction: (intent: string) => void;
  onOpenConversation?: (npc: ContextNpcDTO) => void;
  isLoading: boolean;
}

export const DismissibleContextDrawer: React.FC<DismissibleContextDrawerProps> = ({
  item,
  onClose,
  onExecuteAction,
  onOpenConversation,
  isLoading,
}) => {
  if (!item) return null;

  return (
    <aside
      aria-label="Contextual Details"
      className="fixed inset-y-0 right-0 w-full sm:w-96 bg-[#0c0f16] border-l border-[#1b2234] shadow-2xl z-40 flex flex-col justify-between p-6 animate-slideInRight font-sans select-none"
    >
      <div className="space-y-5">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[#181d2c] pb-4">
          <div className="flex items-center gap-2.5">
            <div className="p-2 rounded-xl bg-[#121622] border border-[#20273a] text-amber-400">
              {item.type === 'npc' && <User className="w-4 h-4" />}
              {item.type === 'place' && <MapPin className="w-4 h-4" />}
              {item.type === 'process' && <Clock className="w-4 h-4" />}
              {item.type === 'object' && <Package className="w-4 h-4" />}
            </div>
            <div>
              <p className="text-[10px] font-mono uppercase tracking-widest text-amber-400/80">
                {item.type === 'npc' && 'Person Profile'}
                {item.type === 'place' && 'Location & Setting'}
                {item.type === 'process' && 'Active Undertaking'}
                {item.type === 'object' && 'Environmental Object'}
              </p>
              <h3 className="font-serif font-bold text-base text-slate-100">
                {item.type === 'npc' && item.data.name}
                {item.type === 'place' && item.data.name}
                {item.type === 'process' && item.data.title}
                {item.type === 'object' && item.data.name}
              </h3>
            </div>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close drawer"
            className="p-1.5 text-slate-400 hover:text-slate-100 rounded-lg hover:bg-slate-800 transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content by Type */}
        {item.type === 'npc' && (
          <div className="space-y-4 text-xs">
            <div className="bg-[#121622] border border-[#20273a] p-3.5 rounded-2xl space-y-1.5">
              <span className="text-[11px] font-mono text-slate-500 uppercase">Relationship & Bond</span>
              <p className="font-serif font-semibold text-amber-300 text-sm">{item.data.relationship_type}</p>
              <p className="text-slate-300 font-serif leading-relaxed">{item.data.trust_description}</p>
            </div>

            <div className="bg-[#121622] border border-[#20273a] p-3.5 rounded-2xl space-y-1">
              <span className="text-[11px] font-mono text-slate-500 uppercase">Current Observation</span>
              <p className="text-slate-200 font-serif leading-relaxed">{item.data.current_activity}</p>
            </div>

            <div className="pt-2 space-y-2">
              <button
                type="button"
                onClick={() => {
                  onOpenConversation?.(item.data);
                  onClose();
                }}
                className="w-full bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 hover:to-amber-400 text-slate-950 font-serif font-bold p-3 rounded-xl flex items-center justify-center gap-2 shadow-md transition-all cursor-pointer"
              >
                <span>Have a Conversation</span>
                <ArrowRight className="w-4 h-4" />
              </button>

              <button
                type="button"
                onClick={() => {
                  onExecuteAction(`I ask ${item.data.name} for personal guidance and life advice.`);
                  onClose();
                }}
                disabled={isLoading}
                className="w-full bg-[#121622] hover:bg-[#181f30] border border-[#20273a] text-slate-200 hover:text-amber-200 font-serif p-2.5 rounded-xl text-left transition-colors"
              >
                Seek Guidance & Life Advice
              </button>
            </div>
          </div>
        )}

        {item.type === 'place' && (
          <div className="space-y-4 text-xs">
            <div className="bg-[#121622] border border-[#20273a] p-3.5 rounded-2xl space-y-1">
              <span className="text-[10px] font-mono text-amber-400 uppercase">{item.data.category}</span>
              <p className="text-slate-200 font-serif leading-relaxed">{item.data.desc}</p>
            </div>

            <div className="space-y-2">
              <span className="text-[11px] font-serif text-slate-400">Available Actions in this Place:</span>
              {item.data.actions.map((act) => (
                <button
                  key={act.id}
                  type="button"
                  onClick={() => {
                    onExecuteAction(act.intent);
                    onClose();
                  }}
                  disabled={isLoading}
                  className="w-full bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-amber-500/40 p-3 rounded-xl text-left space-y-0.5 transition-colors cursor-pointer"
                >
                  <p className="font-serif font-bold text-slate-100 text-xs">{act.title}</p>
                  <p className="text-[11px] text-slate-400 font-sans">{act.desc}</p>
                </button>
              ))}
            </div>
          </div>
        )}

        {item.type === 'process' && (
          <div className="space-y-4 text-xs">
            <div className="bg-[#121622] border border-[#20273a] p-4 rounded-2xl space-y-2">
              <div className="flex justify-between items-center text-xs">
                <span className="text-slate-400 font-mono">Stage Progress</span>
                <span className="font-serif font-bold text-amber-300">
                  Step {item.data.current_step} of {item.data.total_steps}
                </span>
              </div>
              <p className="text-slate-300 font-serif leading-relaxed">
                Status: {item.data.status}. Dedicated follow-up and compliance required.
              </p>
            </div>

            <button
              type="button"
              onClick={() => {
                onExecuteAction(`I dedicate time to advance my ongoing commitment: ${item.data.title}`);
                onClose();
              }}
              disabled={isLoading}
              className="w-full bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 text-slate-950 font-serif font-bold p-3 rounded-xl transition-all"
            >
              Advance Process Work
            </button>
          </div>
        )}

        {item.type === 'object' && (
          <div className="space-y-4 text-xs">
            <div className="bg-[#121622] border border-[#20273a] p-4 rounded-2xl space-y-1">
              <p className="text-slate-200 font-serif leading-relaxed">{item.data.description}</p>
            </div>

            <div className="space-y-2">
              {item.data.possibleActions.map((act, i) => (
                <button
                  key={i}
                  type="button"
                  onClick={() => {
                    onExecuteAction(act);
                    onClose();
                  }}
                  disabled={isLoading}
                  className="w-full bg-[#121622] hover:bg-[#181f30] border border-[#20273a] p-3 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors"
                >
                  {act}
                </button>
              ))}
            </div>
          </div>
        )}
      </div>

      <div className="pt-4 border-t border-[#181d2c] flex justify-end">
        <button
          type="button"
          onClick={onClose}
          className="text-xs text-slate-400 hover:text-slate-200 font-serif px-3 py-1.5"
        >
          Dismiss
        </button>
      </div>
    </aside>
  );
};
