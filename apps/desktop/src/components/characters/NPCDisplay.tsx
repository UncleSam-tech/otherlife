import React from 'react';
import { MessageCircle } from 'lucide-react';

export interface ContextNpcDTO {
  id: string;
  name: string;
  relationship_type: string;
  trust_description: string;
  current_activity: string;
  location_id: string;
  is_new_acquaintance: boolean;
}

interface NPCDisplayProps {
  npc: ContextNpcDTO;
  onSelectNpc?: (npc: ContextNpcDTO) => void;
}

export const NPCDisplay: React.FC<NPCDisplayProps> = ({ npc, onSelectNpc }) => {
  const formatHumanBond = (role: string, id: string) => {
    const roleLower = role.toLowerCase();
    const idLower = id.toLowerCase();
    
    if (idLower.includes('mother') || roleLower.includes('mother')) {
      return 'Your Mother';
    }
    if (idLower.includes('father') || roleLower.includes('father')) {
      return 'Your Father';
    }
    if (roleLower.includes('teacher') || idLower.includes('teacher')) return 'Teacher & Mentor';
    if (roleLower.includes('coach') || idLower.includes('coach')) return 'Sports Coach & Scout';
    if (roleLower.includes('friend') || roleLower.includes('classmate')) return 'Friend & Peer';
    if (roleLower.includes('partner')) return 'Romantic Partner';
    return role;
  };

  return (
    <div
      onClick={() => onSelectNpc?.(npc)}
      className="bg-[#121622] hover:bg-[#161c2b] border border-[#20273a] hover:border-amber-500/50 rounded-2xl p-4 space-y-2.5 transition-all duration-200 shadow-sm cursor-pointer group"
    >
      <div className="flex justify-between items-baseline">
        <h4 className="font-serif font-bold text-slate-100 text-sm tracking-tight group-hover:text-amber-200 flex items-center gap-1.5">
          <span>{npc.name}</span>
          <MessageCircle className="w-3 h-3 text-slate-500 group-hover:text-amber-400 opacity-0 group-hover:opacity-100 transition-opacity" />
        </h4>
        <span className="text-xs font-serif italic text-amber-300/90">
          {formatHumanBond(npc.relationship_type, npc.id)}
        </span>
      </div>

      <p className="text-xs text-slate-300 leading-relaxed font-sans">
        <span className="text-slate-500 font-serif">Currently: </span>
        {npc.current_activity}
      </p>

      <div className="pt-2 border-t border-[#1c2234] text-[11px] text-amber-400/80 font-serif italic flex justify-between items-center">
        <span>Click to interact</span>
        <span className="text-slate-500">{npc.trust_description}</span>
      </div>
    </div>
  );
};
