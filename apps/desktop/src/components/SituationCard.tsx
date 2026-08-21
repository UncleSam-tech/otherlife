import React from 'react';
import { Compass, Clock, ArrowRight, Calendar } from 'lucide-react';

export interface LifeSituationChoiceDTO {
  id: string;
  label: string;
  consequence_hint?: string;
}

export interface LifeSituationDTO {
  id: string;
  category: 'Opportunity' | 'Crisis' | 'Routine' | 'Relationship' | 'Decision' | 'Milestone';
  title: string;
  narrative: string;
  choices: LifeSituationChoiceDTO[];
  min_age: number;
  max_age?: number;
  location_id?: string;
  expires_in_days?: number;
  generated_year: number;
  process_id?: string;
}

interface SituationCardProps {
  situations: LifeSituationDTO[];
  onSelectChoice: (situationId: string, choiceId: string) => void;
  onAdvanceTime: (days: number) => void;
  isLoading: boolean;
}

export const SituationCard: React.FC<SituationCardProps> = ({
  situations,
  onSelectChoice,
  onAdvanceTime,
  isLoading,
}) => {
  const getCategoryBadgeClass = (category: string) => {
    switch (category) {
      case 'Opportunity':
        return 'badge-emerald';
      case 'Crisis':
        return 'badge-rose';
      case 'Decision':
        return 'badge-amber';
      case 'Relationship':
        return 'badge-indigo';
      case 'Milestone':
        return 'badge-emerald';
      default:
        return 'badge-slate';
    }
  };

  return (
    <section style={{
      display: 'flex',
      flexDirection: 'column',
      gap: '16px',
      padding: '24px 32px',
      backgroundColor: 'var(--bg-surface-1)',
      borderTop: '1px solid var(--border-subtle)',
    }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Compass size={18} color="var(--accent-indigo)" />
          <h2 style={{
            fontSize: '15px',
            fontWeight: 700,
            letterSpacing: '0.04em',
            textTransform: 'uppercase',
            color: 'var(--text-primary)',
          }}>
            Current Circumstances & Opportunities
          </h2>
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
          <span style={{ fontSize: '12px', color: 'var(--text-muted)' }}>
            Let time pass:
          </span>
          <button
            onClick={() => onAdvanceTime(7)}
            disabled={isLoading}
            className="btn btn-secondary btn-sm"
            style={{ display: 'flex', alignItems: 'center', gap: '4px', fontSize: '11px', padding: '4px 10px' }}
          >
            <Clock size={12} />
            <span>+1 Week</span>
          </button>
          <button
            onClick={() => onAdvanceTime(30)}
            disabled={isLoading}
            className="btn btn-secondary btn-sm"
            style={{ display: 'flex', alignItems: 'center', gap: '4px', fontSize: '11px', padding: '4px 10px' }}
          >
            <Calendar size={12} />
            <span>+1 Month</span>
          </button>
        </div>
      </div>

      {situations.length === 0 ? (
        <div style={{
          padding: '24px',
          backgroundColor: 'var(--bg-surface-2)',
          borderRadius: 'var(--radius-md)',
          border: '1px solid var(--border-subtle)',
          textAlign: 'center',
          color: 'var(--text-secondary)',
          display: 'flex',
          flexDirection: 'column',
          alignItems: 'center',
          gap: '12px',
        }}>
          <p style={{ fontSize: '14px', margin: 0 }}>
            Routine life continues steadily in your community. No urgent decisions are pending.
          </p>
          <div style={{ display: 'flex', gap: '8px' }}>
            <button
              onClick={() => onAdvanceTime(7)}
              disabled={isLoading}
              className="btn btn-primary btn-sm"
            >
              Advance 1 Week
            </button>
            <button
              onClick={() => onAdvanceTime(30)}
              disabled={isLoading}
              className="btn btn-secondary btn-sm"
            >
              Advance 1 Month
            </button>
          </div>
        </div>
      ) : (
        <div style={{
          display: 'grid',
          gridTemplateColumns: situations.length > 1 ? 'repeat(auto-fit, minmax(340px, 1fr))' : '1fr',
          gap: '16px',
        }}>
          {situations.map((sit) => (
            <div
              key={sit.id}
              style={{
                backgroundColor: 'var(--bg-surface-2)',
                border: '1px solid var(--border-default)',
                borderRadius: 'var(--radius-lg)',
                padding: '20px',
                display: 'flex',
                flexDirection: 'column',
                gap: '14px',
                boxShadow: 'var(--shadow-sm)',
              }}
            >
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <span className={`badge ${getCategoryBadgeClass(sit.category)}`}>
                  {sit.category}
                </span>
                {sit.expires_in_days && (
                  <span style={{
                    fontSize: '11px',
                    color: 'var(--text-muted)',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '4px',
                  }}>
                    <Clock size={12} />
                    Expires in {sit.expires_in_days}d
                  </span>
                )}
              </div>

              <div>
                <h3 style={{
                  fontSize: '16px',
                  fontWeight: 600,
                  color: 'var(--text-primary)',
                  marginBottom: '8px',
                  fontFamily: 'var(--font-serif)',
                }}>
                  {sit.title}
                </h3>
                <p style={{
                  fontSize: '14px',
                  lineHeight: '1.6',
                  color: 'var(--text-secondary)',
                  margin: 0,
                }}>
                  {sit.narrative}
                </p>
              </div>

              <div style={{
                display: 'flex',
                flexDirection: 'column',
                gap: '8px',
                marginTop: 'auto',
                paddingTop: '8px',
              }}>
                {sit.choices.map((choice) => (
                  <button
                    key={choice.id}
                    onClick={() => onSelectChoice(sit.id, choice.id)}
                    disabled={isLoading}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'space-between',
                      padding: '10px 14px',
                      backgroundColor: 'var(--bg-surface-1)',
                      border: '1px solid var(--border-subtle)',
                      borderRadius: 'var(--radius-md)',
                      color: 'var(--text-primary)',
                      fontSize: '13px',
                      fontWeight: 500,
                      cursor: isLoading ? 'not-allowed' : 'pointer',
                      textAlign: 'left',
                      transition: 'all var(--transition-fast)',
                    }}
                    onMouseEnter={(e) => {
                      if (!isLoading) {
                        e.currentTarget.style.borderColor = 'var(--accent-indigo)';
                        e.currentTarget.style.backgroundColor = 'var(--bg-surface-hover)';
                      }
                    }}
                    onMouseLeave={(e) => {
                      if (!isLoading) {
                        e.currentTarget.style.borderColor = 'var(--border-subtle)';
                        e.currentTarget.style.backgroundColor = 'var(--bg-surface-1)';
                      }
                    }}
                  >
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '2px' }}>
                      <span style={{ color: 'var(--text-primary)' }}>{choice.label}</span>
                      {choice.consequence_hint && (
                        <span style={{ fontSize: '11px', color: 'var(--text-muted)' }}>
                          {choice.consequence_hint}
                        </span>
                      )}
                    </div>
                    <ArrowRight size={14} color="var(--accent-indigo)" />
                  </button>
                ))}
              </div>
            </div>
          ))}
        </div>
      )}
    </section>
  );
};
