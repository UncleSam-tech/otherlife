import React from 'react';
import { Eye, Shield, Key, Zap, Target, Lock } from 'lucide-react';

interface SecretSocietyViewProps {
  societyName: string;
  societyType: string;
  rank: string;
  covertReputation: number;
  membershipsCount: number;
  operationsCount: number;
  onJoinSociety: () => void;
  onPerformRitual: () => void;
  onLaunchOperation: () => void;
  onAdvanceRank: () => void;
}

export const SecretSocietyView: React.FC<SecretSocietyViewProps> = ({
  societyName,
  societyType,
  rank,
  covertReputation,
  membershipsCount,
  operationsCount,
  onJoinSociety,
  onPerformRitual,
  onLaunchOperation,
  onAdvanceRank,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Eye size={22} color="var(--accent-purple)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Secret Societies, Subcultures & Underground
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onJoinSociety}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-purple)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Key size={16} />
            <span>Initiate Society Entry</span>
          </button>

          <button
            onClick={onPerformRitual}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-amber)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Zap size={16} />
            <span>Perform Esoteric Ritual</span>
          </button>

          <button
            onClick={onLaunchOperation}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-indigo)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Target size={16} />
            <span>Covert Intelligence Op</span>
          </button>

          <button
            onClick={onAdvanceRank}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-emerald)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Shield size={16} />
            <span>Advance Member Rank</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>SECRET ORDER</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-purple)' }}>
            {societyName || 'The Sovereign Order'} ({societyType || 'OCCULT_ORDER'})
          </div>
          <div style={{ fontSize: '11px', color: 'var(--text-muted)', marginTop: '2px' }}>
            {membershipsCount} Active Membership
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CURRENT LODGE RANK</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Lock size={18} />
            <span>{rank || 'INITIATE'}</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>COVERT REPUTATION</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)' }}>
            {covertReputation.toFixed(1)} / 100
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>COVERT OPERATIONS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-indigo)' }}>
            {operationsCount} Executed
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Eye size={18} color="var(--accent-purple)" />
          <span>Esoteric Rites & Clandestine Underground Influence</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Membership in secret societies grants access to covert networks, esoteric rituals, underground subculture ciphers, and high-level geopolitical conspiracy influence.
        </p>
      </div>
    </div>
  );
};
