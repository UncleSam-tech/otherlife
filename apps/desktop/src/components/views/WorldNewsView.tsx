import React from 'react';
import { Newspaper } from 'lucide-react';

export interface NewsItemData {
  id: string;
  timestamp: string;
  headline: string;
  body: string;
  category: string;
}

interface WorldNewsViewProps {
  newsItems: NewsItemData[];
}

export const WorldNewsView: React.FC<WorldNewsViewProps> = ({ newsItems }) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <Newspaper size={22} color="var(--accent-indigo)" />
        <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
          Simulated World & Local News Digest
        </h2>
      </div>

      {newsItems.length === 0 ? (
        <div style={{ padding: '24px', backgroundColor: 'var(--bg-surface-1)', borderRadius: 'var(--radius-md)', textAlign: 'center', color: 'var(--text-muted)' }}>
          No world news updates reported yet.
        </div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
          {newsItems.map((item) => (
            <div
              key={item.id}
              style={{
                backgroundColor: 'var(--bg-surface-1)',
                padding: '16px',
                borderRadius: 'var(--radius-md)',
                border: '1px solid var(--border-subtle)',
                display: 'flex',
                flexDirection: 'column',
                gap: '8px',
              }}
            >
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <span
                  style={{
                    fontSize: '11px',
                    fontWeight: 700,
                    padding: '2px 8px',
                    borderRadius: 'var(--radius-sm)',
                    backgroundColor: item.category === 'CAREER' ? 'var(--accent-emerald)' : 'var(--accent-indigo)',
                    color: '#FFF',
                  }}
                >
                  {item.category}
                </span>
                <span style={{ fontSize: '11px', color: 'var(--text-muted)' }}>{item.timestamp}</span>
              </div>
              <h3 style={{ fontSize: '16px', fontWeight: 700, color: 'var(--text-primary)' }}>
                {item.headline}
              </h3>
              <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
                {item.body}
              </p>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
