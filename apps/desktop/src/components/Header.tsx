import React from 'react';
import { Home, Calendar, Wallet, MapPin, Code2 } from 'lucide-react';

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
    <header className="h-14 bg-slate-900 border-b border-slate-800 px-6 flex items-center justify-between select-none">
      <div className="flex items-center gap-4">
        <button
          onClick={onReturnToMainMenu}
          className="flex items-center gap-2 text-slate-300 hover:text-white text-xs font-mono font-bold tracking-wider transition-colors"
          title="Return to Main Menu"
        >
          <Home className="w-4 h-4 text-emerald-400" />
          <span>MENU</span>
        </button>

        <span className="text-slate-700">|</span>

        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-slate-100">{playerName}</span>
          <span className="text-xs bg-slate-800 text-slate-400 px-2 py-0.5 rounded font-mono">
            AGE {age}
          </span>
        </div>
      </div>

      <div className="flex items-center gap-6 text-xs font-mono">
        <div className="flex items-center gap-1.5 text-slate-400">
          <Calendar className="w-3.5 h-3.5 text-emerald-400" />
          <span>{timeFormatted}</span>
        </div>

        <div className="flex items-center gap-1.5 text-emerald-400 font-semibold">
          <Wallet className="w-3.5 h-3.5" />
          <span>{currencySymbol}{cash.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })}</span>
        </div>

        <div className="flex items-center gap-1.5 text-slate-400">
          <MapPin className="w-3.5 h-3.5 text-amber-400" />
          <span>{location.replace('city:real:', '').toUpperCase()}</span>
        </div>

        <button
          onClick={onToggleDevMode}
          title="Toggle Causal Inspector"
          className={`flex items-center gap-1 px-2.5 py-1 rounded border text-xs transition-colors ${
            devMode
              ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30'
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
