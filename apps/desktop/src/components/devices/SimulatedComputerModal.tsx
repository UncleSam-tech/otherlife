import React, { useState } from 'react';
import {
  ArrowLeft,
  ArrowRight,
  Briefcase,
  Building2,
  CheckCircle,
  ClipboardCheck,
  FilePenLine,
  Globe,
  Laptop,
  Search,
  ShieldCheck,
  Terminal,
  X,
} from 'lucide-react';
import { StructuredGameplayAction } from '../../types/gameplay';

interface SimulatedComputerModalProps {
  onClose: () => void;
  playerAge?: number;
  currencySymbol: string;
  onExecuteAction: (intent: string) => void;
  onStructuredAction: (action: StructuredGameplayAction) => Promise<boolean>;
  isLoading: boolean;
  ownedCompanyName?: string;
}

interface JobListing {
  id: string;
  title: string;
  company: string;
  salary: number;
  requirement: string;
  location: string;
  description: string;
  responsibilities: string[];
}

const jobListings: JobListing[] = [
  {
    id: 'job_dev',
    title: 'Junior Software Engineer',
    company: 'Apex Digital Systems',
    salary: 250000,
    requirement: 'Algorithms, debugging, and clear technical communication',
    location: 'Hybrid · Central Business District',
    description: 'Join a product engineering team building civic and commercial software used across the region.',
    responsibilities: ['Implement reviewed product features', 'Investigate defects with senior engineers', 'Write tests and technical notes'],
  },
  {
    id: 'job_analyst',
    title: 'Commercial Operations Analyst',
    company: 'Global Capital Advisory',
    salary: 220000,
    requirement: 'Financial analysis, spreadsheets, and diligence',
    location: 'On-site · Financial District',
    description: 'Support transaction teams with market research, operating models, and decision-ready analysis.',
    responsibilities: ['Build operating reports', 'Research counterparties and markets', 'Prepare weekly review materials'],
  },
  {
    id: 'job_admin',
    title: 'Executive Administrative Officer',
    company: 'Civic Infrastructure Ltd',
    salary: 180000,
    requirement: 'Communication, scheduling, and organization',
    location: 'On-site · Government Quarter',
    description: 'Coordinate executive schedules, correspondence, records, and cross-functional meetings.',
    responsibilities: ['Own executive calendar logistics', 'Prepare correspondence and minutes', 'Maintain confidential records'],
  },
];

const businessOperations = [
  { id: 'Recruit a team member', prompt: 'Define the role, salary range, interview questions, and what evidence would make you hire the candidate.' },
  { id: 'Pitch an investor', prompt: 'State the customer problem, evidence of demand, amount requested, proposed terms, and how you would answer “Why you?”' },
  { id: 'Develop a product', prompt: 'Define the customer, problem, product or service, price, delivery plan, and first measurable milestone.' },
  { id: 'Win a customer', prompt: 'Describe the prospect, their need, your offer, price, negotiation boundary, and follow-up plan.' },
];

const StepDots: React.FC<{ current: number; labels: string[] }> = ({ current, labels }) => (
  <ol className="grid grid-cols-3 gap-2" aria-label="Application progress">
    {labels.map((label, index) => (
      <li key={label} className={`rounded-lg border px-2 py-1.5 text-center text-[9px] font-mono uppercase ${index <= current ? 'border-amber-400/40 bg-amber-400/10 text-amber-200' : 'border-[#263049] text-slate-600'}`}>
        {index + 1}. {label}
      </li>
    ))}
  </ol>
);

export const SimulatedComputerModal: React.FC<SimulatedComputerModalProps> = ({
  onClose,
  playerAge = 0,
  currencySymbol,
  onExecuteAction,
  onStructuredAction,
  isLoading,
  ownedCompanyName,
}) => {
  const [activeWindow, setActiveWindow] = useState<'desktop' | 'jobs' | 'incorporation' | 'business' | 'code' | 'research'>('desktop');
  const [selectedJob, setSelectedJob] = useState<JobListing | null>(null);
  const [jobStep, setJobStep] = useState(0);
  const [resumeSummary, setResumeSummary] = useState('');
  const [coverLetter, setCoverLetter] = useState('');
  const [availability, setAvailability] = useState('Two weeks after an offer');

  const [companyStep, setCompanyStep] = useState(0);
  const [companyName, setCompanyName] = useState('');
  const [businessActivity, setBusinessActivity] = useState('');
  const [registeredAddress, setRegisteredAddress] = useState('');
  const [structure, setStructure] = useState('Limited Liability Company (LLC)');
  const [capital, setCapital] = useState('1,000,000');
  const [partners, setPartners] = useState('');

  const [codeDraft, setCodeDraft] = useState('// Write a small program or technical note here.\n');
  const [researchQuery, setResearchQuery] = useState('');
  const [researchSubmitted, setResearchSubmitted] = useState(false);
  const [businessStep, setBusinessStep] = useState(0);
  const [selectedOperation, setSelectedOperation] = useState(businessOperations[0]);
  const [operationPlan, setOperationPlan] = useState('');

  const resetJobs = () => {
    setSelectedJob(null);
    setJobStep(0);
  };

  const submitJobApplication = async () => {
    if (!selectedJob || !resumeSummary.trim() || !coverLetter.trim()) return;
    const success = await onStructuredAction({
      type: 'APPLY_FOR_JOB',
      jobId: selectedJob.id,
      companyId: `company:${selectedJob.id}`,
      title: selectedJob.title,
      companyName: selectedJob.company,
      resumeSummary: resumeSummary.trim(),
      coverLetter: coverLetter.trim(),
      availability,
    });
    if (success) onClose();
  };

  const submitCompanyRegistration = async () => {
    const numericCapital = Number(capital.replace(/,/g, ''));
    const success = await onStructuredAction({
      type: 'REGISTER_COMPANY',
      name: companyName.trim(),
      structure,
      partners: partners.split(',').map((name) => name.trim()).filter(Boolean),
      authorizedCapital: Number.isFinite(numericCapital) ? numericCapital : 0,
      businessActivity: businessActivity.trim(),
      registeredAddress: registeredAddress.trim(),
    });
    if (success) onClose();
  };

  const backToDesktop = () => {
    setActiveWindow('desktop');
    resetJobs();
    setCompanyStep(0);
    setBusinessStep(0);
  };

  const submitBusinessOperation = async () => {
    if (!ownedCompanyName || !operationPlan.trim()) return;
    const success = await onStructuredAction({ type: 'BUSINESS_OPERATION', companyName: ownedCompanyName, operation: selectedOperation.id, plan: operationPlan.trim() });
    if (success) onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4 font-sans text-slate-100 backdrop-blur-sm">
      <div className="flex h-[650px] w-full max-w-3xl flex-col space-y-5 rounded-3xl border border-[#20273a] bg-[#0a0d14] p-6 shadow-2xl animate-fadeIn">
        <header className="flex items-center justify-between border-b border-[#1c2234] pb-3">
          <div className="flex items-center gap-2.5">
            <div className="rounded-xl border border-[#20273a] bg-[#121622] p-2 text-indigo-400"><Laptop className="h-4 w-4" /></div>
            <div>
              <h3 className="font-serif text-sm font-bold uppercase tracking-wider">Workstation Workspace</h3>
              <p className="text-[10px] font-mono text-slate-400">SECURE LOCAL DESKTOP · APPLICATIONS SAVE TO YOUR LIFE</p>
            </div>
          </div>
          <button type="button" onClick={onClose} aria-label="Close computer" className="rounded-xl p-2 text-slate-400 hover:bg-slate-800 hover:text-white"><X className="h-4 w-4" /></button>
        </header>

        <div className="flex-1 overflow-y-auto">
          {activeWindow === 'desktop' ? (
            <div className="space-y-5">
              <div className="rounded-2xl border border-indigo-400/20 bg-gradient-to-r from-indigo-500/10 to-cyan-500/5 p-5">
                <p className="text-[10px] font-mono uppercase tracking-[0.18em] text-indigo-300">Workspace</p>
                <h4 className="mt-1 font-serif text-lg font-bold">What are you sitting down to do?</h4>
                <p className="mt-1 text-xs text-slate-400">Open a real workflow. Review information, complete required fields, and confirm before anything enters the simulation.</p>
              </div>
              <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
                {[
                  { window: 'jobs' as const, label: 'Job Portals', detail: 'Search, inspect, prepare, review', Icon: Briefcase, color: 'text-emerald-400' },
                  { window: 'incorporation' as const, label: 'Company Registry', detail: 'Identity, ownership, filing review', Icon: Building2, color: 'text-amber-400' },
                  ...(ownedCompanyName ? [{ window: 'business' as const, label: ownedCompanyName, detail: 'Team, investors, products, customers', Icon: ClipboardCheck, color: 'text-cyan-400' }] : []),
                  { window: 'code' as const, label: 'Code Editor', detail: 'Write before starting a session', Icon: Terminal, color: 'text-indigo-400' },
                  { window: 'research' as const, label: 'Research Browser', detail: 'Search and select reading', Icon: Globe, color: 'text-blue-400' },
                ].map(({ window, label, detail, Icon, color }) => (
                  <button key={window} type="button" onClick={() => setActiveWindow(window)} disabled={window === 'incorporation' && playerAge < 18} className="min-h-32 rounded-2xl border border-[#20273a] bg-[#121622] p-4 text-left transition hover:border-amber-500/40 hover:bg-[#181f30] disabled:cursor-not-allowed disabled:opacity-40">
                    <Icon className={`h-6 w-6 ${color}`} />
                    <p className="mt-3 font-serif text-xs font-bold">{label}</p>
                    <p className="mt-1 text-[10px] leading-relaxed text-slate-500">{detail}</p>
                  </button>
                ))}
              </div>
            </div>
          ) : null}

          {activeWindow === 'jobs' ? (
            <section className="space-y-4">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-3">
                <button type="button" onClick={selectedJob ? resetJobs : backToDesktop} aria-label="Go back" className="p-1 text-slate-400 hover:text-white"><ArrowLeft className="h-4 w-4" /></button>
                <div>
                  <h4 className="font-serif text-sm font-bold">{selectedJob ? selectedJob.title : 'Current Openings'}</h4>
                  <p className="text-[10px] text-slate-500">{selectedJob ? `${selectedJob.company} · ${selectedJob.location}` : 'Inspect a role before beginning an application.'}</p>
                </div>
              </div>

              {!selectedJob ? (
                <div className="space-y-3">
                  <div className="flex items-center gap-2 rounded-xl border border-[#263049] bg-[#10141f] px-3 py-2 text-xs text-slate-400"><Search className="h-4 w-4" />3 positions matching your location and current stage</div>
                  {jobListings.map((job) => (
                    <article key={job.id} className="rounded-2xl border border-[#20273a] bg-[#121622] p-4">
                      <div className="flex items-start justify-between gap-4">
                        <div>
                          <h5 className="font-serif text-sm font-bold">{job.title}</h5>
                          <p className="mt-1 text-[11px] text-slate-400">{job.company} · {job.location}</p>
                          <p className="mt-2 text-xs leading-relaxed text-slate-300">{job.description}</p>
                          <p className="mt-2 text-[10px] font-mono text-amber-300">{currencySymbol}{job.salary.toLocaleString()} / month</p>
                        </div>
                        <button type="button" onClick={() => setSelectedJob(job)} className="shrink-0 rounded-xl border border-emerald-500/35 bg-emerald-500/10 px-3 py-2 text-xs font-bold text-emerald-300">View role</button>
                      </div>
                    </article>
                  ))}
                </div>
              ) : (
                <div className="space-y-4">
                  <StepDots current={jobStep} labels={['Role', 'Application', 'Review']} />
                  {jobStep === 0 ? (
                    <div className="space-y-4 rounded-2xl border border-[#263049] bg-[#111622] p-5">
                      <div><p className="text-[10px] font-mono uppercase text-slate-500">Role overview</p><p className="mt-2 text-sm leading-relaxed text-slate-200">{selectedJob.description}</p></div>
                      <div><p className="text-[10px] font-mono uppercase text-slate-500">What you would do</p><ul className="mt-2 space-y-1.5 text-xs text-slate-300">{selectedJob.responsibilities.map((item) => <li key={item}>• {item}</li>)}</ul></div>
                      <div><p className="text-[10px] font-mono uppercase text-slate-500">Requirements</p><p className="mt-2 text-xs text-slate-300">{selectedJob.requirement}</p></div>
                      <button type="button" onClick={() => setJobStep(1)} className="flex w-full items-center justify-center gap-2 rounded-xl bg-emerald-500 py-2.5 text-xs font-bold text-slate-950">Begin application <ArrowRight className="h-4 w-4" /></button>
                    </div>
                  ) : null}
                  {jobStep === 1 ? (
                    <div className="space-y-4 rounded-2xl border border-[#263049] bg-[#111622] p-5">
                      <label className="block text-xs text-slate-300">Resume profile<textarea value={resumeSummary} onChange={(event) => setResumeSummary(event.target.value)} rows={3} placeholder="Summarize relevant experience, education, and skills..." className="mt-1.5 w-full resize-none rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white outline-none focus:border-emerald-400" /></label>
                      <label className="block text-xs text-slate-300">Cover letter<textarea value={coverLetter} onChange={(event) => setCoverLetter(event.target.value)} rows={4} placeholder={`Why ${selectedJob.company}, and why this role?`} className="mt-1.5 w-full resize-none rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white outline-none focus:border-emerald-400" /></label>
                      <label className="block text-xs text-slate-300">Earliest availability<select value={availability} onChange={(event) => setAvailability(event.target.value)} className="mt-1.5 w-full rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white"><option>Immediately</option><option>One week after an offer</option><option>Two weeks after an offer</option><option>One month after an offer</option></select></label>
                      <div className="flex gap-3"><button type="button" onClick={() => setJobStep(0)} className="rounded-xl border border-[#2a3550] px-4 py-2.5 text-xs text-slate-300">Back</button><button type="button" onClick={() => setJobStep(2)} disabled={!resumeSummary.trim() || !coverLetter.trim()} className="flex-1 rounded-xl bg-emerald-500 py-2.5 text-xs font-bold text-slate-950 disabled:opacity-40">Review application</button></div>
                    </div>
                  ) : null}
                  {jobStep === 2 ? (
                    <div className="space-y-4 rounded-2xl border border-emerald-500/25 bg-emerald-500/5 p-5">
                      <div className="flex items-center gap-2 text-emerald-300"><ClipboardCheck className="h-5 w-5" /><p className="font-serif text-sm font-bold">Application review</p></div>
                      <dl className="grid grid-cols-2 gap-3 text-xs"><div><dt className="text-slate-500">Role</dt><dd className="mt-1 text-slate-200">{selectedJob.title}</dd></div><div><dt className="text-slate-500">Availability</dt><dd className="mt-1 text-slate-200">{availability}</dd></div><div className="col-span-2"><dt className="text-slate-500">Resume profile</dt><dd className="mt-1 whitespace-pre-wrap text-slate-200">{resumeSummary}</dd></div><div className="col-span-2"><dt className="text-slate-500">Cover letter</dt><dd className="mt-1 whitespace-pre-wrap text-slate-200">{coverLetter}</dd></div></dl>
                      <div className="rounded-xl border border-amber-400/20 bg-amber-400/5 p-3 text-[11px] text-amber-100">Submitting creates a saved hiring process and employer acknowledgement. It does not guarantee an interview or offer.</div>
                      <div className="flex gap-3"><button type="button" onClick={() => setJobStep(1)} className="rounded-xl border border-[#2a3550] px-4 py-2.5 text-xs text-slate-300">Edit</button><button type="button" onClick={submitJobApplication} disabled={isLoading} className="flex flex-1 items-center justify-center gap-2 rounded-xl bg-emerald-500 py-2.5 text-xs font-bold text-slate-950 disabled:opacity-40"><ShieldCheck className="h-4 w-4" />Submit application</button></div>
                    </div>
                  ) : null}
                </div>
              )}
            </section>
          ) : null}

          {activeWindow === 'incorporation' ? (
            <section className="space-y-4">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-3"><button type="button" onClick={backToDesktop} aria-label="Go back" className="p-1 text-slate-400 hover:text-white"><ArrowLeft className="h-4 w-4" /></button><div><h4 className="font-serif text-sm font-bold">Corporate Affairs Registry</h4><p className="text-[10px] text-slate-500">Prepare and review an incorporation filing.</p></div></div>
              <StepDots current={companyStep} labels={['Identity', 'Ownership', 'Review']} />
              {companyStep === 0 ? (
                <div className="space-y-4 rounded-2xl border border-[#263049] bg-[#111622] p-5">
                  <label className="block text-xs text-slate-300">Proposed legal name<input value={companyName} onChange={(event) => setCompanyName(event.target.value)} placeholder="e.g. Horizon Mobility Ltd" className="mt-1.5 w-full rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white outline-none focus:border-amber-400" /></label>
                  <label className="block text-xs text-slate-300">Primary business activity<textarea value={businessActivity} onChange={(event) => setBusinessActivity(event.target.value)} rows={3} placeholder="Describe the products or services the company will provide..." className="mt-1.5 w-full resize-none rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white outline-none focus:border-amber-400" /></label>
                  <label className="block text-xs text-slate-300">Registered office address<input value={registeredAddress} onChange={(event) => setRegisteredAddress(event.target.value)} placeholder="Street, district, and city" className="mt-1.5 w-full rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white outline-none focus:border-amber-400" /></label>
                  <button type="button" onClick={() => setCompanyStep(1)} disabled={!companyName.trim() || !businessActivity.trim() || !registeredAddress.trim()} className="w-full rounded-xl bg-amber-500 py-2.5 text-xs font-bold text-slate-950 disabled:opacity-40">Continue to ownership</button>
                </div>
              ) : null}
              {companyStep === 1 ? (
                <div className="space-y-4 rounded-2xl border border-[#263049] bg-[#111622] p-5">
                  <label className="block text-xs text-slate-300">Legal structure<select value={structure} onChange={(event) => setStructure(event.target.value)} className="mt-1.5 w-full rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white"><option>Limited Liability Company (LLC)</option><option>Sole Proprietorship</option><option>General Partnership</option></select></label>
                  <label className="block text-xs text-slate-300">Partners / co-founders<input value={partners} onChange={(event) => setPartners(event.target.value)} placeholder="Comma-separated full names; leave blank for sole ownership" className="mt-1.5 w-full rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white outline-none focus:border-amber-400" /></label>
                  <label className="block text-xs text-slate-300">Authorized share capital ({currencySymbol})<input value={capital} onChange={(event) => setCapital(event.target.value)} inputMode="decimal" className="mt-1.5 w-full rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs text-white outline-none focus:border-amber-400" /></label>
                  <div className="flex gap-3"><button type="button" onClick={() => setCompanyStep(0)} className="rounded-xl border border-[#2a3550] px-4 py-2.5 text-xs text-slate-300">Back</button><button type="button" onClick={() => setCompanyStep(2)} className="flex-1 rounded-xl bg-amber-500 py-2.5 text-xs font-bold text-slate-950">Review filing</button></div>
                </div>
              ) : null}
              {companyStep === 2 ? (
                <div className="space-y-4 rounded-2xl border border-amber-500/25 bg-amber-500/5 p-5">
                  <div className="flex items-center gap-2 text-amber-300"><FilePenLine className="h-5 w-5" /><p className="font-serif text-sm font-bold">Filing review</p></div>
                  <dl className="grid grid-cols-2 gap-3 text-xs"><div><dt className="text-slate-500">Legal name</dt><dd className="mt-1 text-slate-200">{companyName}</dd></div><div><dt className="text-slate-500">Structure</dt><dd className="mt-1 text-slate-200">{structure}</dd></div><div className="col-span-2"><dt className="text-slate-500">Business activity</dt><dd className="mt-1 text-slate-200">{businessActivity}</dd></div><div className="col-span-2"><dt className="text-slate-500">Registered office</dt><dd className="mt-1 text-slate-200">{registeredAddress}</dd></div><div><dt className="text-slate-500">Partners</dt><dd className="mt-1 text-slate-200">{partners || 'None declared'}</dd></div><div><dt className="text-slate-500">Capital</dt><dd className="mt-1 text-slate-200">{currencySymbol}{capital}</dd></div></dl>
                  <div className="rounded-xl border border-amber-400/20 bg-black/15 p-3 text-[11px] text-amber-100">Filing fee: {currencySymbol}250 · Processing time: 3 calendar days · Review every field before submission.</div>
                  <div className="flex gap-3"><button type="button" onClick={() => setCompanyStep(1)} className="rounded-xl border border-[#2a3550] px-4 py-2.5 text-xs text-slate-300">Edit</button><button type="button" onClick={submitCompanyRegistration} disabled={isLoading} className="flex flex-1 items-center justify-center gap-2 rounded-xl bg-amber-500 py-2.5 text-xs font-bold text-slate-950 disabled:opacity-40"><CheckCircle className="h-4 w-4" />Submit filing and pay fee</button></div>
                </div>
              ) : null}
            </section>
          ) : null}

          {activeWindow === 'business' && ownedCompanyName ? (
            <section className="space-y-4">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-3"><button type="button" onClick={backToDesktop} aria-label="Go back" className="p-1 text-slate-400 hover:text-white"><ArrowLeft className="h-4 w-4" /></button><div><h4 className="font-serif text-sm font-bold">Operating {ownedCompanyName}</h4><p className="text-[10px] text-slate-500">Incorporation was only the beginning.</p></div></div>
              <StepDots current={businessStep} labels={['Operation', 'Plan', 'Review']} />
              {businessStep === 0 ? <div className="grid grid-cols-2 gap-3">{businessOperations.map((operation) => <button key={operation.id} type="button" onClick={() => { setSelectedOperation(operation); setBusinessStep(1); }} className="rounded-2xl border border-[#263049] bg-[#111622] p-4 text-left hover:border-cyan-400/50"><ClipboardCheck className="h-5 w-5 text-cyan-300" /><p className="mt-3 font-serif text-sm font-bold">{operation.id}</p><p className="mt-2 text-[11px] leading-relaxed text-slate-500">{operation.prompt}</p></button>)}</div> : null}
              {businessStep === 1 ? <div className="space-y-4 rounded-2xl border border-[#263049] bg-[#111622] p-5"><div><p className="text-[10px] font-mono uppercase text-cyan-300">{selectedOperation.id}</p><p className="mt-2 text-xs leading-relaxed text-slate-300">{selectedOperation.prompt}</p></div><label className="block text-xs text-slate-300">Your operating plan or response<textarea value={operationPlan} onChange={(event) => setOperationPlan(event.target.value)} rows={9} placeholder="Write the actual plan, pitch response, interview design, or commercial terms..." className="mt-2 w-full resize-none rounded-xl border border-[#2a3550] bg-[#0b0f18] p-3 text-xs leading-relaxed text-white outline-none focus:border-cyan-400" /></label><div className="flex gap-3"><button type="button" onClick={() => setBusinessStep(0)} className="rounded-xl border border-[#2a3550] px-4 text-xs">Back</button><button type="button" onClick={() => setBusinessStep(2)} disabled={!operationPlan.trim()} className="flex-1 rounded-xl bg-cyan-400 py-3 text-xs font-bold text-slate-950 disabled:opacity-40">Review operation</button></div></div> : null}
              {businessStep === 2 ? <div className="space-y-4 rounded-2xl border border-cyan-400/25 bg-cyan-400/5 p-5"><div className="flex items-center gap-2 text-cyan-200"><CheckCircle className="h-5 w-5" /><h5 className="font-serif font-bold">Review before the meeting or work session</h5></div><dl className="space-y-3 text-xs"><div><dt className="text-slate-500">Company</dt><dd className="mt-1">{ownedCompanyName}</dd></div><div><dt className="text-slate-500">Operation</dt><dd className="mt-1">{selectedOperation.id}</dd></div><div><dt className="text-slate-500">Plan / response</dt><dd className="mt-1 whitespace-pre-wrap leading-relaxed">{operationPlan}</dd></div></dl><p className="rounded-xl border border-amber-300/20 bg-amber-300/5 p-3 text-[11px] text-amber-100">This advances an operating process and records your plan. It does not automatically produce a hire, investment, customer, or successful product.</p><div className="flex gap-3"><button type="button" onClick={() => setBusinessStep(1)} className="rounded-xl border border-[#2a3550] px-4 text-xs">Edit</button><button type="button" onClick={submitBusinessOperation} disabled={isLoading} className="flex-1 rounded-xl bg-cyan-400 py-3 text-xs font-bold text-slate-950 disabled:opacity-40">Begin this operation</button></div></div> : null}
            </section>
          ) : null}

          {activeWindow === 'code' ? (
            <section className="space-y-4"><div className="flex items-center gap-2 border-b border-[#1c2234] pb-3"><button type="button" onClick={backToDesktop} aria-label="Go back" className="p-1 text-slate-400"><ArrowLeft className="h-4 w-4" /></button><h4 className="font-serif text-sm font-bold">Local Code Editor</h4></div><textarea value={codeDraft} onChange={(event) => setCodeDraft(event.target.value)} spellCheck={false} className="h-80 w-full resize-none rounded-2xl border border-[#27304a] bg-[#070b12] p-4 font-mono text-xs leading-relaxed text-cyan-100 outline-none focus:border-indigo-400" /><button type="button" onClick={() => { onExecuteAction(`I spend a focused coding session developing this work: ${codeDraft.slice(0, 180)}`); onClose(); }} disabled={!codeDraft.trim() || isLoading} className="w-full rounded-xl bg-indigo-500 py-2.5 text-xs font-bold text-white disabled:opacity-40">Save work and complete coding session</button></section>
          ) : null}

          {activeWindow === 'research' ? (
            <section className="space-y-4"><div className="flex items-center gap-2 border-b border-[#1c2234] pb-3"><button type="button" onClick={backToDesktop} aria-label="Go back" className="p-1 text-slate-400"><ArrowLeft className="h-4 w-4" /></button><h4 className="font-serif text-sm font-bold">Research Browser</h4></div><div className="flex gap-2"><input value={researchQuery} onChange={(event) => { setResearchQuery(event.target.value); setResearchSubmitted(false); }} placeholder="Search a subject, question, or publication..." className="flex-1 rounded-xl border border-[#27304a] bg-[#10141f] px-4 py-3 text-xs text-white outline-none focus:border-blue-400" /><button type="button" onClick={() => setResearchSubmitted(true)} disabled={!researchQuery.trim()} className="rounded-xl bg-blue-500 px-4 text-xs font-bold text-white disabled:opacity-40">Search</button></div>{researchSubmitted ? <div className="space-y-3">{['Foundation overview and key concepts', 'Recent academic discussion', 'Practical case study and implications'].map((title, index) => <button key={title} type="button" onClick={() => { onExecuteAction(`I spend a focused research session reading ${title.toLowerCase()} about ${researchQuery}.`); onClose(); }} className="flex w-full items-center justify-between rounded-2xl border border-[#27304a] bg-[#111622] p-4 text-left"><div><p className="font-serif text-sm text-slate-100">{title}</p><p className="mt-1 text-[11px] text-slate-500">Result {index + 1} for “{researchQuery}”</p></div><ArrowRight className="h-4 w-4 text-blue-400" /></button>)}</div> : <div className="rounded-2xl border border-dashed border-[#27304a] p-10 text-center text-xs text-slate-600">Enter a query before choosing what to read.</div>}</section>
          ) : null}
        </div>

        <footer className="flex items-center justify-between border-t border-[#1c2234] pt-3 text-xs"><span className="font-mono text-[10px] text-slate-600">Workstation online · Local save enabled</span><button type="button" onClick={onClose} className="px-3 py-1 font-serif text-slate-400 hover:text-white">Step away</button></footer>
      </div>
    </div>
  );
};
