import React from 'react';
import { GraduationCap, Award, BookOpen } from 'lucide-react';

interface EducationViewProps {
  gradeLevel: number;
  academicPerformance: number;
  qualifications: { title: string; field: string; year_obtained: number }[];
  onStudy: () => void;
}

export const EducationView: React.FC<EducationViewProps> = ({
  gradeLevel,
  academicPerformance,
  qualifications = [],
  onStudy,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <GraduationCap size={22} color="var(--accent-indigo)" />
        <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
          Education & Academic Standing
        </h2>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CURRENT GRADE LEVEL</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--text-primary)' }}>
            {gradeLevel > 0 ? `Grade Level ${gradeLevel}` : 'Graduated / Higher Ed'}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>ACADEMIC PERFORMANCE</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: academicPerformance < 50 ? 'var(--accent-crimson)' : 'var(--accent-emerald)' }}>
            {academicPerformance.toFixed(1)}% GPA Standing
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '14px', fontWeight: 600 }}>
          <Award size={16} color="var(--accent-amber)" />
          <span>Earned Qualifications & Diplomas</span>
        </div>

        {qualifications.length === 0 ? (
          <p style={{ fontSize: '13px', color: 'var(--text-muted)' }}>No formal degrees or diplomas earned yet.</p>
        ) : (
          qualifications.map((q, idx) => (
            <div key={idx} style={{ padding: '8px 12px', backgroundColor: 'var(--bg-surface-2)', borderRadius: 'var(--radius-sm)', fontSize: '13px' }}>
              <strong>{q.title}</strong> — {q.field} ({q.year_obtained})
            </div>
          ))
        )}
      </div>

      <button
        onClick={onStudy}
        style={{
          alignSelf: 'flex-start',
          display: 'flex',
          alignItems: 'center',
          gap: '8px',
          backgroundColor: 'var(--accent-indigo)',
          color: '#FFF',
          border: 'none',
          borderRadius: 'var(--radius-md)',
          padding: '10px 18px',
          fontSize: '13px',
          fontWeight: 600,
          cursor: 'pointer',
        }}
      >
        <BookOpen size={16} />
        <span>Study & Review Academic Concepts</span>
      </button>
    </div>
  );
};
