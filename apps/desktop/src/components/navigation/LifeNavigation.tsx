import React from 'react';
import { Compass, Users, MapPin, Mail, BookOpen, Globe, Feather } from 'lucide-react';

export type NavLens = 'life' | 'people' | 'places' | 'messages' | 'journal' | 'world' | 'biography';

interface LifeNavigationProps {
  activeLens: NavLens;
  onSelectLens: (lens: NavLens) => void;
  unreadMessagesCount?: number;
}

export const LifeNavigation: React.FC<LifeNavigationProps> = ({
  activeLens,
  onSelectLens,
  unreadMessagesCount = 0,
}) => {
  const navItems: { id: NavLens; label: string; icon: React.ReactNode; badge?: number }[] = [
    { id: 'life', label: 'My Life', icon: <Compass className="w-4 h-4" /> },
    { id: 'people', label: 'People', icon: <Users className="w-4 h-4" /> },
    { id: 'places', label: 'Places', icon: <MapPin className="w-4 h-4" /> },
    { id: 'messages', label: 'Messages', icon: <Mail className="w-4 h-4" />, badge: unreadMessagesCount },
    { id: 'journal', label: 'Chronicle', icon: <BookOpen className="w-4 h-4" /> },
    { id: 'world', label: 'World', icon: <Globe className="w-4 h-4" /> },
    { id: 'biography', label: 'Biography', icon: <Feather className="w-4 h-4" /> },
  ];

  return (
    <aside className="w-56 bg-[#0a0c12] border-r border-[#1c2130] flex flex-col justify-between py-6 px-4 select-none font-sans z-20">
      <div>
        {/* Brand / Logo */}
        <div className="flex items-center gap-3 px-2 mb-8">
          <div className="w-8 h-8 rounded-xl bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400 font-serif font-bold text-sm shadow-sm">
            OL
          </div>
          <div>
            <h1 className="font-serif text-sm font-bold tracking-wider text-slate-100">OTHERLIFE</h1>
            <p className="text-[9px] text-amber-400/80 font-mono tracking-widest uppercase">Human Life Simulator</p>
          </div>
        </div>

        {/* Life Navigation Items */}
        <nav className="space-y-1">
          {navItems.map((item) => {
            const isActive = activeLens === item.id;
            return (
              <button
                key={item.id}
                type="button"
                onClick={() => onSelectLens(item.id)}
                className={`w-full flex items-center justify-between px-3 py-2 rounded-xl text-xs font-serif tracking-wide transition-all duration-200 ${
                  isActive
                    ? 'bg-amber-500/15 text-amber-300 border border-amber-500/40 shadow-sm font-semibold'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-[#121622]'
                }`}
              >
                <div className="flex items-center gap-2.5">
                  <span className={isActive ? 'text-amber-400' : 'text-slate-500'}>{item.icon}</span>
                  <span>{item.label}</span>
                </div>
                {item.badge !== undefined && item.badge > 0 && (
                  <span className="bg-amber-500/20 text-amber-300 border border-amber-500/30 text-[10px] px-2 py-0.5 rounded-full font-mono">
                    {item.badge}
                  </span>
                )}
              </button>
            );
          })}
        </nav>
      </div>

      <div className="pt-4 border-t border-[#1c2130] px-2 space-y-1">
        <p className="text-[11px] font-serif text-slate-400 italic">"Live another human life."</p>
        <p className="text-[10px] text-slate-600 font-mono">v2.0 Living Engine</p>
      </div>
    </aside>
  );
};
