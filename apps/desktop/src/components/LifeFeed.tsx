import React from 'react';
import { HelpCircle, ChevronRight } from 'lucide-react';

export interface FeedEvent {
  id: string;
  timestamp: string;
  eventType: string;
  summary: string;
  causalityNote: string;
  success?: boolean;
}

interface LifeFeedProps {
  events: FeedEvent[];
  onInspectCausality: (event: FeedEvent) => void;
  devMode: boolean;
}

export const LifeFeed: React.FC<LifeFeedProps> = ({ events, onInspectCausality, devMode }) => {
  return (
    <main style={{
      backgroundColor: 'var(--bg-app)',
      padding: '24px 32px',
      overflowY: 'auto',
      display: 'flex',
      flexDirection: 'column',
      gap: '16px',
    }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '8px' }}>
        <h2 style={{ fontSize: '18px', fontWeight: 700, fontFamily: 'var(--font-serif)', color: 'var(--text-primary)' }}>
          Life History & Feed
        </h2>
        <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
          {events.length} Recorded Events
        </span>
      </div>

      {events.length === 0 ? (
        <div style={{
          padding: '40px',
          textAlign: 'center',
          backgroundColor: 'var(--bg-surface-1)',
          borderRadius: 'var(--radius-lg)',
          border: '1px border var(--border-subtle)',
          color: 'var(--text-muted)'
        }}>
          No events recorded yet. Type an action below to begin living your alternate life.
        </div>
      ) : (
        events.map((ev) => (
          <article
            key={ev.id}
            style={{
              backgroundColor: 'var(--bg-surface-1)',
              border: '1px solid var(--border-subtle)',
              borderRadius: 'var(--radius-lg)',
              padding: '16px 20px',
              display: 'flex',
              flexDirection: 'column',
              gap: '10px',
              boxShadow: 'var(--shadow-sm)',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <span className={`badge ${ev.eventType === 'DECEIVE' ? 'badge-amber' : 'badge-emerald'}`}>
                  {ev.eventType}
                </span>
                <span style={{ fontSize: '12px', fontFamily: 'var(--font-mono)', color: 'var(--text-muted)' }}>
                  {ev.timestamp}
                </span>
              </div>

              <button
                onClick={() => onInspectCausality(ev)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '4px',
                  backgroundColor: 'transparent',
                  border: 'none',
                  color: 'var(--accent-indigo)',
                  cursor: 'pointer',
                  fontSize: '12px',
                  fontWeight: 600,
                }}
              >
                <HelpCircle size={14} />
                <span>Why did this happen?</span>
                <ChevronRight size={14} />
              </button>
            </div>

            <p style={{
              fontSize: '15px',
              lineHeight: '1.6',
              fontFamily: 'var(--font-serif)',
              color: 'var(--text-primary)'
            }}>
              {ev.summary}
            </p>

            {devMode && (
              <div style={{
                marginTop: '6px',
                padding: '8px 12px',
                backgroundColor: 'var(--bg-surface-2)',
                borderRadius: 'var(--radius-sm)',
                fontSize: '11px',
                fontFamily: 'var(--font-mono)',
                color: 'var(--text-secondary)'
              }}>
                [DEV CAUSALITY]: {ev.causalityNote}
              </div>
            )}
          </article>
        ))
      )}
    </main>
  );
};
