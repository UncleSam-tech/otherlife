import React from 'react';
import { Sun, Sparkles, Globe2, Crown, Zap } from 'lucide-react';

interface PostScarcityCosmicLegacyViewProps {
  ubdAmount: number;
  automationIndex: number;
  megastructuresCount: number;
  interstellarColoniesCount: number;
  kardashevTier: string;
  onDistributeUBD: () => void;
  onBuildMegastructure: () => void;
  onEstablishColony: () => void;
  onEvaluateLegacy: () => void;
}

export const PostScarcityCosmicLegacyView: React.FC<PostScarcityCosmicLegacyViewProps> = ({
  ubdAmount,
  automationIndex,
  megastructuresCount,
  interstellarColoniesCount,
  kardashevTier,
  onDistributeUBD,
  onBuildMegastructure,
  onEstablishColony,
  onEvaluateLegacy,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Sun size={22} color="var(--accent-amber)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Post-Scarcity & Cosmic Legacy
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onDistributeUBD}
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
            <Sparkles size={16} />
            <span>Distribute Universal Dividend</span>
          </button>

          <button
            onClick={onBuildMegastructure}
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
            <Sun size={16} />
            <span>Construct Dyson Swarm</span>
          </button>

          <button
            onClick={onEstablishColony}
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
            <Globe2 size={16} />
            <span>Found Interstellar Colony</span>
          </button>

          <button
            onClick={onEvaluateLegacy}
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
            <Crown size={16} />
            <span>Evaluate Cosmic Legacy</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>UNIVERSAL DIVIDEND</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)' }}>
            £{ubdAmount.toLocaleString()}/mo ({automationIndex.toFixed(1)}% Automation)
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>COSMIC MEGASTRUCTURES</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Sun size={18} />
            <span>{megastructuresCount} Dyson Swarms</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>INTERSTELLAR COLONIES</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-blue)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Globe2 size={18} />
            <span>{interstellarColoniesCount} Star Systems</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>KARDASHEV SCALE RATING</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-purple)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Crown size={18} />
            <span>{kardashevTier || 'TYPE_II'}</span>
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Zap size={18} color="var(--accent-amber)" />
          <span>Master Sandbox Legacy & Kardashev Type II Civilization Triumph</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          In this post-scarcity era, automated production ensures high universal basic dividends, stellar Dyson swarms capture entire solar energy outputs, and multi-generational human dynasties span interstellar colonies across distant star systems.
        </p>
      </div>
    </div>
  );
};
