import React from 'react';
import { Clock } from 'lucide-react';

export interface ContextProcessDTO {
  id: string;
  title: string;
  progress_percent: number;
  status: string;
}

interface ProcessTrackerProps {
  processes: ContextProcessDTO[];
}

export const ProcessTracker: React.FC<ProcessTrackerProps> = ({ processes }) => {
  if (!processes || processes.length === 0) return null;

  return (
    <div className="space-y-3 pt-2 border-t border-[#1c2234]">
      <h3 className="text-xs font-serif uppercase tracking-widest text-amber-400/80 flex items-center gap-2">
        <Clock className="w-3.5 h-3.5 text-amber-400" />
        <span>Active Undertakings</span>
      </h3>

      <div className="space-y-2">
        {processes.map((proc) => (
          <div
            key={proc.id}
            className="bg-[#121622] border border-[#20273a] rounded-xl p-3.5 space-y-1.5 shadow-sm"
          >
            <div className="flex items-center justify-between text-xs">
              <span className="font-serif font-medium text-slate-200">{proc.title}</span>
              <span className="text-xs font-mono text-amber-400/90">{Math.round(proc.progress_percent)}%</span>
            </div>
            <div className="w-full bg-[#1c2234] rounded-full h-1 overflow-hidden">
              <div
                className="bg-amber-400 h-full rounded-full transition-all duration-300"
                style={{ width: `${Math.min(100, Math.max(5, proc.progress_percent))}%` }}
              />
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
