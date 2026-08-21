import React from 'react';
import { Bookmark } from 'lucide-react';
import { FeedEvent } from './LifeFeed';

interface LifeChronicleProps {
  events: FeedEvent[];
  playerName: string;
}

export const LifeChronicle: React.FC<LifeChronicleProps> = ({ events, playerName }) => {
  if (events.length === 0) {
    return null;
  }

  return (
    <section style={{
      maxWidth: '860px',
      margin: '0 auto',
      width: '100%',
      padding: '0 24px 40px',
      display: 'flex',
      flexDirection: 'column',
      gap: '16px',
    }}>
      <div style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        borderTop: '1px solid var(--border-subtle)',
        paddingTop: '24px',
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Bookmark size={16} style={{ color: 'var(--accent-indigo)' }} />
          <h3 style={{
            fontSize: '14px',
            fontWeight: 700,
            letterSpacing: '0.08em',
            textTransform: 'uppercase',
            color: 'var(--text-muted)',
            margin: 0,
          }}>
            MEMORIES & MILESTONES OF {playerName.toUpperCase()}
          </h3>
        </div>
        <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
          {events.length} Entries Recorded
        </span>
      </div>

      <div style={{
        display: 'flex',
        flexDirection: 'column',
        gap: '12px',
      }}>
        {events.slice(0, 15).map((ev) => (
          <div
            key={ev.id}
            style={{
              backgroundColor: 'var(--bg-surface-1)',
              border: '1px solid var(--border-subtle)',
              borderRadius: 'var(--radius-md)',
              padding: '16px 20px',
              display: 'flex',
              flexDirection: 'column',
              gap: '6px',
            }}
          >
            <div style={{ fontSize: '11px', color: 'var(--text-muted)', letterSpacing: '0.05em' }}>
              {ev.timestamp}
            </div>
            <div style={{
              fontSize: '15px',
              lineHeight: '1.6',
              color: 'var(--text-primary)',
              fontFamily: 'var(--font-serif)',
            }}>
              {ev.summary}
            </div>
          </div>
        ))}
      </div>
    </section>
  );
};
