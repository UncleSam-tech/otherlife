import React, { useState } from 'react';
import { ArrowRight, Dices, ArrowLeft, Globe, User, Home, Sparkles } from 'lucide-react';

export interface NewLifeCreatorConfig {
  creation_mode: string;
  starting_year: number;
  country_id: string;
  location_id: string;
  starting_age: number;
  first_name: string;
  last_name: string;
  sex: string;
  household_income_tier: string;
  traits: Record<string, number>;
  skills: Record<string, number>;
  interests: string[];
  goals: string[];
}

interface LifeCreatorProps {
  onBeginLife: (config: NewLifeCreatorConfig) => void;
  onCancel: () => void;
}

const COUNTRIES = [
  {
    id: 'country:real:nigeria',
    name: 'Nigeria',
    currency: '₦',
    cities: [
      { id: 'city:real:lagos', name: 'Lagos', district: 'Ikeja' },
      { id: 'city:real:abuja', name: 'Abuja', district: 'Garki' },
      { id: 'city:real:ibadan', name: 'Ibadan', district: 'Bodija' },
      { id: 'city:real:port_harcourt', name: 'Port Harcourt', district: 'Old GRA' },
      { id: 'city:real:kano', name: 'Kano', district: 'Nasarawa' },
      { id: 'city:real:enugu', name: 'Enugu', district: 'Independence Layout' },
    ],
    sampleFirstNames: { Male: ['Tunde', 'Israel', 'Chidi', 'Ibrahim', 'Emeka', 'Femi'], Female: ['Funke', 'Sarah', 'Amina', 'Nkechi', 'Blessing', 'Yetunde'] },
    sampleLastNames: ['Adeyemi', 'Oyebamiji', 'Nwosu', 'Bello', 'Okoye', 'Adeleke', 'Briggs'],
  },
  {
    id: 'country:real:united_kingdom',
    name: 'United Kingdom',
    currency: '£',
    cities: [
      { id: 'city:real:london', name: 'London', district: 'Camden' },
      { id: 'city:real:glasgow', name: 'Glasgow', district: 'West End' },
      { id: 'city:real:manchester', name: 'Manchester', district: 'Didsbury' },
      { id: 'city:real:birmingham', name: 'Birmingham', district: 'Edgbaston' },
      { id: 'city:real:edinburgh', name: 'Edinburgh', district: 'Old Town' },
    ],
    sampleFirstNames: { Male: ['Callum', 'Liam', 'Arthur', 'Jack', 'Oliver', 'Harry'], Female: ['Fiona', 'Emma', 'Isobel', 'Claire', 'Gemma', 'Charlotte'] },
    sampleLastNames: ['Sinclair', 'Robertson', 'MacLeod', 'Harrison', 'Taylor', 'Campbell', 'Lewis'],
  },
  {
    id: 'country:real:united_states',
    name: 'United States',
    currency: '$',
    cities: [
      { id: 'city:real:new_york', name: 'New York', district: 'Brooklyn' },
      { id: 'city:real:san_francisco', name: 'San Francisco', district: 'Sunset' },
      { id: 'city:real:los_angeles', name: 'Los Angeles', district: 'Silver Lake' },
      { id: 'city:real:chicago', name: 'Chicago', district: 'Lincoln Park' },
      { id: 'city:real:houston', name: 'Houston', district: 'Montrose' },
    ],
    sampleFirstNames: { Male: ['Marcus', 'Ethan', 'Daniel', 'Lucas', 'Noah', 'Benjamin'], Female: ['Elena', 'Maya', 'Rachel', 'Laura', 'Sarah', 'Chloe'] },
    sampleLastNames: ['Sterling', 'Vance', 'Lin', 'Rivera', 'Murphy', 'Brooks', 'Hayes'],
  },
];

const HOUSEHOLD_TIERS = [
  { id: 'WORKING_CLASS', label: 'Working Class', desc: 'Humble, tightly budgeted home where every opportunity is hard-earned.' },
  { id: 'MIDDLE', label: 'Middle Class', desc: 'Comfortable family home with books on shelves and resources for education.' },
  { id: 'UPPER_MIDDLE', label: 'Upper-Middle Class', desc: 'Substantial family resources, private mentorship, and deep community roots.' },
];

export const LifeCreator: React.FC<LifeCreatorProps> = ({ onBeginLife, onCancel }) => {
  const [selectedCountryIndex, setSelectedCountryIndex] = useState(0);
  const [selectedCityId, setSelectedCityId] = useState(COUNTRIES[0].cities[0].id);
  const [sex, setSex] = useState<'Male' | 'Female'>('Male');
  const [firstName, setFirstName] = useState(COUNTRIES[0].sampleFirstNames.Male[0]);
  const [lastName, setLastName] = useState(COUNTRIES[0].sampleLastNames[0]);
  const [birthYear, setBirthYear] = useState(2005);
  const [householdTier, setHouseholdTier] = useState('MIDDLE');

  const currentCountry = COUNTRIES[selectedCountryIndex];

  const handleCountryChange = (idx: number) => {
    setSelectedCountryIndex(idx);
    const country = COUNTRIES[idx];
    setSelectedCityId(country.cities[0].id);
    const fNames = sex === 'Female' ? country.sampleFirstNames.Female : country.sampleFirstNames.Male;
    setFirstName(fNames[0]);
    setLastName(country.sampleLastNames[0]);
  };

  const handleRandomize = () => {
    const cIdx = Math.floor(Math.random() * COUNTRIES.length);
    setSelectedCountryIndex(cIdx);
    const country = COUNTRIES[cIdx];
    const city = country.cities[Math.floor(Math.random() * country.cities.length)];
    setSelectedCityId(city.id);
    const randomSex: 'Male' | 'Female' = Math.random() > 0.5 ? 'Male' : 'Female';
    setSex(randomSex);
    const fNames = randomSex === 'Female' ? country.sampleFirstNames.Female : country.sampleFirstNames.Male;
    setFirstName(fNames[Math.floor(Math.random() * fNames.length)]);
    setLastName(country.sampleLastNames[Math.floor(Math.random() * country.sampleLastNames.length)]);
    const tiers = ['WORKING_CLASS', 'MIDDLE', 'UPPER_MIDDLE'];
    setHouseholdTier(tiers[Math.floor(Math.random() * tiers.length)]);
    setBirthYear(1995 + Math.floor(Math.random() * 16));
  };

  const handleStart = () => {
    const config: NewLifeCreatorConfig = {
      creation_mode: 'CUSTOM',
      starting_year: birthYear,
      country_id: currentCountry.id,
      location_id: selectedCityId,
      starting_age: 0,
      first_name: firstName.trim() || 'Alex',
      last_name: lastName.trim() || 'Sterling',
      sex,
      household_income_tier: householdTier,
      traits: {},
      skills: {},
      interests: ['curiosity', 'academics'],
      goals: ['discovery'],
    };
    onBeginLife(config);
  };

  return (
    <div className="w-full max-w-2xl bg-[#0e1118] border border-amber-500/30 rounded-3xl p-8 shadow-2xl space-y-6 text-slate-100 font-sans animate-fadeIn">
      {/* Prologue Header */}
      <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
        <div>
          <div className="flex items-center gap-1.5 text-xs uppercase tracking-widest text-amber-400 font-mono">
            <Sparkles className="w-3.5 h-3.5" />
            <span>Character Genesis</span>
          </div>
          <h2 className="text-2xl font-serif font-bold text-slate-100 mt-1">Every life begins somewhere...</h2>
        </div>

        <button
          type="button"
          onClick={handleRandomize}
          className="flex items-center gap-2 bg-[#141824] hover:bg-amber-950/40 border border-[#22283a] hover:border-amber-500/40 text-amber-300 px-4 py-2 rounded-xl text-xs font-serif transition-all shadow-sm"
          title="Let Fate Decide Everything"
        >
          <Dices className="w-4 h-4 text-amber-400" />
          <span>Let Fate Decide</span>
        </button>
      </div>

      {/* Identity: Name & Sex */}
      <div className="space-y-3">
        <div className="flex items-center gap-2 text-xs font-serif text-amber-300/90">
          <User className="w-3.5 h-3.5 text-amber-400" />
          <span>Identity</span>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
          <div>
            <label className="block text-[11px] font-sans text-slate-400 mb-1">First Name</label>
            <input
              type="text"
              value={firstName}
              onChange={(e) => setFirstName(e.target.value)}
              className="w-full bg-[#121622] border border-[#20273a] focus:border-amber-500/60 rounded-xl px-3.5 py-2 text-sm text-slate-100 font-serif focus:outline-none"
            />
          </div>

          <div>
            <label className="block text-[11px] font-sans text-slate-400 mb-1">Family Name</label>
            <input
              type="text"
              value={lastName}
              onChange={(e) => setLastName(e.target.value)}
              className="w-full bg-[#121622] border border-[#20273a] focus:border-amber-500/60 rounded-xl px-3.5 py-2 text-sm text-slate-100 font-serif focus:outline-none"
            />
          </div>

          <div>
            <label className="block text-[11px] font-sans text-slate-400 mb-1">Sex</label>
            <div className="flex gap-2">
              {(['Male', 'Female'] as const).map((s) => (
                <button
                  key={s}
                  type="button"
                  onClick={() => setSex(s)}
                  className={`flex-1 py-2 rounded-xl text-xs font-serif transition-all ${
                    sex === s
                      ? 'bg-amber-500/20 text-amber-300 border border-amber-500/50 font-bold'
                      : 'bg-[#121622] border border-[#20273a] text-slate-400 hover:text-slate-200'
                  }`}
                >
                  {s}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Geography: Country & City */}
      <div className="space-y-3 pt-2 border-t border-[#1c2130]">
        <div className="flex items-center gap-2 text-xs font-serif text-amber-300/90">
          <Globe className="w-3.5 h-3.5 text-amber-400" />
          <span>Birthplace & Origin</span>
        </div>
        
        {/* Country Selector */}
        <div className="grid grid-cols-3 gap-3">
          {COUNTRIES.map((c, idx) => (
            <button
              key={c.id}
              type="button"
              onClick={() => handleCountryChange(idx)}
              className={`p-3 rounded-2xl border text-left transition-all ${
                selectedCountryIndex === idx
                  ? 'bg-amber-500/15 border-amber-500/50 text-amber-200 shadow-md'
                  : 'bg-[#121622] border-[#20273a] text-slate-400 hover:text-slate-200'
              }`}
            >
              <div className="font-serif font-semibold text-sm">{c.name}</div>
              <div className="text-[10px] text-slate-500 font-mono mt-0.5">Currency: {c.currency}</div>
            </button>
          ))}
        </div>

        {/* City Selector */}
        <div className="flex flex-wrap gap-2 pt-1">
          {currentCountry.cities.map((city) => (
            <button
              key={city.id}
              type="button"
              onClick={() => setSelectedCityId(city.id)}
              className={`px-3 py-1 rounded-full text-xs font-serif transition-all ${
                selectedCityId === city.id
                  ? 'bg-amber-400 text-slate-950 font-bold shadow-sm'
                  : 'bg-[#121622] text-slate-300 border border-[#20273a] hover:border-slate-600'
              }`}
            >
              {city.name} ({city.district})
            </button>
          ))}
        </div>
      </div>

      {/* Household Circumstances */}
      <div className="space-y-3 pt-2 border-t border-[#1c2130]">
        <div className="flex items-center gap-2 text-xs font-serif text-amber-300/90">
          <Home className="w-3.5 h-3.5 text-amber-400" />
          <span>Family & Household Reality</span>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
          {HOUSEHOLD_TIERS.map((tier) => (
            <button
              key={tier.id}
              type="button"
              onClick={() => setHouseholdTier(tier.id)}
              className={`p-3 rounded-2xl border text-left transition-all ${
                householdTier === tier.id
                  ? 'bg-amber-500/15 border-amber-500/50 text-amber-200 shadow-md'
                  : 'bg-[#121622] border-[#20273a] text-slate-400 hover:text-slate-200'
              }`}
            >
              <div className="font-serif font-bold text-xs">{tier.label}</div>
              <div className="text-[10px] text-slate-400 font-sans mt-1 leading-snug">{tier.desc}</div>
            </button>
          ))}
        </div>
      </div>

      {/* Era */}
      <div className="flex items-center justify-between pt-2 border-t border-[#1c2130]">
        <span className="text-xs font-serif text-slate-400">Birth Year / Era</span>
        <div className="flex gap-2">
          {[1995, 2000, 2005, 2010].map((yr) => (
            <button
              key={yr}
              type="button"
              onClick={() => setBirthYear(yr)}
              className={`px-3 py-1 rounded-lg text-xs font-mono transition-all ${
                birthYear === yr
                  ? 'bg-amber-400 text-slate-950 font-bold'
                  : 'bg-[#121622] text-slate-400 border border-[#20273a] hover:text-slate-200'
              }`}
            >
              {yr}
            </button>
          ))}
        </div>
      </div>

      {/* Action Navigation */}
      <div className="flex items-center justify-between pt-3 border-t border-[#1c2130]">
        <button
          type="button"
          onClick={onCancel}
          className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-slate-200 font-serif"
        >
          <ArrowLeft className="w-3.5 h-3.5" />
          <span>Back to Menu</span>
        </button>

        <button
          type="button"
          onClick={handleStart}
          className="flex items-center gap-2 bg-amber-500 hover:bg-amber-400 text-slate-950 font-serif font-bold px-6 py-2.5 rounded-2xl text-sm shadow-xl shadow-amber-500/20 transition-all hover:scale-105"
        >
          <span>Begin Life as {firstName} {lastName}</span>
          <ArrowRight className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
};
