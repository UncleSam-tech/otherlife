import React from 'react';
import { X, GitBranch, ArrowUpRight } from 'lucide-react';

export interface CausalEventInfo {
  id: string;
  headline: string;
  summary: string;
  causalityNote?: string;
  effectsSummary?: string;
}

interface CausalityInspectorProps {
  event: CausalEventInfo | null;
  onClose: () => void;
}

export const CausalityInspector: React.FC<CausalityInspectorProps> = ({ event, onClose }) => {
  return (
    <div className="fixed inset-0 bg-black/70 backdrop-blur-sm flex items-center justify-center z-50 p-4 select-none">
      <div className="bg-slate-900 border border-slate-700 rounded-2xl w-full max-w-lg p-6 space-y-5 shadow-2xl animate-fadeIn">
        <div className="flex items-center justify-between border-b border-slate-800 pb-4">
          <div className="flex items-center gap-2 text-emerald-400">
            <GitBranch className="w-5 h-5" />
            <h3 className="font-semibold text-slate-100 text-base">Causality & Simulation Trace</h3>
          </div>
          <button
            onClick={onClose}
            className="text-slate-400 hover:text-white p-1 rounded-lg hover:bg-slate-800 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="space-y-4">
          <div className="bg-slate-950/80 border border-slate-800 rounded-xl p-4 space-y-2">
            <span className="text-xs font-mono text-emerald-400 uppercase tracking-wider">Causal Engine</span>
            <p className="text-sm text-slate-200 leading-relaxed">
              Every outcome in OTHERLIFE is computed deterministically from character traits, skill practice consistency, family trust, and institutional prerequisites.
            </p>
          </div>

          {event && (
            <div className="bg-slate-950/60 border border-slate-800 rounded-xl p-4 space-y-2">
              <h4 className="text-xs font-mono text-slate-400 uppercase">Inspected Event: {event.headline}</h4>
              <p className="text-sm text-slate-300">{event.summary}</p>
              {event.causalityNote && (
                <div className="pt-2 border-t border-slate-800 text-xs text-emerald-400 font-mono flex items-center gap-1.5">
                  <ArrowUpRight className="w-3.5 h-3.5" />
                  <span>{event.causalityNote}</span>
                </div>
              )}
            </div>
          )}
        </div>

        <div className="flex justify-end pt-2">
          <button
            onClick={onClose}
            className="bg-slate-800 hover:bg-slate-700 text-slate-200 px-4 py-2 rounded-xl text-xs font-medium transition-colors"
          >
            Close Inspector
          </button>
        </div>
      </div>
    </div>
  );
};
