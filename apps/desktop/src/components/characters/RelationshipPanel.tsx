import React from 'react';
import { Users } from 'lucide-react';
import { NPCDisplay, ContextNpcDTO } from './NPCDisplay';

interface RelationshipPanelProps {
  npcs: ContextNpcDTO[];
}

export const RelationshipPanel: React.FC<RelationshipPanelProps> = ({ npcs }) => {
  return (
    <main className="flex-1 overflow-y-auto bg-[#0a0c12] p-8 max-w-4xl mx-auto space-y-6 select-text">
      <div className="flex items-center gap-3 border-b border-[#1c2130] pb-4">
        <Users className="w-6 h-6 text-amber-400" />
        <div>
          <h2 className="text-2xl font-serif font-bold text-slate-100">People & Bonds</h2>
          <p className="text-xs font-serif italic text-amber-300/80">
            Family, mentors, companions, and those whose lives intersect with yours
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {npcs.map((npc) => (
          <NPCDisplay key={npc.id} npc={npc} />
        ))}
      </div>
    </main>
  );
};
