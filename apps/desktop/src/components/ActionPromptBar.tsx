import React, { useState } from 'react';
import { Send, Zap } from 'lucide-react';

interface ActionPromptBarProps {
  onSubmitAction: (text: string) => void;
  isLoading: boolean;
  suggestions?: string[];
}

export const ActionPromptBar: React.FC<ActionPromptBarProps> = ({ onSubmitAction, isLoading, suggestions = [] }) => {
  const [inputText, setInputText] = useState('');

  const handleSend = () => {
    if (!inputText.trim() || isLoading) return;
    onSubmitAction(inputText.trim());
    setInputText('');
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSend();
    }
  };

  return (
    <footer style={{
      backgroundColor: 'var(--bg-surface-1)',
      borderTop: '1px solid var(--border-subtle)',
      padding: '12px 20px',
      display: 'flex',
      flexDirection: 'column',
      gap: '8px',
    }}>
      {/* Contextual Shortcut Chips */}
      <div style={{ display: 'flex', alignItems: 'center', gap: '8px', overflowX: 'auto', paddingBottom: '4px' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '4px', fontSize: '11px', color: 'var(--text-muted)', fontWeight: 600 }}>
          <Zap size={12} color="var(--accent-amber)" />
          <span>SUGGESTED:</span>
        </div>

        {suggestions.map((s, idx) => (
          <button
            key={idx}
            onClick={() => setInputText(s)}
            style={{
              padding: '4px 10px',
              backgroundColor: 'var(--bg-surface-2)',
              border: '1px solid var(--border-subtle)',
              borderRadius: 'var(--radius-full)',
              color: 'var(--text-secondary)',
              fontSize: '12px',
              cursor: 'pointer',
              whiteSpace: 'nowrap',
              transition: 'var(--transition-fast)',
            }}
          >
            {s.length > 55 ? `${s.slice(0, 55)}...` : s}
          </button>
        ))}
      </div>

      {/* Free Text Input Bar */}
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <input
          type="text"
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="What do you want to do? (Type any action or intent...)"
          disabled={isLoading}
          style={{
            flex: 1,
            backgroundColor: 'var(--bg-app)',
            border: '1px solid var(--border-strong)',
            borderRadius: 'var(--radius-md)',
            padding: '12px 16px',
            color: 'var(--text-primary)',
            fontSize: '14px',
            outline: 'none',
          }}
        />

        <button
          onClick={handleSend}
          disabled={!inputText.trim() || isLoading}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '6px',
            backgroundColor: inputText.trim() && !isLoading ? 'var(--accent-indigo)' : 'var(--bg-surface-2)',
            color: inputText.trim() && !isLoading ? '#FFF' : 'var(--text-muted)',
            border: 'none',
            borderRadius: 'var(--radius-md)',
            padding: '12px 20px',
            fontSize: '13px',
            fontWeight: 600,
            cursor: inputText.trim() && !isLoading ? 'pointer' : 'not-allowed',
            transition: 'var(--transition-fast)',
          }}
        >
          <span>{isLoading ? 'Simulating...' : 'Execute'}</span>
          <Send size={14} />
        </button>
      </div>
    </footer>
  );
};
