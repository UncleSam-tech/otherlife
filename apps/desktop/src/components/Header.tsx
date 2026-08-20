import React from 'react';
import { Calendar, User, Wallet, MapPin, Code2, Home } from 'lucide-react';

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
  currencySymbol = '£',
  devMode,
  onToggleDevMode,
  onReturnToMainMenu,
}) => {
  return (
    <header style={{
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'space-between',
      padding: '0 20px',
      height: '52px',
      backgroundColor: 'var(--bg-surface-1)',
      borderBottom: '1px solid var(--border-subtle)',
    }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
        <button
          onClick={onReturnToMainMenu}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            backgroundColor: 'transparent',
            border: 'none',
            color: 'var(--text-primary)',
            fontSize: '15px',
            fontWeight: 800,
            letterSpacing: '0.12em',
            fontFamily: 'var(--font-mono)',
            cursor: 'pointer',
          }}
          title="Return to Main Menu"
        >
          <Home size={16} color="var(--accent-indigo)" />
          <span>OTHERLIFE</span>
        </button>

        <span style={{ color: 'var(--border-strong)' }}>|</span>

        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '13px', color: 'var(--text-secondary)' }}>
          <User size={14} />
          <span style={{ fontWeight: 600, color: 'var(--text-primary)' }}>{playerName}</span>
        </div>
      </div>

      <div style={{ display: 'flex', alignItems: 'center', gap: '20px', fontSize: '13px', fontFamily: 'var(--font-mono)' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', color: 'var(--text-secondary)' }}>
          <Calendar size={14} color="var(--accent-indigo)" />
          <span>{timeFormatted}</span>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', color: 'var(--text-secondary)' }}>
          <span>AGE</span>
          <span style={{ fontWeight: 700, color: 'var(--text-primary)' }}>{age}</span>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', color: 'var(--text-secondary)' }}>
          <Wallet size={14} color="var(--accent-emerald)" />
          <span style={{ fontWeight: 700, color: 'var(--accent-emerald)' }}>{currencySymbol}{cash.toFixed(0)}</span>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', color: 'var(--text-secondary)' }}>
          <MapPin size={14} color="var(--accent-amber)" />
          <span>{location.replace('city:real:', '').toUpperCase()}</span>
        </div>

        <button
          onClick={onToggleDevMode}
          title="Toggle Developer Inspection Mode"
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '4px',
            padding: '4px 8px',
            backgroundColor: devMode ? 'var(--accent-indigo-subtle)' : 'transparent',
            border: `1px solid ${devMode ? 'var(--accent-indigo)' : 'var(--border-subtle)'}`,
            borderRadius: 'var(--radius-sm)',
            color: devMode ? 'var(--accent-indigo)' : 'var(--text-muted)',
            cursor: 'pointer',
            fontSize: '11px',
            fontFamily: 'var(--font-mono)'
          }}
        >
          <Code2 size={12} />
          <span>DEV</span>
        </button>
      </div>
    </header>
  );
};
