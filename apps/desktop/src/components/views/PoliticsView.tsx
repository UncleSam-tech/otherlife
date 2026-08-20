import React from 'react';
import { Landmark, Vote, Flag, Sparkles } from 'lucide-react';

interface PoliticsViewProps {
  partyName?: string;
  officeTitle?: string;
  isCampaigning?: boolean;
  pollingPct?: number;
  onLaunchCampaign?: () => void;
  onHoldRally?: () => void;
}

export const PoliticsView: React.FC<PoliticsViewProps> = ({
  partyName = 'Unaffiliated',
  officeTitle = 'No Active Office',
  isCampaigning = false,
  pollingPct = 0,
  onLaunchCampaign,
  onHoldRally,
}) => {
  const hasParty = partyName && partyName !== 'Unaffiliated' && partyName !== 'None';
  const hasOffice = officeTitle && officeTitle !== 'No Active Office' && officeTitle !== 'None';

  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Landmark size={22} color="var(--accent-indigo)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Political Power & Governance
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          {!isCampaigning ? (
            <button
              onClick={onLaunchCampaign}
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
              <Flag size={16} />
              <span>Launch Election Campaign</span>
            </button>
          ) : (
            <button
              onClick={onHoldRally}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
                backgroundColor: 'var(--accent-amber)',
                color: '#000',
                border: 'none',
                borderRadius: 'var(--radius-md)',
                padding: '10px 16px',
                fontSize: '13px',
                fontWeight: 600,
                cursor: 'pointer',
              }}
            >
              <Sparkles size={16} />
              <span>Host Campaign Rally</span>
            </button>
          )}
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>PARTY AFFILIATION</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: hasParty ? 'var(--accent-indigo)' : 'var(--text-muted)' }}>
            {hasParty ? partyName : 'Unaffiliated'}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>ACTIVE PUBLIC OFFICE</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: hasOffice ? 'var(--accent-emerald)' : 'var(--text-muted)' }}>
            {hasOffice ? officeTitle : 'No Active Office'}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>POLLING STANDING</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: isCampaigning ? 'var(--accent-amber)' : 'var(--text-muted)' }}>
            {isCampaigning ? `${pollingPct.toFixed(1)}%` : 'N/A'}
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Vote size={18} color="var(--accent-indigo)" />
          <span>Legislative Agenda & Policy Track Record</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          {hasOffice
            ? 'Active in legislative council. Sponsor policy proposals to impact economy and governance.'
            : 'No active political office held. Join a political party or launch an independent campaign to enter governance.'}
        </p>
      </div>
    </div>
  );
};
