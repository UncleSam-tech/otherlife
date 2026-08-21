import React from 'react';
import { Bookmark } from 'lucide-react';

export interface ChronicleEntry {
  id: string;
  timestamp: string;
  headline: string;
  narrative: string;
}

interface LifeChronicleProps {
  entries: ChronicleEntry[];
  playerName: string;
}

export const LifeChronicle: React.FC<LifeChronicleProps> = ({ entries, playerName }) => {
  if (entries.length === 0) {
    return null;
  }

  return (
    <section className="max-w-3xl mx-auto w-full py-6 space-y-4">
      <div className="flex items-center gap-2 border-b border-slate-800 pb-3">
        <Bookmark className="w-4 h-4 text-emerald-400" />
        <h3 className="text-xs font-mono uppercase tracking-wider text-slate-400">
          Chronicle of {playerName}
        </h3>
      </div>

      <div className="space-y-3">
        {entries.map((entry) => (
          <div key={entry.id} className="border-l-2 border-slate-700 hover:border-emerald-500 pl-4 py-1.5 transition-colors">
            <span className="text-xs font-mono text-emerald-400">{entry.timestamp}</span>
            <h4 className="font-semibold text-slate-200 text-sm">{entry.headline}</h4>
            <p className="text-xs text-slate-400 mt-1 leading-relaxed">{entry.narrative}</p>
          </div>
        ))}
      </div>
    </section>
  );
};
