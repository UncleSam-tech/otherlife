import React from 'react';
import { Shield, Award, Crosshair, Flag, CheckCircle2 } from 'lucide-react';

interface MilitaryWarViewProps {
  branch: string;
  rank: string;
  yearsServed: number;
  combatDeployments: number;
  isActiveDuty: boolean;
  isVeteran: boolean;
  pensionMonthly: number;
  onEnlist: (branch: string) => void;
  onPromoteRank: () => void;
  onDeployCombat: () => void;
  onDischargeVeteran: () => void;
}

export const MilitaryWarView: React.FC<MilitaryWarViewProps> = ({
  branch,
  rank,
  yearsServed,
  combatDeployments,
  isActiveDuty,
  isVeteran,
  pensionMonthly,
  onEnlist,
  onPromoteRank,
  onDeployCombat,
  onDischargeVeteran,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Shield size={22} color="var(--accent-amber)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            War, Military & Geopolitics
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          {!isActiveDuty && !isVeteran && (
            <button
              onClick={() => onEnlist('ARMY')}
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
              <Flag size={16} />
              <span>Enlist in Army</span>
            </button>
          )}

          {isActiveDuty && (
            <>
              <button
                onClick={onPromoteRank}
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
                <Award size={16} />
                <span>Request Rank Promotion</span>
              </button>

              <button
                onClick={onDeployCombat}
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
                <Crosshair size={16} />
                <span>Deploy to Combat</span>
              </button>

              <button
                onClick={onDischargeVeteran}
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
                <CheckCircle2 size={16} />
                <span>Honorable Discharge</span>
              </button>
            </>
          )}
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>ACTIVE BRANCH</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)' }}>
            {branch || 'Civilian'}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>MILITARY RANK</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-indigo)' }}>
            {rank || 'None'}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>COMBAT DEPLOYMENTS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-rose)' }}>
            {combatDeployments} Missions
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>SERVICE STATUS / PENSION</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)' }}>
            {isVeteran ? `Veteran (£${pensionMonthly}/mo)` : isActiveDuty ? `Active Duty (${yearsServed} yrs)` : 'Civilian'}
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Shield size={18} color="var(--accent-amber)" />
          <span>Military Command & Geopolitical Readiness</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Character military service builds combat readiness, leadership commendations, officer promotions, and lifelong veteran pensions upon honorable discharge.
        </p>
      </div>
    </div>
  );
};
