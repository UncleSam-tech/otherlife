import React, { useState } from 'react';
import { X, Lock, Home, GraduationCap, Trophy, Users, Smartphone, Heart, Briefcase, Compass, Landmark } from 'lucide-react';
import { GameStateDTO } from '../App';
import { SidebarStateData } from './NowSidebar';

export interface LifeDomainsDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  gameState: GameStateDTO;
  sidebarData?: SidebarStateData | null;
}

interface DomainDefinition {
  id: string;
  name: string;
  minAge: number;
  icon: React.ElementType;
  description: string;
  category: string;
}

const DOMAINS: DomainDefinition[] = [
  { id: 'family', name: 'Family & Household', minAge: 0, icon: Home, description: 'Parents, upbringing, family harmony, and household support.', category: 'CORE' },
  { id: 'education', name: 'School & Academics', minAge: 4, icon: GraduationCap, description: 'Classroom curriculum, examinations, grades, and university degrees.', category: 'LEARNING' },
  { id: 'talents', name: 'Talents & Hobbies', minAge: 6, icon: Trophy, description: 'Athletics, sports control, artistic creativity, coding, and discipline.', category: 'PASSIONS' },
  { id: 'social', name: 'Friends & Peer Circles', minAge: 7, icon: Users, description: 'Schoolmates, childhood companions, and social reputation.', category: 'COMMUNITY' },
  { id: 'digital', name: 'Digital & Social Media', minAge: 13, icon: Smartphone, description: 'Online platforms, content channels, audience growth, and creators.', category: 'MEDIA' },
  { id: 'romance', name: 'Romance & Relationships', minAge: 15, icon: Heart, description: 'Adolescent crushes, dating, partnerships, and lifelong marriage.', category: 'PERSONAL' },
  { id: 'career', name: 'Career & Enterprise', minAge: 18, icon: Briefcase, description: 'Professions, employment, startups, commercial ventures, and wages.', category: 'LIVELIHOOD' },
  { id: 'faith', name: 'Faith & Principles', minAge: 13, icon: Compass, description: 'Spiritual life, ethical values, and moral philosophy.', category: 'SPIRIT' },
  { id: 'civic', name: 'Civic & Politics', minAge: 18, icon: Landmark, description: 'Community town halls, voting, public advocacy, and leadership.', category: 'SOCIETY' },
];

export const LifeDomainsDrawer: React.FC<LifeDomainsDrawerProps> = ({
  isOpen,
  onClose,
  gameState,
  sidebarData,
}) => {
  const [selectedDomainId, setSelectedDomainId] = useState<string>('family');

  if (!isOpen) return null;

  const currentAge = gameState.age;
  const activeDomain = DOMAINS.find((d) => d.id === selectedDomainId) || DOMAINS[0];
  const isSelectedLocked = currentAge < activeDomain.minAge;

  const currSymbol = gameState.location.includes('nigeria') ? '₦' : '£';

  return (
    <div style={{
      position: 'fixed',
      inset: 0,
      backgroundColor: 'rgba(0,0,0,0.7)',
      backdropFilter: 'blur(6px)',
      display: 'flex',
      justifyContent: 'flex-end',
      zIndex: 1100,
    }}>
      <div style={{
        width: '680px',
        maxWidth: '100vw',
        height: '100%',
        backgroundColor: 'var(--bg-surface-1)',
        borderLeft: '1px solid var(--border-subtle)',
        display: 'flex',
        flexDirection: 'column',
        boxShadow: '-10px 0 40px rgba(0,0,0,0.5)',
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
            <div style={{ fontSize: '11px', textTransform: 'uppercase', letterSpacing: '0.14em', color: 'var(--accent-indigo)', fontWeight: 800 }}>
              DIMENSIONS OF LIFE
            </div>
            <h2 style={{ fontSize: '20px', fontWeight: 700, margin: '2px 0 0', color: 'var(--text-primary)' }}>
              {gameState.playerName} · Age {currentAge}
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

        {/* Horizontal Domain Selector with Age Gates */}
        <div style={{
          display: 'flex',
          gap: '8px',
          padding: '12px 20px',
          backgroundColor: 'var(--bg-surface-2)',
          borderBottom: '1px solid var(--border-subtle)',
          overflowX: 'auto',
        }}>
          {DOMAINS.map((domain) => {
            const isLocked = currentAge < domain.minAge;
            const isSelected = selectedDomainId === domain.id;
            const Icon = domain.icon;

            return (
              <button
                key={domain.id}
                onClick={() => setSelectedDomainId(domain.id)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  padding: '8px 12px',
                  borderRadius: 'var(--radius-sm)',
                  backgroundColor: isSelected ? 'var(--bg-surface-3)' : 'transparent',
                  border: isSelected ? '1px solid var(--accent-indigo)' : '1px solid transparent',
                  color: isLocked ? 'var(--text-muted)' : (isSelected ? 'var(--text-primary)' : 'var(--text-secondary)'),
                  fontSize: '12px',
                  fontWeight: isSelected ? 600 : 500,
                  cursor: 'pointer',
                  whiteSpace: 'nowrap',
                  opacity: isLocked ? 0.6 : 1.0,
                }}
              >
                <Icon size={14} />
                <span>{domain.name}</span>
                {isLocked && <Lock size={12} style={{ marginLeft: '2px', color: 'var(--text-muted)' }} />}
              </button>
            );
          })}
        </div>

        {/* Selected Domain View Body */}
        <div style={{
          padding: '28px',
          overflowY: 'auto',
          flex: 1,
          display: 'flex',
          flexDirection: 'column',
          gap: '24px',
        }}>
          {/* Domain Intro Card */}
          <div style={{
            backgroundColor: 'var(--bg-surface-2)',
            border: '1px solid var(--border-subtle)',
            borderRadius: 'var(--radius-md)',
            padding: '20px',
            display: 'flex',
            alignItems: 'flex-start',
            gap: '16px',
          }}>
            <div style={{
              width: '40px',
              height: '40px',
              borderRadius: 'var(--radius-md)',
              backgroundColor: 'var(--bg-surface-1)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              color: isSelectedLocked ? 'var(--text-muted)' : 'var(--accent-indigo)',
              border: '1px solid var(--border-subtle)',
              flexShrink: 0,
            }}>
              {React.createElement(activeDomain.icon, { size: 20 })}
            </div>
            <div>
              <div style={{ fontSize: '11px', textTransform: 'uppercase', letterSpacing: '0.1em', color: 'var(--text-muted)', fontWeight: 700 }}>
                {activeDomain.category}
              </div>
              <h3 style={{ fontSize: '18px', fontWeight: 700, margin: '2px 0 6px', color: 'var(--text-primary)' }}>
                {activeDomain.name}
              </h3>
              <p style={{ fontSize: '13px', color: 'var(--text-secondary)', margin: 0, lineHeight: '1.5' }}>
                {activeDomain.description}
              </p>
            </div>
          </div>

          {/* Locked Notice if Player is Underage for this Domain */}
          {isSelectedLocked ? (
            <div style={{
              backgroundColor: 'rgba(239, 68, 68, 0.08)',
              border: '1px solid rgba(239, 68, 68, 0.25)',
              borderRadius: 'var(--radius-md)',
              padding: '28px 24px',
              textAlign: 'center',
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              gap: '12px',
            }}>
              <Lock size={28} style={{ color: 'var(--accent-amber)' }} />
              <div>
                <h4 style={{ fontSize: '16px', fontWeight: 700, margin: 0, color: 'var(--text-primary)' }}>
                  Unlocks at Age {activeDomain.minAge}
                </h4>
                <p style={{ fontSize: '13px', color: 'var(--text-secondary)', margin: '6px 0 0', maxWidth: '400px' }}>
                  At Age {currentAge}, this facet of life has not yet opened. As you grow and mature through the years, new responsibilities, freedoms, and opportunities will emerge naturally.
                </p>
              </div>
            </div>
          ) : (
            /* Unlocked Domain Contextual Details */
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              {selectedDomainId === 'family' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '14px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    HOUSEHOLD & PARENTAL BONDS
                  </div>
                  <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '12px', fontSize: '13px' }}>
                    <div>
                      <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Parental Trust</span>
                      <span style={{ fontWeight: 600 }}>
                        {sidebarData?.household_trust && sidebarData.household_trust > 0.6 ? 'Warm & Supportive' : 'Moderate'}
                      </span>
                    </div>
                    <div>
                      <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Pocket Money / Savings</span>
                      <span style={{ fontWeight: 600 }}>{currSymbol}{gameState.cash.toLocaleString()}</span>
                    </div>
                    <div>
                      <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Housing Environment</span>
                      <span style={{ fontWeight: 600 }}>{gameState.housingType} in {gameState.location.replace('city:real:', '').replace('_', ' ').toUpperCase()}</span>
                    </div>
                    <div>
                      <span style={{ color: 'var(--text-muted)', display: 'block', fontSize: '11px' }}>Household Role</span>
                      <span style={{ fontWeight: 600 }}>{currentAge < 13 ? 'Dependent Child' : (currentAge < 18 ? 'Secondary Student' : 'Adult Family Member')}</span>
                    </div>
                  </div>
                  <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: '1.5', margin: '6px 0 0' }}>
                    Pocket money comes from your parents based on household chores and cooperation. In childhood, your livelihood is sheltered by family.
                  </p>
                </div>
              )}

              {selectedDomainId === 'education' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    ACADEMIC PROGRESS
                  </div>
                  <div style={{ fontSize: '15px', fontWeight: 600, color: 'var(--text-primary)' }}>
                    {currentAge <= 11 ? 'Primary School Education' : (currentAge <= 17 ? 'Senior Secondary Education' : 'Higher Education / Adult Learning')}
                  </div>
                  <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: '1.5', margin: 0 }}>
                    Classroom attendance, library study, and exam revisions build your academic performance and cognitive foundation over time.
                  </p>
                </div>
              )}

              {selectedDomainId === 'talents' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    SKILLS & TALENTS
                  </div>
                  {sidebarData?.primary_skill_name && (
                    <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', textTransform: 'capitalize' }}>
                      Primary Pursuit: {sidebarData.primary_skill_name.replace('_', ' ')}
                    </div>
                  )}
                  <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: '1.5', margin: 0 }}>
                    Practice on the pitch, creative drawing, music practice, and reading hone your individual potential.
                  </p>
                </div>
              )}

              {selectedDomainId === 'social' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    PEER CIRCLES
                  </div>
                  <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: '1.5', margin: 0 }}>
                    Friendships formed on the playground, school clubs, and neighborhood courtyards shape your shared memories and social confidence.
                  </p>
                </div>
              )}

              {selectedDomainId === 'digital' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    ONLINE FOOTPRINT
                  </div>
                  {!!sidebarData?.channel_subscribers && sidebarData.channel_subscribers > 0 ? (
                    <div style={{ fontSize: '15px', fontWeight: 600, color: 'var(--accent-indigo)' }}>
                      Creator Channel: {sidebarData.channel_subscribers.toLocaleString()} subscribers
                    </div>
                  ) : (
                    <p style={{ fontSize: '13px', color: 'var(--text-secondary)', margin: 0 }}>
                      Digital channels, video recording, and online discussions are open for exploration.
                    </p>
                  )}
                </div>
              )}

              {selectedDomainId === 'romance' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    ROMANTIC LIFE
                  </div>
                  <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)' }}>
                    Status: {currentAge < 18 ? 'Single (Youth)' : gameState.maritalStatus}
                  </div>
                </div>
              )}

              {selectedDomainId === 'career' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    PROFESSION & LIVELIHOOD
                  </div>
                  <div style={{ fontSize: '15px', fontWeight: 600, color: 'var(--text-primary)' }}>
                    {gameState.jobTitle}
                  </div>
                  {gameState.monthlySalary > 0 && (
                    <div style={{ fontSize: '13px', color: 'var(--accent-emerald)', fontWeight: 600 }}>
                      Monthly Earnings: {currSymbol}{gameState.monthlySalary.toLocaleString()}
                    </div>
                  )}
                </div>
              )}

              {selectedDomainId === 'faith' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    FAITH & PRINCIPLES
                  </div>
                  <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: '1.5', margin: 0 }}>
                    Spiritual traditions, moral discernment, and philosophical values guide your decisions and life purpose.
                  </p>
                </div>
              )}

              {selectedDomainId === 'civic' && (
                <div style={{
                  backgroundColor: 'var(--bg-surface-2)',
                  borderRadius: 'var(--radius-md)',
                  padding: '20px',
                  display: 'flex',
                  flexDirection: 'column',
                  gap: '12px',
                }}>
                  <div style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase' }}>
                    CIVIC & POLITICAL ENGAGEMENT
                  </div>
                  <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: '1.5', margin: 0 }}>
                    Participating in town halls, exercising voting rights, and organizing community movements.
                  </p>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
