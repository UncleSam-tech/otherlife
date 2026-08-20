import React from 'react';
import { HeartPulse, Stethoscope } from 'lucide-react';

interface HealthViewProps {
  fitness: number;
  stress: number;
  onSeekMedicalTreatment: () => void;
}

export const HealthView: React.FC<HealthViewProps> = ({
  fitness,
  stress,
  onSeekMedicalTreatment,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <HeartPulse size={22} color="var(--accent-crimson)" />
        <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
          Health & Well-being
        </h2>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>PHYSICAL FITNESS</div>
          <div style={{ fontSize: '20px', fontWeight: 700, color: 'var(--accent-emerald)' }}>
            {fitness.toFixed(1)} / 100
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>STRESS HORIZON</div>
          <div style={{ fontSize: '20px', fontWeight: 700, color: stress > 40 ? 'var(--accent-amber)' : 'var(--accent-emerald)' }}>
            {stress.toFixed(1)}%
          </div>
        </div>
      </div>

      <button
        onClick={onSeekMedicalTreatment}
        style={{
          alignSelf: 'flex-start',
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
        <Stethoscope size={16} />
        <span>Seek Medical Care & Rest (£40)</span>
      </button>
    </div>
  );
};
