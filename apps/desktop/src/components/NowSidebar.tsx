import React from 'react';
import { Calendar } from 'lucide-react';

export interface SidebarStateData {
  commitments: { title: string; description: string; urgency: string }[];
  household_trust: number;
  household_resentment: number;
  active_interest: string;
  primary_skill_name: string;
  primary_skill_value: number;
  life_stage?: string;
  marital_status?: string;
  job_title?: string;
  monthly_salary?: number;
  fitness?: number;
  stress?: number;
}

interface NowSidebarProps {
  sidebarData: SidebarStateData;
  devMode: boolean;
}

export const NowSidebar: React.FC<NowSidebarProps> = ({ sidebarData, devMode }) => {
  const {
    commitments,
    household_trust,
    household_resentment,
    active_interest,
    primary_skill_name,
    primary_skill_value,
  } = sidebarData;

  return (
    <aside style={{
      backgroundColor: 'var(--bg-surface-1)',
      borderLeft: '1px solid var(--border-subtle)',
      padding: '20px 16px',
      display: 'flex',
      flexDirection: 'column',
      gap: '20px',
      overflowY: 'auto',
    }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <h3 style={{ fontSize: '14px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'var(--text-muted)' }}>
          NOW CONTEXT
        </h3>
        <span className="badge badge-emerald">{active_interest}</span>
      </div>

      {/* Dynamic Commitment Widgets */}
      {commitments.map((c, idx) => (
        <div key={idx} style={{
          backgroundColor: 'var(--bg-surface-2)',
          border: '1px solid var(--border-subtle)',
          borderRadius: 'var(--radius-md)',
          padding: '12px 14px',
          display: 'flex',
          flexDirection: 'column',
          gap: '8px'
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '13px', fontWeight: 600, color: c.urgency === 'HIGH' ? 'var(--accent-amber)' : 'var(--text-primary)' }}>
            <Calendar size={16} />
            <span>{c.title}</span>
          </div>
          <p style={{ fontSize: '12px', color: 'var(--text-secondary)', lineHeight: '1.4' }}>
            {c.description}
          </p>
        </div>
      ))}

      {/* Relationship Tension Matrix */}
      <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
        <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)' }}>
          HOUSEHOLD TENSION
        </div>

        <div style={{
          backgroundColor: 'var(--bg-surface-2)',
          borderRadius: 'var(--radius-md)',
          padding: '12px',
          display: 'flex',
          flexDirection: 'column',
          gap: '8px'
        }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '13px' }}>
            <span style={{ fontWeight: 600 }}>Mother / Parent</span>
            <span className={`badge ${household_trust < 0.6 ? 'badge-amber' : 'badge-emerald'}`}>
              {household_trust < 0.6 ? 'Tense / Concerned' : 'Supportive'}
            </span>
          </div>

          <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '11px', color: 'var(--text-muted)' }}>
              <span>Trust Horizon</span>
              {devMode && <span>Trust: {(household_trust * 100).toFixed(0)}% | Resentment: {(household_resentment * 100).toFixed(0)}%</span>}
            </div>
            <div style={{ height: '4px', backgroundColor: 'var(--bg-app)', borderRadius: '2px', overflow: 'hidden' }}>
              <div style={{ height: '100%', width: `${household_trust * 100}%`, backgroundColor: household_trust < 0.6 ? 'var(--accent-amber)' : 'var(--accent-emerald)' }} />
            </div>
          </div>
        </div>
      </div>

      {/* Dynamic Skill Meter */}
      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)' }}>
          PRIMARY ABILITY
        </div>

        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '12px' }}>
            <span style={{ color: 'var(--text-secondary)', textTransform: 'capitalize' }}>{primary_skill_name.replace('_', ' ')}</span>
            <span style={{ fontWeight: 600, color: 'var(--accent-emerald)' }}>
              {primary_skill_value.toFixed(1)} / 100
            </span>
          </div>
        </div>
      </div>
    </aside>
  );
};
