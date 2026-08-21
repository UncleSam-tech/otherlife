import React from 'react';
import { CloudSun, Sun, CloudRain, Wind, Sparkles, MapPin, Calendar, Menu } from 'lucide-react';

interface LifeHeaderProps {
  playerName: string;
  age: number;
  lifeStage: string;
  timeFormatted: string;
  locationFormatted: string;
  weatherName?: string;
  currencySymbol?: string;
  cash?: number;
  onReturnToMainMenu: () => void;
  devMode: boolean;
  onToggleDevMode: () => void;
}

export const LifeHeader: React.FC<LifeHeaderProps> = ({
  playerName,
  age,
  lifeStage,
  timeFormatted,
  locationFormatted,
  weatherName = 'Harmattan Haze',
  currencySymbol = '₦',
  cash = 0,
  onReturnToMainMenu,
  devMode,
  onToggleDevMode,
}) => {
  const getWeatherIcon = (w: string) => {
    const lower = w.toLowerCase();
    if (lower.includes('rain') || lower.includes('monsoon')) return <CloudRain className="w-3.5 h-3.5 text-blue-400" />;
    if (lower.includes('harmattan') || lower.includes('dust') || lower.includes('wind')) return <Wind className="w-3.5 h-3.5 text-amber-400/90" />;
    if (lower.includes('cloud')) return <CloudSun className="w-3.5 h-3.5 text-slate-300" />;
    return <Sun className="w-3.5 h-3.5 text-amber-400" />;
  };

  return (
    <header className="h-14 bg-[#0a0c12] border-b border-[#1c2130] px-6 flex items-center justify-between select-none z-30 font-sans shadow-md">
      {/* Character Identity & Menu */}
      <div className="flex items-center gap-4">
        <button
          type="button"
          onClick={onReturnToMainMenu}
          className="flex items-center gap-2 text-slate-400 hover:text-amber-300 text-xs font-serif tracking-wider transition-colors px-2.5 py-1 rounded-lg hover:bg-slate-900/60"
          title="Return to Main Menu"
        >
          <Menu className="w-4 h-4 text-amber-400" />
          <span>MENU</span>
        </button>

        <span className="text-slate-800">|</span>

        <div className="flex items-center gap-2.5">
          <span className="text-sm font-serif font-bold text-slate-100 tracking-tight">{playerName}</span>
          <span className="text-[11px] bg-[#141824] text-amber-300/90 px-2.5 py-0.5 rounded-full border border-[#22283a] font-serif">
            Age {age} · {lifeStage}
          </span>
        </div>
      </div>

      {/* Atmospheric Life Indicators */}
      <div className="flex items-center gap-5 text-xs font-sans text-slate-300">
        <div className="flex items-center gap-1.5 text-slate-400">
          <Calendar className="w-3.5 h-3.5 text-amber-400/80" />
          <span>{timeFormatted}</span>
        </div>

        <div className="flex items-center gap-1.5 text-slate-400">
          <MapPin className="w-3.5 h-3.5 text-amber-400/80" />
          <span>{locationFormatted}</span>
        </div>

        <div className="flex items-center gap-1.5 bg-[#121622] px-2.5 py-1 rounded-full border border-[#20273a] text-slate-300 text-[11px]">
          {getWeatherIcon(weatherName)}
          <span>{weatherName}</span>
        </div>

        {cash > 0 && (
          <div className="flex items-center gap-1 text-amber-300 font-serif font-semibold">
            <span>{currencySymbol}{cash.toLocaleString(undefined, { minimumFractionDigits: 0, maximumFractionDigits: 0 })}</span>
          </div>
        )}

        <button
          type="button"
          onClick={onToggleDevMode}
          title="Toggle Causal Inspector"
          className={`flex items-center gap-1 px-2.5 py-1 rounded-lg border text-[11px] font-mono transition-all ${
            devMode
              ? 'bg-amber-500/10 text-amber-300 border-amber-500/30'
              : 'text-slate-500 border-[#1c2130] hover:text-slate-300 hover:border-slate-700'
          }`}
        >
          <Sparkles className="w-3 h-3" />
          <span>Causality</span>
        </button>
      </div>
    </header>
  );
};
