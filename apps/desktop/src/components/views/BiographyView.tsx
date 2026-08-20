import React from 'react';
import { BookOpen } from 'lucide-react';

interface BiographyViewProps {
  biographyText: string;
}

export const BiographyView: React.FC<BiographyViewProps> = ({ biographyText }) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <BookOpen size={22} color="var(--accent-amber)" />
        <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
          Lifetime Biography & Story Record
        </h2>
      </div>

      <div
        style={{
          backgroundColor: 'var(--bg-surface-1)',
          padding: '24px',
          borderRadius: 'var(--radius-md)',
          border: '1px solid var(--border-subtle)',
          lineHeight: 1.6,
          fontSize: '14px',
          color: 'var(--text-primary)',
          whiteSpace: 'pre-wrap',
          fontFamily: 'var(--font-sans)',
        }}
      >
        {biographyText}
      </div>
    </div>
  );
};
