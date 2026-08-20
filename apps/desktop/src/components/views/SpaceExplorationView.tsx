import React from 'react';
import { Rocket, Satellite, Globe, Award, Shield } from 'lucide-react';

interface SpaceExplorationViewProps {
  agencyName: string;
  agencyType: string;
  missionsCount: number;
  satellitesCount: number;
  patentsCount: number;
  reputation: number;
  onFundAgency: () => void;
  onLaunchMission: () => void;
  onDeploySatellite: () => void;
  onRegisterPatent: () => void;
}

export const SpaceExplorationView: React.FC<SpaceExplorationViewProps> = ({
  agencyName,
  agencyType,
  missionsCount,
  satellitesCount,
  patentsCount,
  reputation,
  onFundAgency,
  onLaunchMission,
  onDeploySatellite,
  onRegisterPatent,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Rocket size={22} color="var(--accent-blue)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Space Exploration & Off-Earth Expansion
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onFundAgency}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-blue)',
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
            <span>Fund Aerospace Venture</span>
          </button>

          <button
            onClick={onLaunchMission}
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
            <Rocket size={16} />
            <span>Launch Planetary Mission</span>
          </button>

          <button
            onClick={onDeploySatellite}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-cyan)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Satellite size={16} />
            <span>Deploy Orbital Satellite</span>
          </button>

          <button
            onClick={onRegisterPatent}
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
            <Award size={16} />
            <span>Register Space Tech Patent</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>AEROSPACE AGENCY</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-blue)' }}>
            {agencyName || 'Orion Aerospace'} ({agencyType || 'PRIVATE'})
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>SPACE MISSIONS LAUNCHED</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-purple)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Rocket size={18} />
            <span>{missionsCount} Missions</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>ORBITAL SATELLITES</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-cyan)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Satellite size={18} />
            <span>{satellitesCount} Active</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>SPACE TECH PATENTS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Award size={18} />
            <span>{patentsCount} Registered</span>
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Globe size={18} color="var(--accent-blue)" />
          <span>Off-Earth Mission Launch Control & Agency Status</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Private aerospace ventures enable satellite constellation deployment, lunar surface landers, robotic Martian rovers, deep-space probes, and patentable rocket propulsion research. Industry reputation rating: {reputation.toFixed(1)}/100.
        </p>
      </div>
    </div>
  );
};
