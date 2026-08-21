import React, { useState } from 'react';
import { Compass, Users, MapPin, Mail, BookOpen, Globe, Feather, ChevronRight, ChevronLeft } from 'lucide-react';
import { NavLens } from './LifeNavigation';

interface ExpandableNavigationProps {
  activeLens: NavLens;
  onSelectLens: (lens: NavLens) => void;
  unreadCount?: number;
}

export const ExpandableNavigation: React.FC<ExpandableNavigationProps> = ({
  activeLens,
  onSelectLens,
  unreadCount = 0,
}) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const items = [
    { id: 'life' as NavLens, label: 'Life', icon: <Compass className="w-4 h-4" /> },
    { id: 'people' as NavLens, label: 'People', icon: <Users className="w-4 h-4" /> },
    { id: 'places' as NavLens, label: 'Places', icon: <MapPin className="w-4 h-4" /> },
    { id: 'messages' as NavLens, label: 'Notices', icon: <Mail className="w-4 h-4" />, badge: unreadCount },
    { id: 'journal' as NavLens, label: 'Chronicle', icon: <BookOpen className="w-4 h-4" /> },
    { id: 'biography' as NavLens, label: 'My Story', icon: <Feather className="w-4 h-4" /> },
    { id: 'world' as NavLens, label: 'World', icon: <Globe className="w-4 h-4" /> },
  ];

  return (
    <nav
      aria-label="Life Navigation"
      className={`bg-[#080a0e] border-r border-[#151924] flex flex-col justify-between py-3 select-none transition-all duration-300 z-20 ${
        isExpanded ? 'w-44 px-3' : 'w-14 px-2'
      }`}
    >
      <div className="space-y-4">
        {/* Toggle Button */}
        <div className="flex items-center justify-end px-1">
          <button
            type="button"
            onClick={() => setIsExpanded(!isExpanded)}
            aria-label={isExpanded ? 'Collapse navigation rail' : 'Expand navigation rail'}
            className="p-1 text-slate-500 hover:text-amber-300 rounded-lg hover:bg-[#121622] transition-colors"
          >
            {isExpanded ? <ChevronLeft className="w-3.5 h-3.5" /> : <ChevronRight className="w-3.5 h-3.5" />}
          </button>
        </div>

        {/* Navigation Items */}
        <div className="space-y-1">
          {items.map((item) => {
            const isActive = activeLens === item.id;
            return (
              <button
                key={item.id}
                type="button"
                onClick={() => onSelectLens(item.id)}
                title={item.label}
                aria-label={item.label}
                className={`w-full flex items-center gap-3 px-2.5 py-2 rounded-xl text-xs font-serif transition-all duration-150 focus:outline-none focus:ring-1 focus:ring-amber-500/50 ${
                  isActive
                    ? 'bg-amber-500/15 text-amber-300 border border-amber-500/30 font-semibold'
                    : 'text-slate-400 hover:text-slate-200 hover:bg-[#10141f]'
                }`}
              >
                <span className={isActive ? 'text-amber-400' : 'text-slate-400'}>{item.icon}</span>
                {isExpanded && <span className="truncate">{item.label}</span>}
                {item.badge !== undefined && item.badge > 0 && isExpanded && (
                  <span className="ml-auto bg-amber-500/20 text-amber-300 text-[10px] px-1.5 py-0.2 rounded-full font-mono">
                    {item.badge}
                  </span>
                )}
              </button>
            );
          })}
        </div>
      </div>
    </nav>
  );
};
