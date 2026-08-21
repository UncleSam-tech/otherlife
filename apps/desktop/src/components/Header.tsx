import React from 'react';
import { Home, Calendar, MapPin, Code2 } from 'lucide-react';

interface HeaderProps {
  timeFormatted: string;
  age: number;
  cash: number;
  location: string;
  playerName: string;
  currencySymbol?: string;
  devMode: boolean;
  onToggleDevMode: () => void;
  onReturnToMainMenu: () => void;
}

export const Header: React.FC<HeaderProps> = ({
  timeFormatted,
  age,
  cash,
  location,
  playerName,
  currencySymbol = '₦',
  devMode,
  onToggleDevMode,
  onReturnToMainMenu,
}) => {
  return (
    <header className="h-14 bg-[#0e1118] border-b border-slate-800/80 px-6 flex items-center justify-between select-none font-sans">
      <div className="flex items-center gap-4">
        <button
          onClick={onReturnToMainMenu}
          className="flex items-center gap-2 text-slate-400 hover:text-amber-300 text-xs font-serif tracking-wider transition-colors"
          title="Return to Main Menu"
        >
          <Home className="w-3.5 h-3.5 text-amber-400" />
          <span>MAIN MENU</span>
        </button>

        <span className="text-slate-700">|</span>

        <div className="flex items-center gap-2.5">
          <span className="text-sm font-serif font-bold text-slate-100">{playerName}</span>
          <span className="text-[11px] bg-slate-900 text-amber-300/90 px-2.5 py-0.5 rounded-full border border-slate-800 font-serif">
            Age {age}
          </span>
        </div>
      </div>

      <div className="flex items-center gap-6 text-xs font-sans">
        <div className="flex items-center gap-1.5 text-slate-400">
          <Calendar className="w-3.5 h-3.5 text-amber-400" />
          <span>{timeFormatted}</span>
        </div>

        <div className="flex items-center gap-1.5 text-slate-400">
          <MapPin className="w-3.5 h-3.5 text-amber-400" />
          <span>{location.replace('city:real:', '').replace('_', ' ').toUpperCase()}</span>
        </div>

        <div className="flex items-center gap-1.5 text-amber-300 font-serif font-semibold">
          <span>{currencySymbol}{cash.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })}</span>
        </div>

        <button
          onClick={onToggleDevMode}
          title="Toggle Causal Inspector"
          className={`flex items-center gap-1 px-2.5 py-1 rounded-lg border text-xs font-mono transition-colors ${
            devMode
              ? 'bg-amber-500/10 text-amber-300 border-amber-500/30'
              : 'text-slate-500 border-slate-800 hover:text-slate-300 hover:border-slate-700'
          }`}
        >
          <Code2 className="w-3.5 h-3.5" />
          <span>INSPECT</span>
        </button>
      </div>
    </header>
  );
};
