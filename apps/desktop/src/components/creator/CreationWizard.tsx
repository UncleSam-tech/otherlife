import React, { useState } from 'react';
import { X, Sparkles, ChevronRight, ChevronLeft, Check } from 'lucide-react';
import { QuickStartPresets, PresetConfig } from './QuickStartPresets';
import { SkillBudgetEditor } from './SkillBudgetEditor';

export interface NewLifeFormState {
  startingYear: number;
  countryId: string;
  locationId: string;
  startingAge: number;
  firstName: string;
  lastName: string;
  sex: string;
  householdIncomeTier: string;
  budgetMode: 'REALISTIC' | 'SANDBOX' | 'RANDOM';
  traits: Record<string, number>;
  skills: Record<string, number>;
  interests: string[];
  goals: string[];
}

interface CreationWizardProps {
  registries: any;
  onClose: () => void;
  onSubmitNewLife: (formState: NewLifeFormState) => void;
}

export const CreationWizard: React.FC<CreationWizardProps> = ({ registries, onClose, onSubmitNewLife }) => {
  const [step, setStep] = useState(1);

  const [formState, setFormState] = useState<NewLifeFormState>({
    startingYear: 2026,
    countryId: 'country:real:united_kingdom',
    locationId: 'city:real:glasgow',
    startingAge: 14,
    firstName: 'Alex',
    lastName: 'Morgan',
    sex: 'Non-binary',
    householdIncomeTier: 'MIDDLE',
    budgetMode: 'REALISTIC',
    traits: { ambition: 0.5, discipline: 0.5, risk_tolerance: 0.5 },
    skills: { communication: 50 },
    interests: ['football'],
    goals: ['become_wealthy'],
  });

  const handleSelectPreset = (preset: PresetConfig) => {
    setFormState({
      startingYear: 2026,
      countryId: preset.countryId,
      locationId: preset.locationId,
      startingAge: preset.startingAge,
      firstName: preset.name.split(' ')[0] || 'Alex',
      lastName: 'Morgan',
      sex: 'Non-binary',
      householdIncomeTier: preset.incomeTier,
      budgetMode: 'REALISTIC',
      traits: preset.traits,
      skills: preset.skills,
      interests: preset.interests,
      goals: preset.goals,
    });
    setStep(9); // Jump to Review
  };

  const handleSkillChange = (skillId: string, val: number) => {
    setFormState((prev) => ({
      ...prev,
      skills: { ...prev.skills, [skillId]: val },
    }));
  };

  const toggleInterest = (id: string) => {
    setFormState((prev) => {
      const current = prev.interests;
      const next = current.includes(id) ? current.filter((x) => x !== id) : [...current, id];
      return { ...prev, interests: next };
    });
  };

  const toggleGoal = (id: string) => {
    setFormState((prev) => {
      const current = prev.goals;
      const next = current.includes(id) ? current.filter((x) => x !== id) : [...current, id];
      return { ...prev, goals: next };
    });
  };

  const handleCountryChange = (newCountryId: string) => {
    const validLocs = locations.filter((loc: any) => loc.country_id === newCountryId);
    const firstLocId = validLocs.length > 0 ? validLocs[0].id : '';
    setFormState((prev) => ({
      ...prev,
      countryId: newCountryId,
      locationId: firstLocId,
    }));
  };

  const countries = registries?.countries || [];
  const locations = registries?.locations || [];
  const skillsList = registries?.skills || [];
  const interestsList = registries?.interests || [];
  const goalsList = registries?.goals || [];

  const filteredLocations = locations.filter((loc: any) => loc.country_id === formState.countryId);

  return (
    <div style={{
      position: 'fixed',
      top: 0,
      left: 0,
      right: 0,
      bottom: 0,
      backgroundColor: 'rgba(0,0,0,0.85)',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
      zIndex: 1000,
    }}>
      <div style={{
        backgroundColor: 'var(--bg-surface-1)',
        width: '640px',
        maxHeight: '90vh',
        borderRadius: 'var(--radius-lg)',
        border: '1px solid var(--border-strong)',
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden',
      }}>
        <div style={{
          padding: '16px 20px',
          borderBottom: '1px solid var(--border-subtle)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            <Sparkles size={18} color="var(--accent-amber)" />
            <h3 style={{ fontSize: '16px', fontWeight: 700 }}>Initialize Alternate Timeline</h3>
          </div>
          <button onClick={onClose} style={{ background: 'none', border: 'none', color: 'var(--text-muted)', cursor: 'pointer' }}>
            <X size={18} />
          </button>
        </div>

        <div style={{ padding: '20px', flex: 1, overflowY: 'auto' }}>
          {step === 1 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <p style={{ fontSize: '13px', color: 'var(--text-muted)' }}>
                Choose a pre-configured starting life persona or customize every aspect of your new timeline.
              </p>
              <QuickStartPresets onSelectPreset={handleSelectPreset} />
              <button
                onClick={() => setStep(2)}
                style={{
                  alignSelf: 'flex-start',
                  marginTop: '12px',
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
                Customize Life Step-by-Step →
              </button>
            </div>
          )}

          {step === 2 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600 }}>World & Location</h4>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
                <div>
                  <label style={{ fontSize: '12px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>Country</label>
                  <select
                    value={formState.countryId}
                    onChange={(e) => handleCountryChange(e.target.value)}
                    style={{ width: '100%', padding: '10px', backgroundColor: 'var(--bg-app)', border: '1px solid var(--border-strong)', color: '#FFF', borderRadius: 'var(--radius-sm)' }}
                  >
                    {countries.map((c: any) => (
                      <option key={c.id} value={c.id}>{c.name} ({c.currency_symbol})</option>
                    ))}
                  </select>
                </div>

                <div>
                  <label style={{ fontSize: '12px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>City / Location</label>
                  <select
                    value={formState.locationId}
                    onChange={(e) => setFormState({ ...formState, locationId: e.target.value })}
                    style={{ width: '100%', padding: '10px', backgroundColor: 'var(--bg-app)', border: '1px solid var(--border-strong)', color: '#FFF', borderRadius: 'var(--radius-sm)' }}
                  >
                    {filteredLocations.map((loc: any) => (
                      <option key={loc.id} value={loc.id}>{loc.name} ({loc.region_name})</option>
                    ))}
                  </select>
                </div>
              </div>
            </div>
          )}

          {step === 3 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600 }}>Starting Age & Timeline Year</h4>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
                <div>
                  <label style={{ fontSize: '12px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>Starting Age</label>
                  <select
                    value={formState.startingAge}
                    onChange={(e) => setFormState({ ...formState, startingAge: parseInt(e.target.value, 10) })}
                    style={{ width: '100%', padding: '10px', backgroundColor: 'var(--bg-app)', border: '1px solid var(--border-strong)', color: '#FFF', borderRadius: 'var(--radius-sm)' }}
                  >
                    <option value={0}>Birth (Age 0)</option>
                    <option value={5}>Early Childhood (Age 5)</option>
                    <option value={10}>Childhood (Age 10)</option>
                    <option value={14}>Adolescence (Age 14)</option>
                    <option value={18}>Young Adult (Age 18)</option>
                    <option value={25}>Adult (Age 25)</option>
                    <option value={30}>Mature Adult (Age 30)</option>
                  </select>
                </div>

                <div>
                  <label style={{ fontSize: '12px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>Starting Timeline Year</label>
                  <input
                    type="number"
                    value={formState.startingYear}
                    onChange={(e) => setFormState({ ...formState, startingYear: parseInt(e.target.value, 10) })}
                    style={{ width: '100%', padding: '10px', backgroundColor: 'var(--bg-app)', border: '1px solid var(--border-strong)', color: '#FFF', borderRadius: 'var(--radius-sm)' }}
                  />
                </div>
              </div>
            </div>
          )}

          {step === 4 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600 }}>Identity & Demographics</h4>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
                <div>
                  <label style={{ fontSize: '12px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>First Name</label>
                  <input
                    type="text"
                    value={formState.firstName}
                    onChange={(e) => setFormState({ ...formState, firstName: e.target.value })}
                    style={{ width: '100%', padding: '10px', backgroundColor: 'var(--bg-app)', border: '1px solid var(--border-strong)', color: '#FFF', borderRadius: 'var(--radius-sm)' }}
                  />
                </div>
                <div>
                  <label style={{ fontSize: '12px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>Last Name</label>
                  <input
                    type="text"
                    value={formState.lastName}
                    onChange={(e) => setFormState({ ...formState, lastName: e.target.value })}
                    style={{ width: '100%', padding: '10px', backgroundColor: 'var(--bg-app)', border: '1px solid var(--border-strong)', color: '#FFF', borderRadius: 'var(--radius-sm)' }}
                  />
                </div>
              </div>
            </div>
          )}

          {step === 5 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600 }}>Household Income & Background</h4>
              <div>
                <label style={{ fontSize: '12px', color: 'var(--text-muted)', display: 'block', marginBottom: '4px' }}>Household Income Tier</label>
                <select
                  value={formState.householdIncomeTier}
                  onChange={(e) => setFormState({ ...formState, householdIncomeTier: e.target.value })}
                  style={{ width: '100%', padding: '10px', backgroundColor: 'var(--bg-app)', border: '1px solid var(--border-strong)', color: '#FFF', borderRadius: 'var(--radius-sm)' }}
                >
                  <option value="LOW">Low Income Household</option>
                  <option value="MIDDLE">Middle Income Household</option>
                  <option value="HIGH">High Income / Wealthy Household</option>
                </select>
              </div>
            </div>
          )}

          {step === 6 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600 }}>Skill Creation & Abilities</h4>
              <SkillBudgetEditor
                skillsList={skillsList}
                selectedSkills={formState.skills}
                onChangeSkill={handleSkillChange}
                budgetMode={formState.budgetMode}
              />
            </div>
          )}

          {step === 7 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600 }}>Starting Interests</h4>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(180px, 1fr))', gap: '8px' }}>
                {interestsList.map((int: any) => {
                  const isSel = formState.interests.includes(int.id);
                  return (
                    <button
                      key={int.id}
                      onClick={() => toggleInterest(int.id)}
                      style={{
                        padding: '10px',
                        borderRadius: 'var(--radius-sm)',
                        backgroundColor: isSel ? 'var(--accent-indigo-subtle)' : 'var(--bg-surface-2)',
                        border: `1px solid ${isSel ? 'var(--accent-indigo)' : 'var(--border-subtle)'}`,
                        color: isSel ? 'var(--accent-indigo)' : 'var(--text-secondary)',
                        cursor: 'pointer',
                        textAlign: 'left',
                        fontSize: '12px',
                        fontWeight: isSel ? 600 : 400,
                      }}
                    >
                      {int.name}
                    </button>
                  );
                })}
              </div>
            </div>
          )}

          {step === 8 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600 }}>Lifelong Ambitions & Goals</h4>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr', gap: '8px' }}>
                {goalsList.map((g: any) => {
                  const isSel = formState.goals.includes(g.id);
                  return (
                    <button
                      key={g.id}
                      onClick={() => toggleGoal(g.id)}
                      style={{
                        padding: '12px',
                        borderRadius: 'var(--radius-sm)',
                        backgroundColor: isSel ? 'var(--accent-emerald-subtle)' : 'var(--bg-surface-2)',
                        border: `1px solid ${isSel ? 'var(--accent-emerald)' : 'var(--border-subtle)'}`,
                        color: isSel ? 'var(--accent-emerald)' : 'var(--text-secondary)',
                        cursor: 'pointer',
                        textAlign: 'left',
                        fontSize: '13px',
                        fontWeight: isSel ? 600 : 400,
                      }}
                    >
                      {g.name}
                    </button>
                  );
                })}
              </div>
            </div>
          )}

          {step === 9 && (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              <h4 style={{ fontSize: '15px', fontWeight: 600, color: 'var(--accent-emerald)' }}>Review & Begin Alternate Life</h4>
              <div style={{ backgroundColor: 'var(--bg-surface-2)', padding: '16px', borderRadius: 'var(--radius-md)', display: 'flex', flexDirection: 'column', gap: '8px', fontSize: '13px' }}>
                <div><strong>Character:</strong> {formState.firstName} {formState.lastName} (Age {formState.startingAge})</div>
                <div><strong>Location:</strong> {formState.locationId.replace('city:real:', '').toUpperCase()} ({formState.countryId.replace('country:real:', '').toUpperCase()})</div>
                <div><strong>Income Tier:</strong> {formState.householdIncomeTier}</div>
                <div><strong>Selected Interests:</strong> {formState.interests.join(', ') || 'None'}</div>
                <div><strong>Selected Goals:</strong> {formState.goals.join(', ') || 'None'}</div>
              </div>
            </div>
          )}
        </div>

        {/* Modal Footer Controls */}
        <div style={{
          padding: '16px 24px',
          backgroundColor: 'var(--bg-surface-2)',
          borderTop: '1px solid var(--border-subtle)',
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}>
          {step > 1 ? (
            <button onClick={() => setStep(step - 1)} style={{ display: 'flex', alignItems: 'center', gap: '4px', backgroundColor: 'var(--bg-surface-1)', border: '1px solid var(--border-subtle)', color: '#FFF', padding: '8px 16px', borderRadius: 'var(--radius-md)', cursor: 'pointer' }}>
              <ChevronLeft size={16} />
              <span>Back</span>
            </button>
          ) : <div />}

          {step < 9 ? (
            <button onClick={() => setStep(step + 1)} style={{ display: 'flex', alignItems: 'center', gap: '4px', backgroundColor: 'var(--accent-indigo)', color: '#FFF', border: 'none', padding: '8px 16px', borderRadius: 'var(--radius-md)', cursor: 'pointer', fontWeight: 600 }}>
              <span>Next</span>
              <ChevronRight size={16} />
            </button>
          ) : (
            <button onClick={() => onSubmitNewLife(formState)} style={{ display: 'flex', alignItems: 'center', gap: '6px', backgroundColor: 'var(--accent-emerald)', color: '#FFF', border: 'none', padding: '10px 20px', borderRadius: 'var(--radius-md)', cursor: 'pointer', fontWeight: 700 }}>
              <Check size={16} />
              <span>Begin Timeline</span>
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
