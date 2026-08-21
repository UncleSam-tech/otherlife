import React from 'react';
import { Compass, Users, MapPin, Mail, BookOpen, Globe, Feather } from 'lucide-react';

export type NavLens = 'life' | 'people' | 'places' | 'messages' | 'journal' | 'world' | 'biography';

interface LeftNavProps {
  activeLens: NavLens;
  onSelectLens: (lens: NavLens) => void;
  unreadMessagesCount?: number;
}

export const LeftNav: React.FC<LeftNavProps> = ({
  activeLens,
  onSelectLens,
  unreadMessagesCount = 0,
}) => {
  const navItems: { id: NavLens; label: string; icon: React.ReactNode; badge?: number }[] = [
    { id: 'life', label: 'Living Stage', icon: <Compass className="w-5 h-5" /> },
    { id: 'people', label: 'People & Network', icon: <Users className="w-5 h-5" /> },
    { id: 'places', label: 'Places & Institutions', icon: <MapPin className="w-5 h-5" /> },
    { id: 'messages', label: 'Letters & Notices', icon: <Mail className="w-5 h-5" />, badge: unreadMessagesCount },
    { id: 'journal', label: 'Chronicle & Memory', icon: <BookOpen className="w-5 h-5" /> },
    { id: 'world', label: 'World & Economy', icon: <Globe className="w-5 h-5" /> },
    { id: 'biography', label: 'Life Memoir', icon: <Feather className="w-5 h-5" /> },
  ];

  return (
    <aside className="w-64 bg-slate-900 border-r border-slate-800 flex flex-col justify-between py-6 px-4 select-none">
      <div>
        <div className="flex items-center gap-3 px-3 mb-8">
          <div className="w-9 h-9 rounded-lg bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-center text-emerald-400 font-bold tracking-widest text-sm">
            OL
          </div>
          <div>
            <h1 className="font-serif text-lg font-semibold tracking-wide text-slate-100">OTHERLIFE</h1>
            <p className="text-xs text-slate-500 font-mono">HUMAN SIMULATION</p>
          </div>
        </div>

        <nav className="space-y-1.5">
          {navItems.map((item) => {
            const isActive = activeLens === item.id;
            return (
              <button
                key={item.id}
                onClick={() => onSelectLens(item.id)}
                className={`w-full flex items-center justify-between px-3.5 py-2.5 rounded-lg text-sm font-medium transition-all duration-200 ${
                  isActive
                    ? 'bg-emerald-500/15 text-emerald-400 border border-emerald-500/30 shadow-sm'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
                }`}
              >
                <div className="flex items-center gap-3">
                  <span className={isActive ? 'text-emerald-400' : 'text-slate-400'}>{item.icon}</span>
                  <span>{item.label}</span>
                </div>
                {item.badge !== undefined && item.badge > 0 && (
                  <span className="bg-emerald-500/20 text-emerald-400 border border-emerald-500/30 text-xs px-2 py-0.5 rounded-full font-mono">
                    {item.badge}
                  </span>
                )}
              </button>
            );
          })}
        </nav>
      </div>

      <div className="pt-4 border-t border-slate-800/80 px-3">
        <p className="text-xs text-slate-500">Autonomous Reality Engine</p>
        <p className="text-xs text-slate-600 font-mono">v2.0 · Universal Human Core</p>
      </div>
    </aside>
  );
};
