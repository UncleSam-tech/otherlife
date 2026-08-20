import React from 'react';
import { Cpu, Zap, Activity, HardDrive, Shield } from 'lucide-react';

interface TranshumanismCyberneticsViewProps {
  implantsCount: number;
  mindUploadsCount: number;
  digitalAvatarName: string;
  substrate: string;
  fidelity: number;
  onInstallImplant: () => void;
  onUploadMind: () => void;
  onUpgradeSubstrate: () => void;
}

export const TranshumanismCyberneticsView: React.FC<TranshumanismCyberneticsViewProps> = ({
  implantsCount,
  mindUploadsCount,
  digitalAvatarName,
  substrate,
  fidelity,
  onInstallImplant,
  onUploadMind,
  onUpgradeSubstrate,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Cpu size={22} color="var(--accent-cyan)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Transhumanism, Cybernetics & AI
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onInstallImplant}
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
            <Cpu size={16} />
            <span>Install Cybernetic Implant</span>
          </button>

          <button
            onClick={onUploadMind}
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
            <HardDrive size={16} />
            <span>Upload Consciousness</span>
          </button>

          <button
            onClick={onUpgradeSubstrate}
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
            <Zap size={16} />
            <span>Upgrade Substrate to Quantum</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CYBERNETIC IMPLANTS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-cyan)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Cpu size={18} />
            <span>{implantsCount} Active Implants</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>DIGITAL AVATAR</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-purple)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <HardDrive size={18} />
            <span>{digitalAvatarName || 'Avatar-Alpha'} ({mindUploadsCount})</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>MIND SUBSTRATE</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Shield size={18} />
            <span>{substrate || 'QUANTUM_CORE'}</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>UPLOAD FIDELITY</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Activity size={18} />
            <span>{fidelity.toFixed(1)}% Synaptic Fidelity</span>
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Cpu size={18} color="var(--accent-cyan)" />
          <span>Bio-Digital Immortality & Synthetic Neural Enhancements</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Neural link implants, bionic prosthetic limbs, and ocular HUD overlays enhance physical and cognitive faculties. Mind uploading transfers brain synaptic connectomes into cloud or quantum core substrates for digital immortality.
        </p>
      </div>
    </div>
  );
};
