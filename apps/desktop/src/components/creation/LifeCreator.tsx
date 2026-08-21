import React, { useState } from 'react';
import { ArrowRight, Dices, ArrowLeft, Globe, User, Home, Sparkles, Calendar, BookOpen } from 'lucide-react';

export interface NewLifeCreatorConfig {
  creation_mode: string;
  starting_year: number;
  country_id: string;
  location_id: string;
  starting_age: number;
  birth_year: number;
  birth_month: number;
  birth_day: number;
  first_name: string;
  last_name: string;
  sex: string;
  household_income_tier: string;
  mother_name?: string;
  mother_job?: string;
  father_name?: string;
  father_job?: string;
  custom_backstory?: string;
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
    defaultParents: {
      motherName: 'Sarah Adeyemi',
      motherJob: 'Healthcare Officer',
      fatherName: 'David Adeyemi',
      fatherJob: 'Civil Engineering Inspector',
    }
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
    defaultParents: {
      motherName: 'Fiona Sinclair',
      motherJob: 'Senior Architect',
      fatherName: 'Callum Sinclair',
      fatherJob: 'Secondary Educator',
    }
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
    defaultParents: {
      motherName: 'Elena Sterling',
      motherJob: 'Biotech Scientist',
      fatherName: 'Marcus Sterling',
      fatherJob: 'Systems Project Director',
    }
  },
];

const HOUSEHOLD_TIERS = [
  { id: 'WORKING_CLASS', label: 'Working Class', desc: 'Humble, tightly budgeted home where every opportunity is hard-earned.' },
  { id: 'MIDDLE', label: 'Middle Class', desc: 'Comfortable family home with books on shelves and resources for education.' },
  { id: 'UPPER_MIDDLE', label: 'Upper-Middle Class', desc: 'Substantial family resources, private mentorship, and deep community roots.' },
];

const STARTING_AGES = [
  { age: 0, label: 'Age 0 · Newborn Infancy', desc: 'Experience life from your very first moments and nursery surroundings.' },
  { age: 6, label: 'Age 6 · Primary School', desc: 'Begin as a young child entering classrooms and neighborhood playgrounds.' },
  { age: 11, label: 'Age 11 · Secondary School', desc: 'Jump in as an adolescent facing exam revisions, sports trials, and friendships.' },
  { age: 16, label: 'Age 16 · Senior Youth', desc: 'Prepare for national higher examinations, talent scouting, and early ambitions.' },
  { age: 18, label: 'Age 18 · Early Adulthood', desc: 'Enter adult independence, university admissions, and career pathways.' },
  { age: 25, label: 'Age 25 · Established Professional', desc: 'Begin as an independent adult building businesses, careers, and wealth.' },
];

const MONTH_NAMES = [
  'January', 'February', 'March', 'April', 'May', 'June',
  'July', 'August', 'September', 'October', 'November', 'December'
];

export const LifeCreator: React.FC<LifeCreatorProps> = ({ onBeginLife, onCancel }) => {
  const [selectedCountryIndex, setSelectedCountryIndex] = useState(0);
  const [selectedCityId, setSelectedCityId] = useState(COUNTRIES[0].cities[0].id);
  const [sex, setSex] = useState<'Male' | 'Female'>('Male');
  const [firstName, setFirstName] = useState(COUNTRIES[0].sampleFirstNames.Male[0]);
  const [lastName, setLastName] = useState(COUNTRIES[0].sampleLastNames[0]);
  
  // Date of Birth & Starting Age
  const [birthYear, setBirthYear] = useState(1998);
  const [birthMonth, setBirthMonth] = useState(6);
  const [birthDay, setBirthDay] = useState(14);
  const [startingAge, setStartingAge] = useState(0);

  const [householdTier, setHouseholdTier] = useState('MIDDLE');

  // Custom Parents & Backstory (for Starting Age > 0)
  const currentCountry = COUNTRIES[selectedCountryIndex];
  const [motherName, setMotherName] = useState(currentCountry.defaultParents.motherName);
  const [motherJob, setMotherJob] = useState(currentCountry.defaultParents.motherJob);
  const [fatherName, setFatherName] = useState(currentCountry.defaultParents.fatherName);
  const [fatherJob, setFatherJob] = useState(currentCountry.defaultParents.fatherJob);
  const [customBackstory, setCustomBackstory] = useState(
    'Raised in a supportive household with an emphasis on curiosity, academic diligence, and integrity.'
  );

  const simStartingYear = birthYear + startingAge;

  const handleCountryChange = (idx: number) => {
    setSelectedCountryIndex(idx);
    const country = COUNTRIES[idx];
    setSelectedCityId(country.cities[0].id);
    const fNames = sex === 'Female' ? country.sampleFirstNames.Female : country.sampleFirstNames.Male;
    setFirstName(fNames[0]);
    setLastName(country.sampleLastNames[0]);
    setMotherName(country.defaultParents.motherName);
    setMotherJob(country.defaultParents.motherJob);
    setFatherName(country.defaultParents.fatherName);
    setFatherJob(country.defaultParents.fatherJob);
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
    const lName = country.sampleLastNames[Math.floor(Math.random() * country.sampleLastNames.length)];
    setLastName(lName);
    const tiers = ['WORKING_CLASS', 'MIDDLE', 'UPPER_MIDDLE'];
    setHouseholdTier(tiers[Math.floor(Math.random() * tiers.length)]);
    
    const yr = 1990 + Math.floor(Math.random() * 20);
    setBirthYear(yr);
    setBirthMonth(1 + Math.floor(Math.random() * 12));
    setBirthDay(1 + Math.floor(Math.random() * 28));

    setMotherName(country.defaultParents.motherName);
    setMotherJob(country.defaultParents.motherJob);
    setFatherName(country.defaultParents.fatherName);
    setFatherJob(country.defaultParents.fatherJob);
  };

  const handleStart = () => {
    const config: NewLifeCreatorConfig = {
      creation_mode: 'CUSTOM',
      starting_year: simStartingYear,
      country_id: currentCountry.id,
      location_id: selectedCityId,
      starting_age: startingAge,
      birth_year: birthYear,
      birth_month: birthMonth,
      birth_day: birthDay,
      first_name: firstName.trim() || 'Alex',
      last_name: lastName.trim() || 'Sterling',
      sex,
      household_income_tier: householdTier,
      mother_name: motherName.trim() || currentCountry.defaultParents.motherName,
      mother_job: motherJob.trim() || currentCountry.defaultParents.motherJob,
      father_name: fatherName.trim() || currentCountry.defaultParents.fatherName,
      father_job: fatherJob.trim() || currentCountry.defaultParents.fatherJob,
      custom_backstory: startingAge > 0 ? customBackstory.trim() : undefined,
      traits: {},
      skills: {},
      interests: ['curiosity', 'academics'],
      goals: ['discovery'],
    };
    onBeginLife(config);
  };

  return (
    <div className="w-full max-w-3xl bg-[#0e1118] border border-amber-500/30 rounded-3xl p-8 shadow-2xl space-y-6 text-slate-100 font-sans max-h-[90vh] overflow-y-auto animate-fadeIn">
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

      {/* 1. Identity */}
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

      {/* 2. Exact Date of Birth & Starting Age */}
      <div className="space-y-3 pt-2 border-t border-[#1c2130]">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-xs font-serif text-amber-300/90">
            <Calendar className="w-3.5 h-3.5 text-amber-400" />
            <span>Date of Birth & Starting Age</span>
          </div>
          <span className="text-[11px] font-mono text-amber-400 bg-amber-500/10 px-2.5 py-0.5 rounded-full border border-amber-500/20">
            Timeline Starts: {MONTH_NAMES[birthMonth - 1]} {birthDay}, {simStartingYear}
          </span>
        </div>

        {/* Date Pickers */}
        <div className="grid grid-cols-3 gap-3">
          <div>
            <label className="block text-[11px] font-sans text-slate-400 mb-1">Birth Month</label>
            <select
              value={birthMonth}
              onChange={(e) => setBirthMonth(Number(e.target.value))}
              className="w-full bg-[#121622] border border-[#20273a] focus:border-amber-500/60 rounded-xl px-3 py-2 text-xs text-slate-100 font-serif focus:outline-none"
            >
              {MONTH_NAMES.map((m, idx) => (
                <option key={idx} value={idx + 1}>{m}</option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-[11px] font-sans text-slate-400 mb-1">Birth Day</label>
            <select
              value={birthDay}
              onChange={(e) => setBirthDay(Number(e.target.value))}
              className="w-full bg-[#121622] border border-[#20273a] focus:border-amber-500/60 rounded-xl px-3 py-2 text-xs text-slate-100 font-serif focus:outline-none"
            >
              {Array.from({ length: 31 }, (_, i) => i + 1).map((d) => (
                <option key={d} value={d}>{d}</option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-[11px] font-sans text-slate-400 mb-1">Birth Year</label>
            <input
              type="number"
              value={birthYear}
              min={1960}
              max={2026}
              onChange={(e) => setBirthYear(Number(e.target.value))}
              className="w-full bg-[#121622] border border-[#20273a] focus:border-amber-500/60 rounded-xl px-3 py-2 text-xs text-slate-100 font-mono focus:outline-none"
            />
          </div>
        </div>

        {/* Starting Age Selector */}
        <div className="space-y-1.5 pt-1">
          <label className="block text-[11px] font-sans text-slate-400">At what stage of life will you begin?</label>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-2.5">
            {STARTING_AGES.map((st) => (
              <button
                key={st.age}
                type="button"
                onClick={() => setStartingAge(st.age)}
                className={`p-3 rounded-2xl border text-left transition-all ${
                  startingAge === st.age
                    ? 'bg-amber-500/20 border-amber-500/60 text-amber-200 shadow-md'
                    : 'bg-[#121622] border-[#20273a] text-slate-400 hover:text-slate-200'
                }`}
              >
                <div className="font-serif font-bold text-xs">{st.label}</div>
                <div className="text-[10px] text-slate-500 font-sans mt-0.5 leading-snug">{st.desc}</div>
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* 3. Custom Parents & Backstory (if Starting Age > 0) */}
      {startingAge > 0 && (
        <div className="space-y-3 pt-2 border-t border-[#1c2130] bg-[#121622]/50 p-4 rounded-2xl border border-[#20273a]">
          <div className="flex items-center gap-2 text-xs font-serif text-amber-300">
            <BookOpen className="w-3.5 h-3.5 text-amber-400" />
            <span>Family Background & Childhood Backstory (Pre-Game History)</span>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div>
              <label className="block text-[10px] font-sans text-slate-400 mb-1">Mother's Name & Career</label>
              <div className="flex gap-2">
                <input
                  type="text"
                  placeholder="Mother's Name"
                  value={motherName}
                  onChange={(e) => setMotherName(e.target.value)}
                  className="w-1/2 bg-[#121622] border border-[#20273a] rounded-xl px-3 py-1.5 text-xs text-slate-100 font-serif"
                />
                <input
                  type="text"
                  placeholder="Mother's Job"
                  value={motherJob}
                  onChange={(e) => setMotherJob(e.target.value)}
                  className="w-1/2 bg-[#121622] border border-[#20273a] rounded-xl px-3 py-1.5 text-xs text-slate-100 font-serif"
                />
              </div>
            </div>

            <div>
              <label className="block text-[10px] font-sans text-slate-400 mb-1">Father's Name & Career</label>
              <div className="flex gap-2">
                <input
                  type="text"
                  placeholder="Father's Name"
                  value={fatherName}
                  onChange={(e) => setFatherName(e.target.value)}
                  className="w-1/2 bg-[#121622] border border-[#20273a] rounded-xl px-3 py-1.5 text-xs text-slate-100 font-serif"
                />
                <input
                  type="text"
                  placeholder="Father's Job"
                  value={fatherJob}
                  onChange={(e) => setFatherJob(e.target.value)}
                  className="w-1/2 bg-[#121622] border border-[#20273a] rounded-xl px-3 py-1.5 text-xs text-slate-100 font-serif"
                />
              </div>
            </div>
          </div>

          <div>
            <label className="block text-[10px] font-sans text-slate-400 mb-1">Childhood Upbringing & Narrative Notes</label>
            <textarea
              rows={2}
              value={customBackstory}
              onChange={(e) => setCustomBackstory(e.target.value)}
              className="w-full bg-[#121622] border border-[#20273a] rounded-xl p-2.5 text-xs text-slate-200 font-serif focus:outline-none focus:border-amber-500/60"
            />
          </div>
        </div>
      )}

      {/* 4. Geography */}
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

      {/* 5. Household Reality */}
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

      {/* Action Navigation */}
      <div className="flex items-center justify-between pt-4 border-t border-[#1c2130]">
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
          <span>Begin Life as {firstName} {lastName} ({startingAge === 0 ? 'Infant' : `Age ${startingAge}`})</span>
          <ArrowRight className="w-4 h-4" />
        </button>
      </div>
    </div>
  );
};
