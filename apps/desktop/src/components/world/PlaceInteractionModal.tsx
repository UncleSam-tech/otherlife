import React from 'react';
import { BookOpen, Activity, Building, Briefcase, DollarSign, Home, X, Send } from 'lucide-react';

export interface PlaceLocationDTO {
  id: string;
  name: string;
  category: string;
  desc: string;
  actions: { id: string; title: string; desc: string; intent: string }[];
}

interface PlaceInteractionModalProps {
  place: PlaceLocationDTO | null;
  onClose: () => void;
  onExecuteAction: (intentText: string) => void;
  isLoading: boolean;
}

export const PlaceInteractionModal: React.FC<PlaceInteractionModalProps> = ({
  place,
  onClose,
  onExecuteAction,
  isLoading,
}) => {
  if (!place) return null;

  const getIcon = (cat: string) => {
    const c = cat.toLowerCase();
    if (c.includes('school') || c.includes('education')) return <BookOpen className="w-5 h-5 text-blue-400" />;
    if (c.includes('sports') || c.includes('pitch')) return <Activity className="w-5 h-5 text-orange-400" />;
    if (c.includes('business') || c.includes('work')) return <Briefcase className="w-5 h-5 text-cyan-400" />;
    if (c.includes('bank') || c.includes('finance')) return <DollarSign className="w-5 h-5 text-emerald-400" />;
    if (c.includes('home')) return <Home className="w-5 h-5 text-amber-400" />;
    return <Building className="w-5 h-5 text-indigo-400" />;
  };

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      <div className="bg-[#0e1118] border border-amber-500/30 rounded-3xl max-w-lg w-full p-6 space-y-5 shadow-2xl animate-fadeIn">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-2xl bg-[#141824] border border-[#22283a]">
              {getIcon(place.category)}
            </div>
            <div>
              <h3 className="font-serif font-bold text-lg text-slate-100">{place.name}</h3>
              <p className="text-xs text-amber-300/80 font-mono uppercase">{place.category}</p>
            </div>
          </div>
          <button
            type="button"
            onClick={onClose}
            className="p-1.5 text-slate-400 hover:text-slate-100 rounded-xl hover:bg-slate-800 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <p className="text-xs text-slate-300 font-serif leading-relaxed">
          {place.desc}
        </p>

        {/* Location Actions */}
        <div className="space-y-2 max-h-80 overflow-y-auto pr-1">
          {place.actions.map((act) => (
            <button
              key={act.id}
              type="button"
              onClick={() => {
                onExecuteAction(act.intent);
                onClose();
              }}
              disabled={isLoading}
              className="w-full bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] hover:border-amber-500/50 p-3.5 rounded-2xl text-left transition-all duration-200 flex items-center justify-between group shadow-sm"
            >
              <div>
                <div className="font-serif font-bold text-xs text-slate-100 group-hover:text-amber-200">
                  {act.title}
                </div>
                <div className="text-[11px] text-slate-400 font-sans mt-0.5">
                  {act.desc}
                </div>
              </div>
              <Send className="w-3.5 h-3.5 text-slate-600 group-hover:text-amber-400 group-hover:translate-x-0.5 transition-all flex-shrink-0" />
            </button>
          ))}
        </div>

        <div className="pt-2 border-t border-[#1c2130] flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="text-xs text-slate-400 hover:text-slate-200 font-serif px-4 py-2"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
