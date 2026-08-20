import React, { useState } from 'react';
import { Sparkles, ChevronRight, ChevronLeft, Check, RotateCcw, AlertTriangle, X } from 'lucide-react';
import { SkillBudgetEditor } from './SkillBudgetEditor';

export interface NewLifeFormState {
  startingYear: number;
  countryId: string | null;
  locationId: string | null;
  startingAge: number | null;
  firstName: string;
  lastName: string;
  sex: string | null;
  householdIncomeTier: string | null;
  realismMode: 'REALISTIC' | 'SANDBOX' | 'RANDOM';
  traits: Record<string, number>;
  skills: Record<string, number>;
  interests: string[];
  goals: string[];
  backstory: {
    education: string;
    occupation: string;
    maritalStatus: string;
  };
}

interface CreationWizardProps {
  registries: any;
  onClose: () => void;
  onSubmitNewLife: (formState: NewLifeFormState) => void;
}

const STEPS = [
  { id: 1, name: 'World' },
  { id: 2, name: 'Identity' },
  { id: 3, name: 'Family' },
  { id: 4, name: 'Personality' },
  { id: 5, name: 'Skills' },
  { id: 6, name: 'Interests & Goals' },
  { id: 7, name: 'Backstory' },
  { id: 8, name: 'Quick Start' },
  { id: 9, name: 'Review' },
];

export const CreationWizard: React.FC<CreationWizardProps> = ({ registries, onClose, onSubmitNewLife }) => {
  const [step, setStep] = useState(1);

  const [formState, setFormState] = useState<NewLifeFormState>({
    startingYear: 2026,
    countryId: null,
    locationId: null,
    startingAge: null,
    firstName: '',
    lastName: '',
    sex: null,
    householdIncomeTier: null,
    realismMode: 'REALISTIC',
    traits: { ambition: 0.5, discipline: 0.5, risk_tolerance: 0.5, empathy: 0.5, creativity: 0.5, resilience: 0.5 },
    skills: {},
    interests: [],
    goals: [],
    backstory: { education: '', occupation: '', maritalStatus: 'Single' },
  });

  const countries = registries?.countries || [];
  const locations = registries?.locations || [];
  const registeredSkills = registries?.skills || [];
  const registeredInterests = registries?.interests || [];

  const validLocations = formState.countryId
    ? locations.filter((loc: any) => loc.country_id === formState.countryId)
    : [];

  const handleCountryChange = (newCountryId: string) => {
    setFormState((prev) => ({
      ...prev,
      countryId: newCountryId,
      locationId: null, // Clear incompatible city
    }));
  };

  const handleRandomizeSection = (section: number) => {
    if (section === 1) {
      const randomCountry = countries[Math.floor(Math.random() * countries.length)];
      const cId = randomCountry ? randomCountry.id : 'country:real:nigeria';
      const locs = locations.filter((l: any) => l.country_id === cId);
      const rLoc = locs[Math.floor(Math.random() * locs.length)];
      setFormState((prev) => ({
        ...prev,
        countryId: cId,
        locationId: rLoc ? rLoc.id : 'city:real:lagos',
        startingYear: 2026,
      }));
    } else if (section === 2) {
      const firstNames = ['Israel', 'Amina', 'Liam', 'Sarah', 'Kaito', 'Elena', 'Mateo', 'Zoe'];
      const lastNames = ['Okonkwo', 'Smith', 'Adeyemi', 'Vance', 'Tanaka', 'Garcia', 'Chen', 'Taylor'];
      const sexes = ['Male', 'Female', 'Non-binary'];
      setFormState((prev) => ({
        ...prev,
        firstName: firstNames[Math.floor(Math.random() * firstNames.length)],
        lastName: lastNames[Math.floor(Math.random() * lastNames.length)],
        sex: sexes[Math.floor(Math.random() * sexes.length)],
        startingAge: Math.floor(Math.random() * 40),
      }));
    } else if (section === 3) {
      const tiers = ['POOR', 'WORKING', 'MIDDLE', 'HIGH', 'WEALTHY'];
      setFormState((prev) => ({
        ...prev,
        householdIncomeTier: tiers[Math.floor(Math.random() * tiers.length)],
      }));
    } else if (section === 4) {
      setFormState((prev) => ({
        ...prev,
        traits: {
          ambition: Math.random(),
          discipline: Math.random(),
          risk_tolerance: Math.random(),
          empathy: Math.random(),
          creativity: Math.random(),
          resilience: Math.random(),
        },
      }));
    } else if (section === 5) {
      const randomSkills: Record<string, number> = {};
      if (registeredSkills.length > 0) {
        const s1 = registeredSkills[Math.floor(Math.random() * registeredSkills.length)];
        randomSkills[s1.id] = Math.floor(Math.random() * 50) + 20;
      }
      setFormState((prev) => ({ ...prev, skills: randomSkills }));
    } else if (section === 6) {
      const rInterests = registeredInterests.length > 0
        ? [registeredInterests[Math.floor(Math.random() * registeredInterests.length)].id]
        : [];
      setFormState((prev) => ({ ...prev, interests: rInterests, goals: [] }));
    }
  };

  const handleFullRandomLife = () => {
    handleRandomizeSection(1);
    handleRandomizeSection(2);
    handleRandomizeSection(3);
    handleRandomizeSection(4);
    handleRandomizeSection(5);
    handleRandomizeSection(6);
    setStep(9);
  };

  const isStepValid = (s: number): boolean => {
    switch (s) {
      case 1:
        return !!formState.countryId && !!formState.locationId && !!formState.startingYear;
      case 2:
        return !!formState.firstName.trim() && !!formState.lastName.trim() && !!formState.sex && formState.startingAge !== null;
      case 3:
        return !!formState.householdIncomeTier;
      case 4:
        return true;
      case 5:
        return true;
      case 6:
        return true; // Optional interests and goals
      case 7:
        return true; // Backstory optional
      case 8:
        return true;
      case 9:
        return isStepValid(1) && isStepValid(2) && isStepValid(3);
      default:
        return true;
    }
  };

  const handleNext = () => {
    if (step < 9) {
      // Skip backstory if age is 0
      if (step === 6 && formState.startingAge === 0) {
        setStep(8);
      } else {
        setStep(step + 1);
      }
    }
  };

  const handlePrev = () => {
    if (step > 1) {
      if (step === 8 && formState.startingAge === 0) {
        setStep(6);
      } else {
        setStep(step - 1);
      }
    }
  };

  const isDataMissing = !countries || countries.length === 0;

  return (
    <div style={{
      position: 'fixed',
      inset: 0,
      backgroundColor: '#0A0C10',
      color: '#E2E8F0',
      zIndex: 1000,
      display: 'flex',
      flexDirection: 'column',
      fontFamily: 'var(--font-sans, system-ui, sans-serif)',
      overflowY: 'auto',
    }}>
      {/* Top Bar Navigation */}
      <header style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '16px 32px',
        backgroundColor: '#0F172A',
        borderBottom: '1px solid #1E293B',
      }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
          <h2 style={{ fontSize: '18px', fontWeight: 800, fontFamily: 'monospace', color: '#F8FAFC', letterSpacing: '0.1em', margin: 0 }}>
            CREATE NEW LIFE
          </h2>
          <span style={{ color: '#334155' }}>|</span>
          <span style={{ fontSize: '13px', color: '#94A3B8' }}>
            Step {step} of 9: <strong style={{ color: '#818CF8' }}>{STEPS.find((s) => s.id === step)?.name}</strong>
          </span>
        </div>

        <button
          onClick={onClose}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '6px',
            backgroundColor: 'transparent',
            color: '#94A3B8',
            border: '1px solid #334155',
            borderRadius: '6px',
            padding: '6px 12px',
            fontSize: '13px',
            cursor: 'pointer',
          }}
        >
          <X size={16} />
          <span>Cancel</span>
        </button>
      </header>

      {/* Step Indicator Progress Bar */}
      <div style={{
        display: 'flex',
        borderBottom: '1px solid #1E293B',
        backgroundColor: '#0B0F19',
        overflowX: 'auto',
      }}>
        {STEPS.map((s) => (
          <button
            key={s.id}
            onClick={() => setStep(s.id)}
            style={{
              flex: 1,
              minWidth: '100px',
              padding: '12px 10px',
              backgroundColor: s.id === step ? '#1E293B' : 'transparent',
              color: s.id === step ? '#818CF8' : s.id < step ? '#10B981' : '#475569',
              border: 'none',
              borderBottom: s.id === step ? '2px solid #818CF8' : 'none',
              fontSize: '12px',
              fontWeight: 600,
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '6px',
            }}
          >
            <span>{s.id}. {s.name}</span>
            {s.id < step && <Check size={12} />}
          </button>
        ))}
      </div>

      {/* Main Content Area */}
      <div style={{
        flex: 1,
        maxWidth: '800px',
        width: '100%',
        margin: '0 auto',
        padding: '36px 24px',
        display: 'flex',
        flexDirection: 'column',
        gap: '24px',
      }}>
        {isDataMissing && (
          <div style={{
            padding: '14px 18px',
            backgroundColor: '#451A1A',
            border: '1px solid #991B1B',
            borderRadius: '8px',
            color: '#FCA5A5',
            fontSize: '13px',
            display: 'flex',
            alignItems: 'center',
            gap: '10px',
          }}>
            <AlertTriangle size={18} />
            <span>World registry data unavailable. Please check backend connection or developer mode.</span>
          </div>
        )}

        {/* STEP 1: WORLD */}
        {step === 1 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Starting World & Location</h3>
                <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Choose the starting country, city, and era for this life.</p>
              </div>
              <button
                onClick={() => handleRandomizeSection(1)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: '#1E293B',
                  color: '#818CF8',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  padding: '6px 12px',
                  fontSize: '12px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <RotateCcw size={14} />
                <span>Randomize World</span>
              </button>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <div>
                <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>STARTING YEAR</label>
                <input
                  type="number"
                  value={formState.startingYear}
                  onChange={(e) => setFormState({ ...formState, startingYear: parseInt(e.target.value) || 2026 })}
                  style={{
                    width: '100%',
                    padding: '10px 14px',
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    borderRadius: '6px',
                    color: '#FFF',
                    fontSize: '14px',
                  }}
                />
              </div>

              <div>
                <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>BIRTH COUNTRY *</label>
                <select
                  value={formState.countryId || ''}
                  onChange={(e) => handleCountryChange(e.target.value)}
                  style={{
                    width: '100%',
                    padding: '10px 14px',
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    borderRadius: '6px',
                    color: formState.countryId ? '#FFF' : '#64748B',
                    fontSize: '14px',
                  }}
                >
                  <option value="" disabled>-- Select Country --</option>
                  {countries.map((c: any) => (
                    <option key={c.id} value={c.id}>{c.name} ({c.currency_symbol || '£'})</option>
                  ))}
                </select>
              </div>
            </div>

            <div>
              <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>BIRTH CITY / LOCATION *</label>
              <select
                disabled={!formState.countryId}
                value={formState.locationId || ''}
                onChange={(e) => setFormState({ ...formState, locationId: e.target.value })}
                style={{
                  width: '100%',
                  padding: '10px 14px',
                  backgroundColor: formState.countryId ? '#1E293B' : '#0F172A',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  color: formState.locationId ? '#FFF' : '#64748B',
                  fontSize: '14px',
                  cursor: formState.countryId ? 'pointer' : 'not-allowed',
                }}
              >
                <option value="" disabled>
                  {formState.countryId ? '-- Select City --' : 'Select a country first'}
                </option>
                {validLocations.map((loc: any) => (
                  <option key={loc.id} value={loc.id}>
                    {loc.name} {loc.region_name ? `(${loc.region_name})` : ''}
                  </option>
                ))}
              </select>
            </div>
          </div>
        )}

        {/* STEP 2: IDENTITY & AGE */}
        {step === 2 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Personal Identity & Age</h3>
                <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Define the starting age and basic identity details.</p>
              </div>
              <button
                onClick={() => handleRandomizeSection(2)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: '#1E293B',
                  color: '#818CF8',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  padding: '6px 12px',
                  fontSize: '12px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <RotateCcw size={14} />
                <span>Randomize Identity</span>
              </button>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <div>
                <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>FIRST NAME *</label>
                <input
                  type="text"
                  placeholder="e.g. Israel, Amina, Liam"
                  value={formState.firstName}
                  onChange={(e) => setFormState({ ...formState, firstName: e.target.value })}
                  style={{
                    width: '100%',
                    padding: '10px 14px',
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    borderRadius: '6px',
                    color: '#FFF',
                    fontSize: '14px',
                  }}
                />
              </div>

              <div>
                <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>LAST NAME *</label>
                <input
                  type="text"
                  placeholder="e.g. Okonkwo, Smith, Taylor"
                  value={formState.lastName}
                  onChange={(e) => setFormState({ ...formState, lastName: e.target.value })}
                  style={{
                    width: '100%',
                    padding: '10px 14px',
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    borderRadius: '6px',
                    color: '#FFF',
                    fontSize: '14px',
                  }}
                />
              </div>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <div>
                <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>SEX *</label>
                <select
                  value={formState.sex || ''}
                  onChange={(e) => setFormState({ ...formState, sex: e.target.value })}
                  style={{
                    width: '100%',
                    padding: '10px 14px',
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    borderRadius: '6px',
                    color: formState.sex ? '#FFF' : '#64748B',
                    fontSize: '14px',
                  }}
                >
                  <option value="" disabled>-- Select Sex --</option>
                  <option value="Male">Male</option>
                  <option value="Female">Female</option>
                  <option value="Non-binary">Non-binary</option>
                </select>
              </div>

              <div>
                <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>STARTING AGE *</label>
                <div style={{ display: 'flex', gap: '10px', alignItems: 'center' }}>
                  <input
                    type="range"
                    min={0}
                    max={75}
                    value={formState.startingAge ?? 0}
                    onChange={(e) => setFormState({ ...formState, startingAge: parseInt(e.target.value) })}
                    style={{ flex: 1 }}
                  />
                  <span style={{ width: '60px', textAlign: 'right', fontWeight: 700, color: '#818CF8', fontSize: '16px' }}>
                    {formState.startingAge !== null ? (formState.startingAge === 0 ? 'Birth (0)' : `${formState.startingAge} yrs`) : 'Unset'}
                  </span>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* STEP 3: FAMILY & BACKGROUND */}
        {step === 3 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Family & Economic Background</h3>
                <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Starting household environment and economic condition.</p>
              </div>
              <button
                onClick={() => handleRandomizeSection(3)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: '#1E293B',
                  color: '#818CF8',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  padding: '6px 12px',
                  fontSize: '12px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <RotateCcw size={14} />
                <span>Randomize Background</span>
              </button>
            </div>

            <div>
              <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '8px', fontWeight: 600 }}>HOUSEHOLD INCOME TIER *</label>
              <div style={{ display: 'grid', gridTemplateColumns: 'repeat(5, 1fr)', gap: '10px' }}>
                {['POOR', 'WORKING', 'MIDDLE', 'HIGH', 'WEALTHY'].map((tier) => (
                  <button
                    key={tier}
                    onClick={() => setFormState({ ...formState, householdIncomeTier: tier })}
                    style={{
                      padding: '12px 8px',
                      backgroundColor: formState.householdIncomeTier === tier ? '#4F46E5' : '#1E293B',
                      color: formState.householdIncomeTier === tier ? '#FFF' : '#CBD5E1',
                      border: '1px solid',
                      borderColor: formState.householdIncomeTier === tier ? '#6366F1' : '#334155',
                      borderRadius: '6px',
                      fontSize: '12px',
                      fontWeight: 600,
                      cursor: 'pointer',
                    }}
                  >
                    {tier}
                  </button>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* STEP 4: PERSONALITY */}
        {step === 4 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Personality & Temperament</h3>
                <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Calibrate innate personality tendencies.</p>
              </div>
              <button
                onClick={() => handleRandomizeSection(4)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: '#1E293B',
                  color: '#818CF8',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  padding: '6px 12px',
                  fontSize: '12px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <RotateCcw size={14} />
                <span>Randomize Traits</span>
              </button>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              {Object.entries(formState.traits).map(([traitKey, val]) => (
                <div key={traitKey} style={{ backgroundColor: '#1E293B', padding: '12px 16px', borderRadius: '6px', border: '1px solid #334155' }}>
                  <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '12px', marginBottom: '6px' }}>
                    <span style={{ color: '#E2E8F0', fontWeight: 600, textTransform: 'capitalize' }}>{traitKey.replace('_', ' ')}</span>
                    <span style={{ color: '#818CF8', fontWeight: 700 }}>{(val * 100).toFixed(0)}%</span>
                  </div>
                  <input
                    type="range"
                    min={0}
                    max={1}
                    step={0.05}
                    value={val}
                    onChange={(e) => setFormState({
                      ...formState,
                      traits: { ...formState.traits, [traitKey]: parseFloat(e.target.value) },
                    })}
                    style={{ width: '100%' }}
                  />
                </div>
              ))}
            </div>
          </div>
        )}

        {/* STEP 5: SKILLS */}
        {step === 5 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Skills & Initial Aptitudes</h3>
                <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Configure baseline abilities (or leave empty to acquire naturally).</p>
              </div>
              <button
                onClick={() => handleRandomizeSection(5)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  backgroundColor: '#1E293B',
                  color: '#818CF8',
                  border: '1px solid #334155',
                  borderRadius: '6px',
                  padding: '6px 12px',
                  fontSize: '12px',
                  fontWeight: 600,
                  cursor: 'pointer',
                }}
              >
                <RotateCcw size={14} />
                <span>Randomize Skills</span>
              </button>
            </div>

            <SkillBudgetEditor
              skillsList={registeredSkills}
              selectedSkills={formState.skills}
              onChangeSkill={(id, val) => setFormState({ ...formState, skills: { ...formState.skills, [id]: val } })}
              budgetMode={formState.realismMode}
            />
          </div>
        )}

        {/* STEP 6: INTERESTS & GOALS */}
        {step === 6 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div>
                <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Interests & Aspirations (Optional)</h3>
                <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>You may leave these unselected — interests and goals will emerge naturally.</p>
              </div>
            </div>

            <div>
              <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '8px', fontWeight: 600 }}>INTERESTS</label>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
                {registeredInterests.map((int: any) => {
                  const selected = formState.interests.includes(int.id);
                  return (
                    <button
                      key={int.id}
                      onClick={() => {
                        const next = selected ? formState.interests.filter((x) => x !== int.id) : [...formState.interests, int.id];
                        setFormState({ ...formState, interests: next });
                      }}
                      style={{
                        padding: '8px 14px',
                        backgroundColor: selected ? '#4F46E5' : '#1E293B',
                        color: selected ? '#FFF' : '#94A3B8',
                        border: '1px solid',
                        borderColor: selected ? '#6366F1' : '#334155',
                        borderRadius: '20px',
                        fontSize: '13px',
                        fontWeight: 600,
                        cursor: 'pointer',
                      }}
                    >
                      {int.name}
                    </button>
                  );
                })}
              </div>
            </div>
          </div>
        )}

        {/* STEP 7: BACKSTORY (If older) */}
        {step === 7 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div>
              <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Life Backstory & History</h3>
              <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Starting at age {formState.startingAge || 0}. Generate or define prior education and occupation.</p>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <div>
                <label style={{ display: 'block', fontSize: '12px', color: '#94A3B8', marginBottom: '6px', fontWeight: 600 }}>MARITAL STATUS</label>
                <select
                  value={formState.backstory.maritalStatus}
                  onChange={(e) => setFormState({
                    ...formState,
                    backstory: { ...formState.backstory, maritalStatus: e.target.value },
                  })}
                  style={{
                    width: '100%',
                    padding: '10px 14px',
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    borderRadius: '6px',
                    color: '#FFF',
                    fontSize: '14px',
                  }}
                >
                  <option value="Single">Single</option>
                  <option value="Married">Married</option>
                  <option value="Divorced">Divorced</option>
                </select>
              </div>
            </div>
          </div>
        )}

        {/* STEP 8: QUICK START / PRESETS */}
        {step === 8 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div>
              <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Quick Start Generation Modes</h3>
              <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Alternatively, let the simulation generate your starting life instantly.</p>
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '14px' }}>
              <button
                onClick={handleFullRandomLife}
                style={{
                  padding: '20px',
                  backgroundColor: '#1E293B',
                  border: '1px solid #475569',
                  borderRadius: '8px',
                  textAlign: 'left',
                  color: '#F8FAFC',
                  cursor: 'pointer',
                }}
              >
                <div style={{ fontSize: '16px', fontWeight: 700, color: '#818CF8' }}>🎲 Fully Random Life</div>
                <div style={{ fontSize: '12px', color: '#94A3B8', marginTop: '4px' }}>
                  Generate random country, city, identity, and starting age realistically.
                </div>
              </button>
            </div>
          </div>
        )}

        {/* STEP 9: REVIEW & LAUNCH */}
        {step === 9 && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
            <div>
              <h3 style={{ fontSize: '20px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>Review Life Parameters</h3>
              <p style={{ fontSize: '13px', color: '#94A3B8', marginTop: '4px' }}>Confirm your selections before initializing the simulation engine.</p>
            </div>

            <div style={{ backgroundColor: '#1E293B', borderRadius: '8px', padding: '20px', border: '1px solid #334155', display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
              <div>
                <div style={{ fontSize: '11px', color: '#94A3B8', fontWeight: 600 }}>NAME & IDENTITY</div>
                <div style={{ fontSize: '16px', fontWeight: 700, color: '#FFF', marginTop: '2px' }}>
                  {formState.firstName || 'Unset'} {formState.lastName} ({formState.sex || 'Unset'})
                </div>
              </div>

              <div>
                <div style={{ fontSize: '11px', color: '#94A3B8', fontWeight: 600 }}>STARTING AGE</div>
                <div style={{ fontSize: '16px', fontWeight: 700, color: '#818CF8', marginTop: '2px' }}>
                  {formState.startingAge !== null ? (formState.startingAge === 0 ? 'Birth (0)' : `${formState.startingAge} yrs`) : 'Unset'}
                </div>
              </div>

              <div>
                <div style={{ fontSize: '11px', color: '#94A3B8', fontWeight: 600 }}>LOCATION</div>
                <div style={{ fontSize: '16px', fontWeight: 700, color: '#FFF', marginTop: '2px' }}>
                  {formState.locationId ? formState.locationId.replace('city:real:', '').toUpperCase() : 'Unset'} ({formState.countryId ? formState.countryId.replace('country:real:', '').toUpperCase() : 'Unset'})
                </div>
              </div>

              <div>
                <div style={{ fontSize: '11px', color: '#94A3B8', fontWeight: 600 }}>HOUSEHOLD TIER</div>
                <div style={{ fontSize: '16px', fontWeight: 700, color: '#10B981', marginTop: '2px' }}>
                  {formState.householdIncomeTier || 'Unset'}
                </div>
              </div>
            </div>

            {!isStepValid(9) && (
              <div style={{ padding: '12px', backgroundColor: '#451A1A', color: '#FCA5A5', borderRadius: '6px', fontSize: '13px' }}>
                ⚠️ Missing required creation fields (Country, City, Identity, or Age). Please complete earlier steps.
              </div>
            )}
          </div>
        )}
      </div>

      {/* Bottom Step Control Bar */}
      <footer style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '16px 32px',
        backgroundColor: '#0F172A',
        borderTop: '1px solid #1E293B',
      }}>
        <button
          onClick={handlePrev}
          disabled={step === 1}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '6px',
            backgroundColor: '#1E293B',
            color: step === 1 ? '#475569' : '#FFF',
            border: '1px solid #334155',
            borderRadius: '6px',
            padding: '10px 18px',
            fontSize: '13px',
            fontWeight: 600,
            cursor: step === 1 ? 'not-allowed' : 'pointer',
          }}
        >
          <ChevronLeft size={16} />
          <span>Back</span>
        </button>

        {step < 9 ? (
          <button
            onClick={handleNext}
            disabled={!isStepValid(step)}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '6px',
              backgroundColor: isStepValid(step) ? '#4F46E5' : '#312E81',
              color: isStepValid(step) ? '#FFF' : '#6366F1',
              border: 'none',
              borderRadius: '6px',
              padding: '10px 22px',
              fontSize: '13px',
              fontWeight: 700,
              cursor: isStepValid(step) ? 'pointer' : 'not-allowed',
            }}
          >
            <span>Next</span>
            <ChevronRight size={16} />
          </button>
        ) : (
          <button
            onClick={() => isStepValid(9) && onSubmitNewLife(formState)}
            disabled={!isStepValid(9)}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: isStepValid(9) ? '#10B981' : '#065F46',
              color: '#FFF',
              border: 'none',
              borderRadius: '6px',
              padding: '10px 24px',
              fontSize: '14px',
              fontWeight: 700,
              cursor: isStepValid(9) ? 'pointer' : 'not-allowed',
              boxShadow: '0 4px 14px rgba(16, 185, 129, 0.4)',
            }}
          >
            <Sparkles size={16} />
            <span>Begin Life</span>
          </button>
        )}
      </footer>
    </div>
  );
};
