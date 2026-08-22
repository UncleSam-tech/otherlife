import React, { useState } from 'react';
import { ArrowLeft, BookOpen, CheckCircle2, GraduationCap, WalletCards, X } from 'lucide-react';
import { StructuredGameplayAction } from '../../types/gameplay';

interface UniversityApplicationModalProps {
  institution: string;
  currencySymbol: string;
  isLoading: boolean;
  onClose: () => void;
  onStructuredAction: (action: StructuredGameplayAction) => Promise<boolean>;
}

const programmes: Record<string, string[]> = {
  'BSc Computer Science': ['Software Engineering', 'Artificial Intelligence', 'Cybersecurity', 'Data Systems'],
  'BSc Business Administration': ['Entrepreneurship', 'Finance', 'Marketing', 'Operations Management'],
  'BA Politics & International Relations': ['Comparative Politics', 'Public Policy', 'Diplomacy', 'Political Economy'],
  'BEng Civil Engineering': ['Structural Engineering', 'Transport Systems', 'Construction Management', 'Water Engineering'],
};
const studyModes = ['Full-time campus', 'Part-time evening', 'Blended learning'];
const fundingPlans = ['Family sponsorship', 'Scholarship application', 'Student loan', 'Self-funded'];

export const UniversityApplicationModal: React.FC<UniversityApplicationModalProps> = ({ institution, currencySymbol, isLoading, onClose, onStructuredAction }) => {
  const [step, setStep] = useState(0);
  const [degreeProgram, setDegreeProgram] = useState(Object.keys(programmes)[0]);
  const [primaryCourse, setPrimaryCourse] = useState(programmes[Object.keys(programmes)[0]][0]);
  const [studyMode, setStudyMode] = useState(studyModes[0]);
  const [fundingPlan, setFundingPlan] = useState(fundingPlans[0]);

  const selectProgramme = (programme: string) => {
    setDegreeProgram(programme);
    setPrimaryCourse(programmes[programme][0]);
  };
  const submit = async () => {
    const success = await onStructuredAction({ type: 'UNIVERSITY_APPLICATION', institution, degreeProgram, primaryCourse, studyMode, fundingPlan });
    if (success) onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4 text-slate-100 backdrop-blur-sm">
      <section className="flex h-[680px] w-full max-w-3xl flex-col gap-5 rounded-3xl border border-violet-400/30 bg-[#0a0d16] p-6 shadow-2xl" aria-label="University programme application">
        <header className="flex items-center justify-between border-b border-[#22283a] pb-4"><div className="flex items-center gap-3"><GraduationCap className="h-6 w-6 text-violet-300" /><div><h3 className="font-serif text-lg font-bold">{institution}</h3><p className="text-[11px] text-slate-400">Programme selection and admissions</p></div></div><button type="button" onClick={onClose} aria-label="Close university application" className="p-2 text-slate-400 hover:text-white"><X className="h-4 w-4" /></button></header>
        <ol className="grid grid-cols-3 gap-2">{['Programme', 'Study plan', 'Review'].map((label, index) => <li key={label} className={`rounded-lg border px-2 py-2 text-center text-[10px] font-mono uppercase ${index <= step ? 'border-violet-300/50 bg-violet-300/10 text-violet-200' : 'border-[#293149] text-slate-600'}`}>{index + 1}. {label}</li>)}</ol>
        <div className="flex-1 overflow-y-auto">
          {step === 0 ? <div className="space-y-5"><div><p className="text-[10px] font-mono uppercase text-violet-300">Choose a degree</p><p className="mt-1 text-xs text-slate-400">Enrollment cannot begin until you choose what you intend to study.</p></div><div className="grid grid-cols-2 gap-3">{Object.keys(programmes).map((programme) => <button key={programme} type="button" onClick={() => selectProgramme(programme)} aria-pressed={degreeProgram === programme} className={`rounded-2xl border p-4 text-left ${degreeProgram === programme ? 'border-violet-300 bg-violet-300/10' : 'border-[#293149] bg-[#111622] hover:border-violet-300/50'}`}><BookOpen className="h-5 w-5 text-violet-300" /><p className="mt-3 font-serif text-sm font-bold">{programme}</p><p className="mt-1 text-[11px] text-slate-500">{programmes[programme].length} specialisations</p></button>)}</div><fieldset className="space-y-2"><legend className="text-xs text-slate-300">Primary course / specialisation</legend><div className="grid grid-cols-2 gap-2">{programmes[degreeProgram].map((course) => <button key={course} type="button" onClick={() => setPrimaryCourse(course)} aria-pressed={primaryCourse === course} className={`rounded-xl border px-3 py-3 text-left text-xs ${primaryCourse === course ? 'border-violet-300 bg-violet-300/10 text-violet-100' : 'border-[#293149] bg-[#111622] text-slate-300'}`}>{course}</button>)}</div></fieldset><button type="button" onClick={() => setStep(1)} className="w-full rounded-xl bg-violet-400 py-3 text-xs font-bold text-slate-950">Continue with this programme</button></div> : null}
          {step === 1 ? <div className="space-y-6"><fieldset className="space-y-2"><legend className="text-xs text-slate-300">How will you study?</legend>{studyModes.map((mode) => <button key={mode} type="button" onClick={() => setStudyMode(mode)} aria-pressed={studyMode === mode} className={`w-full rounded-xl border p-4 text-left text-xs ${studyMode === mode ? 'border-violet-300 bg-violet-300/10' : 'border-[#293149] bg-[#111622]'}`}>{mode}</button>)}</fieldset><fieldset className="space-y-2"><legend className="text-xs text-slate-300">Funding plan</legend><div className="grid grid-cols-2 gap-2">{fundingPlans.map((plan) => <button key={plan} type="button" onClick={() => setFundingPlan(plan)} aria-pressed={fundingPlan === plan} className={`rounded-xl border p-4 text-left text-xs ${fundingPlan === plan ? 'border-violet-300 bg-violet-300/10' : 'border-[#293149] bg-[#111622]'}`}><WalletCards className="mb-2 h-4 w-4 text-violet-300" />{plan}</button>)}</div></fieldset><div className="flex gap-3"><button type="button" onClick={() => setStep(0)} className="rounded-xl border border-[#293149] px-4 text-xs">Back</button><button type="button" onClick={() => setStep(2)} className="flex-1 rounded-xl bg-violet-400 py-3 text-xs font-bold text-slate-950">Review application</button></div></div> : null}
          {step === 2 ? <div className="space-y-5"><div className="flex items-center gap-2 text-violet-200"><CheckCircle2 className="h-5 w-5" /><h4 className="font-serif font-bold">Review before submission</h4></div><dl className="grid grid-cols-2 gap-4 rounded-2xl border border-[#293149] bg-[#111622] p-5 text-xs"><div><dt className="text-slate-500">Institution</dt><dd className="mt-1">{institution}</dd></div><div><dt className="text-slate-500">Degree</dt><dd className="mt-1">{degreeProgram}</dd></div><div><dt className="text-slate-500">Primary course</dt><dd className="mt-1">{primaryCourse}</dd></div><div><dt className="text-slate-500">Study mode</dt><dd className="mt-1">{studyMode}</dd></div><div><dt className="text-slate-500">Funding</dt><dd className="mt-1">{fundingPlan}</dd></div><div><dt className="text-slate-500">Application fee</dt><dd className="mt-1">{currencySymbol}25.00</dd></div></dl><p className="rounded-xl border border-amber-400/20 bg-amber-400/5 p-4 text-xs text-amber-100">Submitting starts a six-stage process: records review, decision, offer, acceptance, enrollment, and timetable. It does not instantly place you in university.</p><div className="flex gap-3"><button type="button" onClick={() => setStep(1)} className="flex items-center gap-2 rounded-xl border border-[#293149] px-4 text-xs"><ArrowLeft className="h-4 w-4" />Edit</button><button type="button" onClick={submit} disabled={isLoading} className="flex-1 rounded-xl bg-violet-400 py-3 text-xs font-bold text-slate-950 disabled:opacity-40">Pay fee and submit application</button></div></div> : null}
        </div>
      </section>
    </div>
  );
};
