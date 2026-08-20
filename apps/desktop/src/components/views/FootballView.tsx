import React from 'react';
import { Trophy, Activity, Shield } from 'lucide-react';

interface FootballViewProps {
  footballRole?: string;
  clubName?: string;
  weeklyWage?: number;
  currencySymbol?: string;
  ballControl?: number;
  pace?: number;
  stamina?: number;
  onTrain?: () => void;
  onPlayMatch?: () => void;
}

export const FootballView: React.FC<FootballViewProps> = ({
  footballRole = 'None',
  clubName = 'None',
  weeklyWage = 0,
  currencySymbol = '£',
  ballControl = 50,
  pace = 50,
  stamina = 50,
  onTrain,
  onPlayMatch,
}) => {
  const hasActiveRole = footballRole && footballRole !== 'None';
  const hasClub = clubName && clubName !== 'None';

  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <Trophy size={22} color="var(--accent-amber)" />
        <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
          Football Ecosystem & Career Standing
        </h2>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>ROLE</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--text-primary)' }}>
            {hasActiveRole ? footballRole : 'No Active Role'}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CLUB AFFILIATION</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: hasClub ? 'var(--accent-indigo)' : 'var(--text-muted)' }}>
            {hasClub ? clubName : 'Unaffiliated / Grassroots'}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>WEEKLY WAGE</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: weeklyWage > 0 ? 'var(--accent-emerald)' : 'var(--text-muted)' }}>
            {currencySymbol}{weeklyWage.toFixed(0)} / wk
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)' }}>TECHNICAL & PHYSICAL ATTRIBUTES</div>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: '12px' }}>
          <div style={{ backgroundColor: 'var(--bg-surface-2)', padding: '10px 14px', borderRadius: 'var(--radius-sm)' }}>
            <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>BALL CONTROL</div>
            <div style={{ fontSize: '16px', fontWeight: 700, color: 'var(--accent-emerald)' }}>{ballControl.toFixed(0)} / 100</div>
          </div>
          <div style={{ backgroundColor: 'var(--bg-surface-2)', padding: '10px 14px', borderRadius: 'var(--radius-sm)' }}>
            <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>PACE</div>
            <div style={{ fontSize: '16px', fontWeight: 700, color: 'var(--accent-indigo)' }}>{pace.toFixed(0)} / 100</div>
          </div>
          <div style={{ backgroundColor: 'var(--bg-surface-2)', padding: '10px 14px', borderRadius: 'var(--radius-sm)' }}>
            <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>STAMINA</div>
            <div style={{ fontSize: '16px', fontWeight: 700, color: 'var(--accent-amber)' }}>{stamina.toFixed(0)} / 100</div>
          </div>
        </div>
      </div>

      <div style={{ display: 'flex', gap: '12px' }}>
        <button
          onClick={onTrain}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            backgroundColor: 'var(--accent-indigo)',
            color: '#FFF',
            border: 'none',
            borderRadius: 'var(--radius-md)',
            padding: '10px 18px',
            fontSize: '13px',
            fontWeight: 600,
            cursor: 'pointer',
          }}
        >
          <Activity size={16} />
          <span>Attend Casual Training</span>
        </button>

        {hasClub && (
          <button
            onClick={onPlayMatch}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-amber)',
              color: '#000',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 18px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Shield size={16} />
            <span>Simulate Scheduled Match</span>
          </button>
        )}
      </div>
    </div>
  );
};
