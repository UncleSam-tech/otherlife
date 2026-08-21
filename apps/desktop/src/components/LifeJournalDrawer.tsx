import React, { useState } from 'react';
import { X, GraduationCap, Briefcase, Heart, Award, BookOpen } from 'lucide-react';
import { GameStateDTO } from '../App';
import { SidebarStateData } from './NowSidebar';

interface LifeJournalDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  gameState: GameStateDTO;
  sidebarData?: SidebarStateData | null;
  biography?: string;
}

export const LifeJournalDrawer: React.FC<LifeJournalDrawerProps> = ({
  isOpen,
  onClose,
  gameState,
  sidebarData,
  biography,
}) => {
  const [activeSection, setActiveSection] = useState<'OVERVIEW' | 'EDUCATION' | 'CAREER' | 'RELATIONSHIPS' | 'BIOGRAPHY'>('OVERVIEW');

  if (!isOpen) return null;

  return (
    <div style={{
      position: 'fixed',
      inset: 0,
      backgroundColor: 'rgba(0,0,0,0.65)',
      backdropFilter: 'blur(4px)',
      display: 'flex',
      justifyContent: 'flex-end',
      zIndex: 1000,
    }}>
      <div style={{
        width: '560px',
        maxWidth: '100vw',
        height: '100%',
        backgroundColor: 'var(--bg-surface-1)',
        borderLeft: '1px solid var(--border-subtle)',
        display: 'flex',
        flexDirection: 'column',
        boxShadow: '-8px 0 32px rgba(0,0,0,0.4)',
      }}>
        {/* Drawer Header */}
        <div style={{
          padding: '24px 28px',
          borderBottom: '1px solid var(--border-subtle)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}>
          <div>
            <div style={{ fontSize: '12px', textTransform: 'uppercase', letterSpacing: '0.1em', color: 'var(--text-muted)', fontWeight: 700 }}>
              PERSONAL LIFE JOURNAL
            </div>
            <h2 style={{ fontSize: '20px', fontWeight: 700, margin: '2px 0 0', color: 'var(--text-primary)' }}>
              {gameState.playerName}
            </h2>
          </div>

          <button
            onClick={onClose}
            className="btn btn-secondary"
            style={{ padding: '8px', borderRadius: '50%' }}
          >
            <X size={18} />
          </button>
        </div>

        {/* Section Navigation Tabs */}
        <div style={{
          display: 'flex',
          borderBottom: '1px solid var(--border-subtle)',
          padding: '0 20px',
          backgroundColor: 'var(--bg-surface-2)',
          overflowX: 'auto',
        }}>
          {[
            { id: 'OVERVIEW', label: 'Summary', icon: BookOpen },
            { id: 'EDUCATION', label: 'Education', icon: GraduationCap },
            { id: 'CAREER', label: 'Career', icon: Briefcase },
            { id: 'RELATIONSHIPS', label: 'Family & Ties', icon: Heart },
            { id: 'BIOGRAPHY', label: 'Story', icon: Award },
          ].map((sec) => {
            const Icon = sec.icon;
            const isSelected = activeSection === sec.id;
            return (
              <button
                key={sec.id}
                onClick={() => setActiveSection(sec.id as any)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '12px 14px',
                  background: 'none',
                  border: 'none',
                  borderBottom: isSelected ? '2px solid var(--accent-indigo)' : '2px solid transparent',
                  color: isSelected ? 'var(--text-primary)' : 'var(--text-muted)',
                  fontSize: '13px',
                  fontWeight: isSelected ? 600 : 500,
                  cursor: 'pointer',
                  whiteSpace: 'nowrap',
                }}
              >
                <Icon size={14} />
                <span>{sec.label}</span>
              </button>
            );
          })}
        </div>

        {/* Drawer Content Body */}
        <div style={{
          padding: '28px',
          overflowY: 'auto',
          flex: 1,
          display: 'flex',
          flexDirection: 'column',
          gap: '24px',
        }}>
          {activeSection === 'OVERVIEW' && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
              <div style={{
                backgroundColor: 'var(--bg-surface-2)',
                borderRadius: 'var(--radius-md)',
                padding: '18px 20px',
                display: 'flex',
                flexDirection: 'column',
                gap: '12px',
              }}>
                <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                  LIFE ESSENTIALS
                </div>
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px', fontSize: '13px' }}>
                  <div>
                    <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Age & Stage</span>
                    <span style={{ fontWeight: 600 }}>{gameState.age} years old ({gameState.lifeStage})</span>
                  </div>
                  <div>
                    <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Residence</span>
                    <span style={{ fontWeight: 600 }}>{gameState.location.replace('city:real:', '').replace('_', ' ').toUpperCase()}</span>
                  </div>
                  <div>
                    <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Financial Standing</span>
                    <span style={{ fontWeight: 600 }}>{gameState.cash.toLocaleString()}</span>
                  </div>
                  <div>
                    <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Housing</span>
                    <span style={{ fontWeight: 600 }}>{gameState.housingType}</span>
                  </div>
                </div>
              </div>

              {sidebarData?.primary_skill_name && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '18px 20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '8px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    DEVELOPING TALENT
                  </div>
                  <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', textTransform: 'capitalize' }}>
                    {sidebarData.primary_skill_name.replace('_', ' ')}
                  </div>
                </div>
              )}
            </div>
          )}

          {activeSection === 'EDUCATION' && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <div style={{
                backgroundColor: 'var(--bg-surface-2)',
                borderRadius: 'var(--radius-md)',
                padding: '18px 20px',
                display: 'flex',
                flexDirection: 'column',
                gap: '8px',
              }}>
                <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                  ACADEMIC RECORD
                </div>
                <p style={{ fontSize: '14px', lineHeight: '1.6', color: 'var(--text-secondary)', margin: 0 }}>
                  Academic progress unfolds through your life stages — from primary school examinations and senior secondary qualifications to multi-year university degrees.
                </p>
              </div>
            </div>
          )}

          {activeSection === 'CAREER' && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <div style={{
                backgroundColor: 'var(--bg-surface-2)',
                borderRadius: 'var(--radius-md)',
                padding: '18px 20px',
                display: 'flex',
                flexDirection: 'column',
                gap: '8px',
              }}>
                <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                  OCCUPATION & LIVELIHOOD
                </div>
                <div style={{ fontSize: '15px', fontWeight: 600, color: 'var(--text-primary)' }}>
                  {gameState.jobTitle}
                </div>
                {gameState.monthlySalary > 0 && (
                  <div style={{ fontSize: '13px', color: 'var(--accent-emerald)', fontWeight: 600 }}>
                    Monthly Salary: {gameState.monthlySalary.toLocaleString()}
                  </div>
                )}
              </div>
            </div>
          )}

          {activeSection === 'RELATIONSHIPS' && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <div style={{
                backgroundColor: 'var(--bg-surface-2)',
                borderRadius: 'var(--radius-md)',
                padding: '18px 20px',
                display: 'flex',
                flexDirection: 'column',
                gap: '8px',
              }}>
                <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                  HOUSEHOLD & FAMILY BONDS
                </div>
                <div style={{ fontSize: '14px', color: 'var(--text-secondary)' }}>
                  Family Trust: {sidebarData?.household_trust && sidebarData.household_trust > 0.6 ? 'Supportive & Close' : 'Moderate Tension'}
                </div>
                <div style={{ fontSize: '14px', color: 'var(--text-secondary)' }}>
                  Marital Status: {gameState.maritalStatus}
                </div>
              </div>
            </div>
          )}

          {activeSection === 'BIOGRAPHY' && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <div style={{
                backgroundColor: 'var(--bg-surface-2)',
                borderRadius: 'var(--radius-md)',
                padding: '20px',
                display: 'flex',
                flexDirection: 'column',
                gap: '12px',
              }}>
                <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                  MEMORIES OF THIS LIFE
                </div>
                <p style={{
                  fontSize: '15px',
                  lineHeight: '1.7',
                  fontFamily: 'var(--font-serif)',
                  color: 'var(--text-primary)',
                  margin: 0,
                  whiteSpace: 'pre-line',
                }}>
                  {biography || 'The story of this life is currently being written through the choices and events you experience.'}
                </p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
