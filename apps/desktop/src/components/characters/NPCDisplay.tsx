import React from 'react';

export interface ContextNpcDTO {
  id: string;
  name: string;
  relationship_type: string;
  trust_description: string;
  current_activity: string;
}

interface NPCDisplayProps {
  npc: ContextNpcDTO;
}

export const NPCDisplay: React.FC<NPCDisplayProps> = ({ npc }) => {
  const formatHumanBond = (role: string, name: string) => {
    const roleLower = role.toLowerCase();
    if (roleLower.includes('parent') || roleLower.includes('mother')) {
      const n = name.toLowerCase();
      return n.includes('sarah') || n.includes('fiona') || n.includes('elena') || n.includes('funke') || n.includes('amina') || n.includes('isobel')
        ? 'Your Mother'
        : 'Your Father';
    }
    if (roleLower.includes('teacher')) return 'Teacher & Mentor';
    if (roleLower.includes('coach')) return 'Sports Coach & Scout';
    if (roleLower.includes('friend') || roleLower.includes('classmate')) return 'Friend & Peer';
    return role;
  };

  const getNpcPersonalityExcerpt = (name: string) => {
    const n = name.toLowerCase();
    if (n.includes('sarah') || n.includes('fiona') || n.includes('elena') || n.includes('funke') || n.includes('amina')) {
      return 'Patient, nurturing, and attentive to family wellbeing.';
    }
    if (n.includes('david') || n.includes('callum') || n.includes('marcus') || n.includes('babajide') || n.includes('ibrahim')) {
      return 'Disciplined, principled, and holds high standards.';
    }
    if (n.includes('adewale') || n.includes('macleod') || n.includes('hayes') || n.includes('bello')) {
      return 'Inspirational mentor who encourages intellectual rigor.';
    }
    if (n.includes('ibrahim') || n.includes('gordon') || n.includes('miller') || n.includes('odegbami')) {
      return 'Insists on physical stamina, tactical positioning, and discipline.';
    }
    return 'Lively companion sharing daily adventures.';
  };

  return (
    <div className="bg-[#121622] hover:bg-[#161c2b] border border-[#20273a] hover:border-[#2d3752] rounded-2xl p-4 space-y-2.5 transition-all duration-200 shadow-sm">
      <div className="flex justify-between items-baseline">
        <h4 className="font-serif font-bold text-slate-100 text-sm tracking-tight">{npc.name}</h4>
        <span className="text-xs font-serif italic text-amber-300/90">
          {formatHumanBond(npc.relationship_type, npc.name)}
        </span>
      </div>

      <p className="text-xs text-slate-300 leading-relaxed font-sans">
        <span className="text-slate-500 font-serif">Currently: </span>
        {npc.current_activity}
      </p>

      <div className="pt-2 border-t border-[#1c2234] text-xs text-slate-400 font-serif italic">
        {getNpcPersonalityExcerpt(npc.name)}
      </div>
    </div>
  );
};
