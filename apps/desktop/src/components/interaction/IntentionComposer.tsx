import React, { useState } from 'react';
import { Sparkles, Clock, Moon, Calendar, Send, Smartphone, Laptop, Wallet, FileText } from 'lucide-react';

interface IntentionComposerProps {
  playerAge: number;
  suggestions: string[];
  onSubmitIntent: (intentText: string) => void;
  onOpenDevice?: (deviceType: 'phone' | 'computer' | 'wallet' | 'documents' | 'mail') => void;
  isLoading: boolean;
}

export const IntentionComposer: React.FC<IntentionComposerProps> = ({
  playerAge,
  suggestions,
  onSubmitIntent,
  onOpenDevice,
  isLoading,
}) => {
  const [intentInput, setIntentInput] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!intentInput.trim() || isLoading) return;
    onSubmitIntent(intentInput.trim());
    setIntentInput('');
  };

  const handleSuggestionClick = (sug: string) => {
    if (isLoading) return;
    onSubmitIntent(sug);
  };

  return (
    <div className="bg-[#0b0e17] border-t border-[#1a2030] p-5 space-y-4 font-sans select-none z-30">
      {/* 1. Contextual Intent Suggestions */}
      {suggestions.length > 0 && (
        <div className="space-y-2">
          <div className="flex items-center gap-1.5 text-[11px] font-serif uppercase tracking-widest text-amber-400/80">
            <Sparkles className="w-3.5 h-3.5 text-amber-400" />
            <span>Immediate World Possibilities</span>
          </div>
          <div className="flex flex-wrap gap-2">
            {suggestions.map((sug, idx) => (
              <button
                key={idx}
                type="button"
                onClick={() => handleSuggestionClick(sug)}
                disabled={isLoading}
                className="bg-[#121624] hover:bg-[#1c2336] active:bg-[#252f48] border border-[#222a40] hover:border-amber-500/40 text-slate-200 hover:text-amber-200 text-xs px-3.5 py-2 rounded-full transition-all duration-200 shadow-sm text-left font-serif leading-snug cursor-pointer disabled:opacity-50"
              >
                {sug}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* 2. Open-Ended Intention Input */}
      <form onSubmit={handleSubmit} className="flex items-center gap-2">
        <div className="relative flex-1">
          <input
            type="text"
            value={intentInput}
            onChange={(e) => setIntentInput(e.target.value)}
            placeholder={
              playerAge < 4
                ? 'What do you want to do? (e.g., Cuddle close to mother, explore toys, take first steps)'
                : playerAge < 13
                ? 'What do you want to do? (e.g., Complete arithmetic homework, practice football, help parents)'
                : playerAge < 18
                ? 'What do you want to do? (e.g., Revise past examination papers, train under coach, ask for allowance)'
                : 'What do you want to do? (e.g., Apply for engineering positions, incorporate a company, travel abroad)'
            }
            disabled={isLoading}
            className="w-full bg-[#121622] border border-[#20273a] focus:border-amber-500/70 rounded-2xl px-4 py-3 text-xs text-slate-100 placeholder:text-slate-500 focus:outline-none transition-colors shadow-inner font-sans"
          />
        </div>
        <button
          type="submit"
          disabled={!intentInput.trim() || isLoading}
          className="bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 hover:to-amber-400 disabled:from-slate-800 disabled:to-slate-800 text-slate-950 font-serif font-bold text-xs px-5 py-3 rounded-2xl flex items-center gap-2 transition-all shadow-md cursor-pointer disabled:cursor-not-allowed"
        >
          <span>Act</span>
          <Send className="w-3.5 h-3.5" />
        </button>
      </form>

      {/* 3. Diegetic Tools & Time Advancement Bar */}
      <div className="flex flex-wrap items-center justify-between gap-3 pt-2 border-t border-[#181d2c] text-xs">
        {/* Diegetic In-World Objects */}
        <div className="flex items-center gap-2">
          {playerAge >= 13 && (
            <button
              type="button"
              onClick={() => onOpenDevice?.('phone')}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] text-slate-300 hover:text-amber-300 transition-colors font-serif text-[11px]"
            >
              <Smartphone className="w-3.5 h-3.5 text-slate-400" />
              <span>Phone</span>
            </button>
          )}

          {playerAge >= 10 && (
            <button
              type="button"
              onClick={() => onOpenDevice?.('computer')}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] text-slate-300 hover:text-amber-300 transition-colors font-serif text-[11px]"
            >
              <Laptop className="w-3.5 h-3.5 text-slate-400" />
              <span>Computer</span>
            </button>
          )}

          {playerAge >= 13 && (
            <button
              type="button"
              onClick={() => onOpenDevice?.('wallet')}
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] text-slate-300 hover:text-amber-300 transition-colors font-serif text-[11px]"
            >
              <Wallet className="w-3.5 h-3.5 text-slate-400" />
              <span>Wallet & Cards</span>
            </button>
          )}

          <button
            type="button"
            onClick={() => onOpenDevice?.('documents')}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] text-slate-300 hover:text-amber-300 transition-colors font-serif text-[11px]"
          >
            <FileText className="w-3.5 h-3.5 text-slate-400" />
            <span>Documents</span>
          </button>
        </div>

        {/* Causal Time Controls */}
        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={() => onSubmitIntent('I spend an hour quietly reading and resting.')}
            disabled={isLoading}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] text-slate-300 hover:text-amber-300 transition-colors font-serif text-[11px]"
          >
            <Clock className="w-3.5 h-3.5 text-slate-400" />
            <span>Wait 1 Hour</span>
          </button>

          <button
            type="button"
            onClick={() => onSubmitIntent('I sleep peacefully through the night and wake up refreshed.')}
            disabled={isLoading}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] text-slate-300 hover:text-amber-300 transition-colors font-serif text-[11px]"
          >
            <Moon className="w-3.5 h-3.5 text-slate-400" />
            <span>Sleep</span>
          </button>

          <button
            type="button"
            onClick={() => onSubmitIntent('I follow my daily routine diligently for the next week.')}
            disabled={isLoading}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] text-slate-300 hover:text-amber-300 transition-colors font-serif text-[11px]"
          >
            <Calendar className="w-3.5 h-3.5 text-slate-400" />
            <span>Follow Routine (1 Week)</span>
          </button>
        </div>
      </div>
    </div>
  );
};
