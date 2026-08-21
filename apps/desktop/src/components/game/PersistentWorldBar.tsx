import React from 'react';
import { Calendar, Smartphone, Mail, FileText, Menu, Sun, CloudSun, CloudRain, Wind } from 'lucide-react';

interface PersistentWorldBarProps {
  characterName: string;
  dateTimeFormatted: string;
  locationFormatted: string;
  weatherName: string;
  unreadNotificationsCount: number;
  playerAge: number;
  onOpenCalendar: () => void;
  onOpenPhone: () => void;
  onOpenMessages: () => void;
  onOpenDocuments: () => void;
  onOpenMenu: () => void;
}

export const PersistentWorldBar: React.FC<PersistentWorldBarProps> = ({
  characterName,
  dateTimeFormatted,
  locationFormatted,
  weatherName,
  unreadNotificationsCount,
  playerAge,
  onOpenCalendar,
  onOpenPhone,
  onOpenMessages,
  onOpenDocuments,
  onOpenMenu,
}) => {
  const getWeatherIcon = (w: string) => {
    const lower = w.toLowerCase();
    if (lower.includes('rain') || lower.includes('monsoon')) return <CloudRain className="w-3.5 h-3.5 text-blue-400" />;
    if (lower.includes('harmattan') || lower.includes('dust') || lower.includes('wind')) return <Wind className="w-3.5 h-3.5 text-amber-400" />;
    if (lower.includes('cloud') || lower.includes('mist')) return <CloudSun className="w-3.5 h-3.5 text-slate-300" />;
    return <Sun className="w-3.5 h-3.5 text-amber-400" />;
  };

  const getInitials = (name: string) => {
    return name
      .split(' ')
      .filter(Boolean)
      .slice(0, 2)
      .map((part) => part[0]?.toUpperCase() || '')
      .join('');
  };

  return (
    <header className="h-12 bg-[#090b10] border-b border-[#161a26] px-4 flex items-center justify-between select-none z-30 font-sans shadow-sm text-slate-200">
      {/* 1. Character Identity & Menu Access */}
      <div className="flex items-center gap-3">
        <button
          type="button"
          onClick={onOpenMenu}
          aria-label="Open main menu"
          className="p-1.5 rounded-lg text-slate-400 hover:text-amber-300 hover:bg-[#121622] transition-colors focus:outline-none focus:ring-2 focus:ring-amber-500/50"
        >
          <Menu className="w-4 h-4" />
        </button>

        <div className="flex items-center gap-2">
          <div className="w-7 h-7 rounded-full bg-gradient-to-br from-amber-500/20 to-amber-700/20 border border-amber-500/40 flex items-center justify-center text-[11px] font-serif font-bold text-amber-300 shadow-inner">
            {getInitials(characterName) || 'P'}
          </div>
          <span className="font-serif font-semibold text-xs tracking-tight text-slate-100">{characterName}</span>
        </div>
      </div>

      {/* 2. Compact Atmospheric Life Indicators */}
      <div className="flex items-center gap-4 text-xs">
        <div className="flex items-center gap-1.5 text-slate-300 font-serif">
          <span>{dateTimeFormatted}</span>
        </div>

        <span className="text-slate-700">·</span>

        <div className="text-slate-300 font-serif">
          <span>{locationFormatted}</span>
        </div>

        <span className="text-slate-700">·</span>

        <div className="flex items-center gap-1.5 bg-[#10141f] px-2 py-0.5 rounded-full border border-[#1b2234] text-[11px] text-slate-300">
          {getWeatherIcon(weatherName)}
          <span>{weatherName}</span>
        </div>
      </div>

      {/* 3. Quick Diegetic Tool Access */}
      <div className="flex items-center gap-1">
        <button
          type="button"
          onClick={onOpenCalendar}
          aria-label="Open Calendar and Schedule"
          className="p-1.5 rounded-lg text-slate-400 hover:text-amber-300 hover:bg-[#121622] transition-colors focus:outline-none focus:ring-2 focus:ring-amber-500/50 relative"
          title="Calendar & Time"
        >
          <Calendar className="w-4 h-4" />
        </button>

        {playerAge >= 13 && (
          <button
            type="button"
            onClick={onOpenPhone}
            aria-label="Open Personal Smartphone"
            className="p-1.5 rounded-lg text-slate-400 hover:text-amber-300 hover:bg-[#121622] transition-colors focus:outline-none focus:ring-2 focus:ring-amber-500/50"
            title="Smartphone"
          >
            <Smartphone className="w-4 h-4" />
          </button>
        )}

        <button
          type="button"
          onClick={onOpenMessages}
          aria-label="Open Messages and Letters"
          className="p-1.5 rounded-lg text-slate-400 hover:text-amber-300 hover:bg-[#121622] transition-colors focus:outline-none focus:ring-2 focus:ring-amber-500/50 relative"
          title="Letters & Notices"
        >
          <Mail className="w-4 h-4" />
          {unreadNotificationsCount > 0 && (
            <span className="absolute top-1 right-1 w-2 h-2 rounded-full bg-amber-500" />
          )}
        </button>

        <button
          type="button"
          onClick={onOpenDocuments}
          aria-label="Open Official Documents and Credentials"
          className="p-1.5 rounded-lg text-slate-400 hover:text-amber-300 hover:bg-[#121622] transition-colors focus:outline-none focus:ring-2 focus:ring-amber-500/50"
          title="Documents & Credentials"
        >
          <FileText className="w-4 h-4" />
        </button>
      </div>
    </header>
  );
};
