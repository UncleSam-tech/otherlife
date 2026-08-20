import React from 'react';
import { Dices, Trophy, Music, Landmark, DollarSign } from 'lucide-react';

export interface PresetConfig {
  name: string;
  description: string;
  icon: any;
  countryId: string;
  locationId: string;
  startingAge: number;
  incomeTier: string;
  interests: string[];
  goals: string[];
  skills: Record<string, number>;
  traits: Record<string, number>;
}

interface QuickStartPresetsProps {
  onSelectPreset: (preset: PresetConfig) => void;
}

export const presetsList: PresetConfig[] = [
  {
    name: 'Academy Football Prospect',
    description: '14yo in Glasgow with high athletic potential and ambition for professional football.',
    icon: Trophy,
    countryId: 'country:real:united_kingdom',
    locationId: 'city:real:glasgow',
    startingAge: 14,
    incomeTier: 'MIDDLE',
    interests: ['football'],
    goals: ['play_pro_football'],
    skills: { football_control: 70, speed: 72, mathematics: 42 },
    traits: { ambition: 0.8, discipline: 0.6, risk_tolerance: 0.5 },
  },
  {
    name: 'Aspiring Musician',
    description: '16yo in Lagos, Nigeria passionate about Afrobeats and songwriting.',
    icon: Music,
    countryId: 'country:real:nigeria',
    locationId: 'city:real:lagos',
    startingAge: 16,
    incomeTier: 'MIDDLE',
    interests: ['music', 'writing'],
    goals: ['become_musician'],
    skills: { singing: 68, songwriting: 62, communication: 60 },
    traits: { creativity: 0.85, sociability: 0.75, ambition: 0.7 },
  },
  {
    name: 'Young Political Organizer',
    description: '22yo in Paris, France dedicated to public policy and community organizing.',
    icon: Landmark,
    countryId: 'country:real:france',
    locationId: 'city:real:paris',
    startingAge: 22,
    incomeTier: 'MIDDLE',
    interests: ['politics', 'social_causes'],
    goals: ['become_prime_minister'],
    skills: { public_speaking: 72, persuasion: 70, history: 65 },
    traits: { ambition: 0.85, confidence: 0.8, empathy: 0.7 },
  },
  {
    name: 'Wall Street Accountant',
    description: '30yo in New York City seeking corporate wealth and financial independence.',
    icon: DollarSign,
    countryId: 'country:real:united_states',
    locationId: 'city:real:new_york',
    startingAge: 30,
    incomeTier: 'HIGH',
    interests: ['finance', 'business'],
    goals: ['become_wealthy'],
    skills: { finance_accounting: 80, mathematics: 78, management: 65 },
    traits: { discipline: 0.85, risk_tolerance: 0.6, ambition: 0.8 },
  },
  {
    name: 'Completely Random Life',
    description: 'Let the simulation engine generate a totally unique starting persona.',
    icon: Dices,
    countryId: 'country:real:united_kingdom',
    locationId: 'city:real:london',
    startingAge: 0,
    incomeTier: 'MIDDLE',
    interests: [],
    goals: [],
    skills: {},
    traits: {},
  },
];

export const QuickStartPresets: React.FC<QuickStartPresetsProps> = ({ onSelectPreset }) => {
  return (
    <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(240px, 1fr))', gap: '12px' }}>
      {presetsList.map((preset) => {
        const Icon = preset.icon;
        return (
          <button
            key={preset.name}
            onClick={() => onSelectPreset(preset)}
            style={{
              backgroundColor: 'var(--bg-surface-2)',
              border: '1px solid var(--border-subtle)',
              borderRadius: 'var(--radius-md)',
              padding: '16px',
              display: 'flex',
              flexDirection: 'column',
              gap: '8px',
              textAlign: 'left',
              cursor: 'pointer',
              transition: 'var(--transition-fast)',
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', color: 'var(--accent-indigo)', fontWeight: 600, fontSize: '14px' }}>
              <Icon size={18} />
              <span>{preset.name}</span>
            </div>
            <p style={{ fontSize: '12px', color: 'var(--text-secondary)', lineHeight: '1.4' }}>
              {preset.description}
            </p>
          </button>
        );
      })}
    </div>
  );
};
