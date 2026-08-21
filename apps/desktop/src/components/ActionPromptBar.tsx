import React, { useState } from 'react';
import { Send, Feather, Compass } from 'lucide-react';

interface ActionPromptBarProps {
  onSubmitIntent: (text: string) => void;
  isLoading: boolean;
  suggestions?: string[];
}

export const ActionPromptBar: React.FC<ActionPromptBarProps> = ({
  onSubmitIntent,
  isLoading,
  suggestions = [],
}) => {
  const [inputText, setInputText] = useState('');

  const handleSend = () => {
    if (!inputText.trim() || isLoading) return;
    onSubmitIntent(inputText.trim());
    setInputText('');
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSend();
    }
  };

  return (
    <div className="space-y-4">
      {/* Subtle Inspirations & Environmental Cues */}
      {suggestions.length > 0 && (
        <div className="space-y-1.5 select-none">
          <p className="text-xs font-serif italic text-amber-300/80 flex items-center gap-1.5">
            <Feather className="w-3.5 h-3.5 text-amber-400" />
            <span>You consider:</span>
          </p>
          <div className="flex items-center gap-2 overflow-x-auto no-scrollbar pb-1">
            {suggestions.map((s, idx) => (
              <button
                key={idx}
                type="button"
                onClick={() => setInputText(s)}
                className="bg-slate-900/90 hover:bg-amber-950/30 text-slate-300 hover:text-amber-200 border border-slate-800 hover:border-amber-500/40 rounded-full px-3.5 py-1 text-xs whitespace-nowrap transition-all duration-200 shadow-sm"
              >
                {s}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Intention Input Bar */}
      <div className="relative flex items-center">
        <input
          type="text"
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="What do you do? (Express your action, words, habit, or decision...)"
          disabled={isLoading}
          className="w-full bg-slate-900/90 border border-slate-800 focus:border-amber-500/70 focus:bg-slate-900 rounded-2xl px-5 py-4 text-sm text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-1 focus:ring-amber-500/40 transition-all font-serif pr-32 shadow-inner"
        />

        <button
          type="button"
          onClick={handleSend}
          disabled={!inputText.trim() || isLoading}
          className={`absolute right-2 px-5 py-2.5 rounded-xl font-serif text-xs flex items-center gap-2 transition-all duration-200 ${
            inputText.trim() && !isLoading
              ? 'bg-amber-500 hover:bg-amber-400 text-slate-950 font-bold shadow-lg shadow-amber-500/20 cursor-pointer scale-100 hover:scale-105'
              : 'bg-slate-800/80 text-slate-500 cursor-not-allowed'
          }`}
        >
          {isLoading ? (
            <>
              <Compass className="w-3.5 h-3.5 animate-spin" />
              <span>Living...</span>
            </>
          ) : (
            <>
              <span>Act</span>
              <Send className="w-3.5 h-3.5" />
            </>
          )}
        </button>
      </div>
    </div>
  );
};
