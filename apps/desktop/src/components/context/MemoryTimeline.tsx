import React from 'react';
import { BookOpen } from 'lucide-react';

interface MemoryTimelineProps {
  timeFormatted: string;
  headline?: string;
  narrative?: string;
}

export const MemoryTimeline: React.FC<MemoryTimelineProps> = ({
  timeFormatted,
  headline,
  narrative,
}) => {
  return (
    <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
      <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
        <BookOpen className="w-6 h-6 text-amber-400" />
        <div>
          <h2 className="text-2xl font-serif font-bold text-slate-100">Life Chronicle</h2>
          <p className="text-xs font-serif italic text-amber-300/80">Defining memories and turning points recorded in time</p>
        </div>
      </div>
      <div className="space-y-6">
        <div className="relative pl-6 border-l-2 border-amber-500/80 space-y-2">
          <div className="absolute -left-[9px] top-1.5 w-4 h-4 rounded-full bg-[#0a0c12] border-2 border-amber-400 flex items-center justify-center">
            <div className="w-1.5 h-1.5 rounded-full bg-amber-400" />
          </div>
          <span className="text-xs font-mono text-amber-400">{timeFormatted}</span>
          <h4 className="font-serif font-bold text-slate-100 text-lg">{headline || 'A New Life Begins'}</h4>
          <p className="text-slate-300 font-serif text-base leading-relaxed italic">{narrative}</p>
        </div>
      </div>
    </main>
  );
};
