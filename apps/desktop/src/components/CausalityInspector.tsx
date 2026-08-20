import React from 'react';
import { X, GitBranch, ArrowUpRight, ArrowDownRight } from 'lucide-react';
import { FeedEvent } from './LifeFeed';

interface CausalityInspectorProps {
  event: FeedEvent | null;
  onClose: () => void;
}

export const CausalityInspector: React.FC<CausalityInspectorProps> = ({ event, onClose }) => {
  if (!event) return null;

  return (
    <div style={{
      position: 'fixed',
      top: 0,
      left: 0,
      width: '100vw',
      height: '100vh',
      backgroundColor: 'rgba(0, 0, 0, 0.65)',
      backdropFilter: 'blur(4px)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      zIndex: 100,
    }}>
      <div style={{
        backgroundColor: 'var(--bg-surface-1)',
        border: '1px solid var(--border-strong)',
        borderRadius: 'var(--radius-lg)',
        width: '560px',
        maxWidth: '90vw',
        padding: '24px',
        display: 'flex',
        flexDirection: 'column',
        gap: '16px',
        boxShadow: 'var(--shadow-lg)',
      }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <GitBranch size={18} color="var(--accent-indigo)" />
            <h3 style={{ fontSize: '16px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
              Causality Trace ("Why Did This Happen?")
            </h3>
          </div>
          <button
            onClick={onClose}
            style={{
              backgroundColor: 'transparent',
              border: 'none',
              color: 'var(--text-muted)',
              cursor: 'pointer',
            }}
          >
            <X size={18} />
          </button>
        </div>

        <div style={{
          backgroundColor: 'var(--bg-surface-2)',
          padding: '12px 16px',
          borderRadius: 'var(--radius-md)',
          fontSize: '14px',
          color: 'var(--text-primary)',
          fontFamily: 'var(--font-serif)'
        }}>
          "{event.summary}"
        </div>

        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)' }}>
            CONTRIBUTING SIMULATION FACTORS
          </div>

          <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
              padding: '10px 12px',
              backgroundColor: 'var(--bg-app)',
              borderRadius: 'var(--radius-md)',
              fontSize: '13px'
            }}>
              <ArrowUpRight size={16} color="var(--accent-emerald)" />
              <span>{event.causalityNote || 'Simulated trust and skill thresholds evaluated.'}</span>
            </div>

            <div style={{
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
              padding: '10px 12px',
              backgroundColor: 'var(--bg-app)',
              borderRadius: 'var(--radius-md)',
              fontSize: '13px'
            }}>
              <ArrowDownRight size={16} color="var(--accent-amber)" />
              <span>Prior event: Academic performance failure (Math exam score 42%)</span>
            </div>
          </div>
        </div>

        <button
          onClick={onClose}
          style={{
            alignSelf: 'flex-end',
            backgroundColor: 'var(--bg-surface-2)',
            border: '1px solid var(--border-subtle)',
            borderRadius: 'var(--radius-md)',
            padding: '8px 16px',
            color: 'var(--text-primary)',
            fontSize: '13px',
            cursor: 'pointer',
          }}
        >
          Close Inspector
        </button>
      </div>
    </div>
  );
};
