import React from 'react';
import { Clock, ArrowRight, BookOpen } from 'lucide-react';

export interface TodayChoiceDTO {
  id: string;
  label: string;
  consequence_hint?: string | null;
  category: string;
  remaining_days?: number | null;
}

export interface ActiveDeadlineDTO {
  id: string;
  title: string;
  description: string;
  deadline_day_total: number;
  category: string;
}

export interface TodaySceneDTO {
  greeting: string;
  date_formatted: string;
  location_formatted: string;
  age: number;
  headline: string;
  narrative: string;
  circumstances: string[];
  choices: TodayChoiceDTO[];
  pending_deadlines: ActiveDeadlineDTO[];
  life_stage: string;
}

interface TodayViewProps {
  scene: TodaySceneDTO;
  onSelectChoice: (choiceId: string) => void;
  onAdvanceTime: (days: number) => void;
  onOpenJournal: () => void;
  isLoading?: boolean;
}

export const TodayView: React.FC<TodayViewProps> = ({
  scene,
  onSelectChoice,
  onAdvanceTime,
  onOpenJournal,
  isLoading = false,
}) => {
  return (
    <div style={{
      maxWidth: '860px',
      margin: '0 auto',
      width: '100%',
      padding: '32px 24px',
      display: 'flex',
      flexDirection: 'column',
      gap: '28px',
      color: 'var(--text-primary)',
      fontFamily: 'var(--font-sans)',
    }}>
      {/* Literary Date & Context Banner */}
      <header style={{
        display: 'flex',
        flexWrap: 'wrap',
        alignItems: 'center',
        justifyContent: 'space-between',
        borderBottom: '1px solid var(--border-subtle)',
        paddingBottom: '16px',
        gap: '12px',
      }}>
        <div>
          <div style={{
            fontSize: '13px',
            textTransform: 'uppercase',
            letterSpacing: '0.12em',
            color: 'var(--text-muted)',
            fontWeight: 700,
            marginBottom: '4px',
          }}>
            {scene.life_stage} · Age {scene.age}
          </div>
          <div style={{ fontSize: '18px', fontWeight: 600, color: 'var(--text-primary)' }}>
            {scene.location_formatted}
          </div>
          <div style={{ fontSize: '14px', color: 'var(--text-secondary)', marginTop: '2px' }}>
            {scene.date_formatted}
          </div>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <button
            onClick={onOpenJournal}
            className="btn btn-secondary"
            style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '13px', padding: '8px 14px' }}
          >
            <BookOpen size={15} />
            <span>Personal Journal</span>
          </button>
        </div>
      </header>

      {/* Main Living Scene / Today's Passage */}
      <section style={{
        backgroundColor: 'var(--bg-surface-1)',
        border: '1px solid var(--border-subtle)',
        borderRadius: 'var(--radius-lg)',
        padding: '28px 32px',
        display: 'flex',
        flexDirection: 'column',
        gap: '20px',
        boxShadow: '0 4px 20px rgba(0,0,0,0.25)',
      }}>
        <div>
          <span style={{
            fontSize: '11px',
            textTransform: 'uppercase',
            letterSpacing: '0.14em',
            fontWeight: 800,
            color: 'var(--accent-indigo)',
            display: 'block',
            marginBottom: '6px',
          }}>
            TODAY IN YOUR LIFE
          </span>
          <h2 style={{
            fontSize: '24px',
            fontWeight: 700,
            fontFamily: 'var(--font-serif)',
            letterSpacing: '-0.01em',
            margin: 0,
            color: 'var(--text-primary)',
          }}>
            {scene.headline}
          </h2>
        </div>

        <p style={{
          fontSize: '16px',
          lineHeight: '1.75',
          color: 'var(--text-secondary)',
          margin: 0,
          whiteSpace: 'pre-line',
          fontFamily: 'var(--font-serif)',
        }}>
          {scene.narrative}
        </p>

        {/* Ongoing Circumstances Tags */}
        {scene.circumstances.length > 0 && (
          <div style={{
            display: 'flex',
            flexWrap: 'wrap',
            gap: '8px',
            paddingTop: '12px',
            borderTop: '1px solid var(--border-subtle)',
          }}>
            {scene.circumstances.map((circ, idx) => (
              <span
                key={idx}
                style={{
                  fontSize: '12px',
                  backgroundColor: 'var(--bg-surface-2)',
                  color: 'var(--text-secondary)',
                  padding: '4px 10px',
                  borderRadius: 'var(--radius-sm)',
                  border: '1px solid var(--border-subtle)',
                }}
              >
                {circ}
              </span>
            ))}
          </div>
        )}
      </section>

      {/* Interactive Choices ("WHAT DO YOU DO?") */}
      <section style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <h3 style={{
            fontSize: '14px',
            fontWeight: 700,
            letterSpacing: '0.08em',
            textTransform: 'uppercase',
            color: 'var(--text-muted)',
            margin: 0,
          }}>
            WHAT DO YOU DO?
          </h3>

          <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
            <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>Let time pass:</span>
            <button
              onClick={() => onAdvanceTime(1)}
              disabled={isLoading}
              className="btn btn-secondary"
              style={{ fontSize: '12px', padding: '4px 10px' }}
            >
              +1 Day
            </button>
            <button
              onClick={() => onAdvanceTime(7)}
              disabled={isLoading}
              className="btn btn-secondary"
              style={{ fontSize: '12px', padding: '4px 10px' }}
            >
              +1 Week
            </button>
            <button
              onClick={() => onAdvanceTime(30)}
              disabled={isLoading}
              className="btn btn-secondary"
              style={{ fontSize: '12px', padding: '4px 10px' }}
            >
              +1 Month
            </button>
          </div>
        </div>

        <div style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(360px, 1fr))',
          gap: '12px',
        }}>
          {scene.choices.map((choice) => {
            const isOpportunity = choice.category === 'OPPORTUNITY';
            const hasCountdown = typeof choice.remaining_days === 'number';

            return (
              <button
                key={choice.id}
                onClick={() => onSelectChoice(choice.id)}
                disabled={isLoading}
                style={{
                  textAlign: 'left',
                  backgroundColor: 'var(--bg-surface-1)',
                  border: isOpportunity ? '1px solid var(--accent-indigo)' : '1px solid var(--border-subtle)',
                  borderRadius: 'var(--radius-md)',
                  padding: '16px 18px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '6px',
                  cursor: isLoading ? 'not-allowed' : 'pointer',
                  transition: 'all 0.15s ease',
                  position: 'relative',
                }}
                onMouseEnter={(e) => {
                  if (!isLoading) {
                    e.currentTarget.style.backgroundColor = 'var(--bg-surface-2)';
                    e.currentTarget.style.borderColor = 'var(--border-strong)';
                  }
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.backgroundColor = 'var(--bg-surface-1)';
                  e.currentTarget.style.borderColor = isOpportunity ? 'var(--accent-indigo)' : 'var(--border-subtle)';
                }}
              >
                <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between', gap: '8px' }}>
                  <span style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', lineHeight: '1.4' }}>
                    {choice.label}
                  </span>
                  <ArrowRight size={16} style={{ color: 'var(--text-muted)', flexShrink: 0, marginTop: '2px' }} />
                </div>

                {hasCountdown && (
                  <div style={{ display: 'flex', alignItems: 'center', gap: '4px', fontSize: '11px', color: 'var(--accent-amber)', fontWeight: 600 }}>
                    <Clock size={13} />
                    <span>{choice.remaining_days! <= 1 ? 'Expires today!' : `Expires in ${choice.remaining_days} days`}</span>
                  </div>
                )}

                {choice.consequence_hint && (
                  <span style={{ fontSize: '12px', color: 'var(--text-secondary)', lineHeight: '1.4' }}>
                    {choice.consequence_hint}
                  </span>
                )}
              </button>
            );
          })}
        </div>
      </section>
    </div>
  );
};
