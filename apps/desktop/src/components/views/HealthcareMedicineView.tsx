import React from 'react';
import { HeartPulse, Stethoscope, Syringe, FileText, AlertTriangle, Activity } from 'lucide-react';

interface HealthcareMedicineViewProps {
  fitness: number;
  stress: number;
  conditionsCount: number;
  surgeriesCount: number;
  hasWill: boolean;
  onUndergoSurgery: () => void;
  onDraftWill: () => void;
  onQuarantineCheck: () => void;
}

export const HealthcareMedicineView: React.FC<HealthcareMedicineViewProps> = ({
  fitness,
  stress,
  conditionsCount,
  surgeriesCount,
  hasWill,
  onUndergoSurgery,
  onDraftWill,
  onQuarantineCheck,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <HeartPulse size={22} color="var(--accent-rose)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Healthcare, Medicine & Mortality
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onUndergoSurgery}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-rose)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Syringe size={16} />
            <span>Undergo Surgery</span>
          </button>

          <button
            onClick={onDraftWill}
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
            <FileText size={16} />
            <span>Draft Will & Testament</span>
          </button>

          <button
            onClick={onQuarantineCheck}
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
            <AlertTriangle size={16} />
            <span>Epidemic Advisory</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>PHYSICAL FITNESS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Activity size={18} />
            <span>{fitness.toFixed(1)} / 100</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>STRESS HORIZON</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-rose)' }}>
            {stress.toFixed(1)} / 100
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>ACTIVE DIAGNOSES</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)' }}>
            {conditionsCount} Conditions
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>WILL & SURGERIES</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-indigo)' }}>
            {hasWill ? 'Will Registered' : 'No Will'} ({surgeriesCount} Surgeries)
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Stethoscope size={18} color="var(--accent-rose)" />
          <span>Clinical Medicine, Epidemics & End-of-Life Planning</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Comprehensive medical diagnostic records, surgical procedures, epidemic exposure tracking, and estate distribution wills govern character healthcare and long-term mortality resilience.
        </p>
      </div>
    </div>
  );
};
