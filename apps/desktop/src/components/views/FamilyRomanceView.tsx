import React from 'react';
import { Heart, Users, Sparkles, Baby } from 'lucide-react';

interface FamilyRomanceViewProps {
  maritalStatus: string;
  onDate: () => void;
  onMarry: () => void;
  onDivorce: () => void;
  onHaveChild: () => void;
}

export const FamilyRomanceView: React.FC<FamilyRomanceViewProps> = ({
  maritalStatus,
  onDate,
  onMarry,
  onDivorce,
  onHaveChild,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <Heart size={22} color="var(--accent-crimson)" />
        <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
          Family, Romance & Relationships
        </h2>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
        <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>MARITAL / ROMANTIC STATUS</div>
        <div style={{ fontSize: '20px', fontWeight: 700, color: 'var(--text-primary)' }}>
          {maritalStatus}
        </div>
      </div>

      <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
        <button
          onClick={onDate}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            backgroundColor: 'var(--bg-surface-2)',
            border: '1px solid var(--border-strong)',
            color: 'var(--text-primary)',
            borderRadius: 'var(--radius-md)',
            padding: '10px 18px',
            fontSize: '13px',
            fontWeight: 600,
            cursor: 'pointer',
          }}
        >
          <Heart size={16} color="var(--accent-crimson)" />
          <span>Go on a Date</span>
        </button>

        {maritalStatus !== 'Married' && (
          <button
            onClick={onMarry}
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
            <Sparkles size={16} />
            <span>Marry Partner</span>
          </button>
        )}

        {maritalStatus === 'Married' && (
          <>
            <button
              onClick={onHaveChild}
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
              <Baby size={16} />
              <span>Have a Child</span>
            </button>

            <button
              onClick={onDivorce}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                backgroundColor: 'var(--bg-surface-2)',
                border: '1px solid var(--accent-crimson)',
                color: 'var(--accent-crimson)',
                borderRadius: 'var(--radius-md)',
                padding: '10px 18px',
                fontSize: '13px',
                fontWeight: 600,
                cursor: 'pointer',
              }}
            >
              <Users size={16} />
              <span>Divorce</span>
            </button>
          </>
        )}
      </div>
    </div>
  );
};
