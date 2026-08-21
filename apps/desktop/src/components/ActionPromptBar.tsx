import React, { useState } from 'react';
import { Send, Sparkles, Clock } from 'lucide-react';

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
    <div className="space-y-3">
      {/* Suggestions Chips */}
      {suggestions.length > 0 && (
        <div className="flex items-center gap-2 overflow-x-auto pb-1 text-xs select-none">
          <span className="flex items-center gap-1 font-mono text-emerald-400 font-medium shrink-0">
            <Sparkles className="w-3.5 h-3.5" />
            POSSIBILITIES:
          </span>
          <div className="flex items-center gap-2 overflow-x-auto no-scrollbar">
            {suggestions.map((s, idx) => (
              <button
                key={idx}
                type="button"
                onClick={() => setInputText(s)}
                className="bg-slate-900 hover:bg-slate-800 text-slate-300 hover:text-slate-100 border border-slate-700/60 rounded-full px-3 py-1 text-xs whitespace-nowrap transition-colors"
              >
                {s}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Intention Input */}
      <div className="relative flex items-center">
        <input
          type="text"
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="What do you do? (Express your intention, conversation, or habit...)"
          disabled={isLoading}
          className="w-full bg-slate-900/90 border border-slate-700 focus:border-emerald-500 rounded-xl px-4 py-3.5 text-sm text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-1 focus:ring-emerald-500 transition-all font-sans pr-28"
        />

        <button
          type="button"
          onClick={handleSend}
          disabled={!inputText.trim() || isLoading}
          className={`absolute right-2 px-4 py-2 rounded-lg font-medium text-xs flex items-center gap-2 transition-all ${
            inputText.trim() && !isLoading
              ? 'bg-emerald-600 hover:bg-emerald-500 text-white shadow-md cursor-pointer'
              : 'bg-slate-800 text-slate-500 cursor-not-allowed'
          }`}
        >
          {isLoading ? (
            <>
              <Clock className="w-3.5 h-3.5 animate-spin" />
              <span>Living...</span>
            </>
          ) : (
            <>
              <span>Express</span>
              <Send className="w-3.5 h-3.5" />
            </>
          )}
        </button>
      </div>
    </div>
  );
};
