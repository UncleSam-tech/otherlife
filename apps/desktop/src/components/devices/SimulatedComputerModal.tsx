import React, { useState } from 'react';
import { Laptop, Globe, Terminal, Briefcase, Building2, X, ArrowLeft, CheckCircle } from 'lucide-react';

interface SimulatedComputerModalProps {
  onClose: () => void;
  playerAge?: number;
  currencySymbol: string;
  onExecuteAction: (intent: string) => void;
  isLoading: boolean;
}

export const SimulatedComputerModal: React.FC<SimulatedComputerModalProps> = ({
  onClose,
  currencySymbol,
  onExecuteAction,
  isLoading,
}) => {
  const [activeWindow, setActiveWindow] = useState<'desktop' | 'jobs' | 'incorporation' | 'code' | 'research'>('desktop');

  // Incorporation Form State
  const [companyName, setCompanyName] = useState('');
  const [structure, setStructure] = useState('Limited Liability Company (LLC)');
  const [capital, setCapital] = useState('1,000,000');

  const handleRegisterCompany = (e: React.FormEvent) => {
    e.preventDefault();
    if (!companyName.trim() || isLoading) return;
    onExecuteAction(
      `I formally incorporate a new company named "${companyName.trim()}" as a ${structure} with authorized capital of ${currencySymbol}${capital}.`
    );
    onClose();
  };

  const jobListings = [
    { id: 'job_dev', title: 'Junior Software Engineer', company: 'Apex Digital Systems', salary: `${currencySymbol}250,000 / mo`, req: 'Algorithms & Problem Solving' },
    { id: 'job_analyst', title: 'Commercial Operations Analyst', company: 'Global Capital Advisory', salary: `${currencySymbol}220,000 / mo`, req: 'Financial Analysis & Diligence' },
    { id: 'job_admin', title: 'Executive Administrative Officer', company: 'Civic Infrastructure Ltd', salary: `${currencySymbol}180,000 / mo`, req: 'Communications & Organization' },
  ];

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      <div className="bg-[#0a0d14] border border-[#20273a] rounded-3xl max-w-2xl w-full p-6 space-y-5 shadow-2xl flex flex-col h-[560px] animate-fadeIn">
        {/* Computer Window Title Bar */}
        <div className="flex items-center justify-between border-b border-[#1c2234] pb-3">
          <div className="flex items-center gap-2.5">
            <div className="p-2 rounded-xl bg-[#121622] border border-[#20273a] text-indigo-400">
              <Laptop className="w-4 h-4" />
            </div>
            <div>
              <h3 className="font-serif font-bold text-sm text-slate-100 uppercase tracking-wider">
                Workstation Workspace
              </h3>
              <p className="text-[10px] text-slate-400 font-mono">CONNECTED TO LOCAL DESKTOP ENVIRONMENT</p>
            </div>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close computer"
            className="p-1.5 text-slate-400 hover:text-slate-100 rounded-xl hover:bg-slate-800 transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto space-y-4">
          {activeWindow === 'desktop' && (
            <div className="space-y-4">
              <div className="grid grid-cols-2 sm:grid-cols-3 gap-3">
                {/* 1. Job Applications Portal */}
                <button
                  type="button"
                  onClick={() => setActiveWindow('jobs')}
                  className="p-4 rounded-2xl bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-emerald-500/50 text-left space-y-2 transition-all cursor-pointer group"
                >
                  <Briefcase className="w-6 h-6 text-emerald-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <h4 className="font-serif font-bold text-xs text-slate-100">Job Portals</h4>
                    <p className="text-[10px] text-slate-400 font-sans">Browse listings and submit applications.</p>
                  </div>
                </button>

                {/* 2. Company Incorporation Registry */}
                <button
                  type="button"
                  onClick={() => setActiveWindow('incorporation')}
                  className="p-4 rounded-2xl bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-amber-500/50 text-left space-y-2 transition-all cursor-pointer group"
                >
                  <Building2 className="w-6 h-6 text-amber-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <h4 className="font-serif font-bold text-xs text-slate-100">Company Registry</h4>
                    <p className="text-[10px] text-slate-400 font-sans">Incorporate a new business enterprise.</p>
                  </div>
                </button>

                {/* 3. Code Editor */}
                <button
                  type="button"
                  onClick={() => {
                    onExecuteAction('I spend time on the workstation writing code and studying system architectures.');
                    onClose();
                  }}
                  disabled={isLoading}
                  className="p-4 rounded-2xl bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-indigo-500/50 text-left space-y-2 transition-all cursor-pointer group"
                >
                  <Terminal className="w-6 h-6 text-indigo-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <h4 className="font-serif font-bold text-xs text-slate-100">IDE & Coding Editor</h4>
                    <p className="text-[10px] text-slate-400 font-sans">Practice software programming.</p>
                  </div>
                </button>

                {/* 4. Research & Academic Articles */}
                <button
                  type="button"
                  onClick={() => {
                    onExecuteAction('I spend hours studying academic journals and technical literature online.');
                    onClose();
                  }}
                  disabled={isLoading}
                  className="p-4 rounded-2xl bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-blue-500/50 text-left space-y-2 transition-all cursor-pointer group"
                >
                  <Globe className="w-6 h-6 text-blue-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <h4 className="font-serif font-bold text-xs text-slate-100">Web Research</h4>
                    <p className="text-[10px] text-slate-400 font-sans">Read journals and publications.</p>
                  </div>
                </button>
              </div>
            </div>
          )}

          {/* Job Listings Window */}
          {activeWindow === 'jobs' && (
            <div className="space-y-3">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-2">
                <button
                  type="button"
                  onClick={() => setActiveWindow('desktop')}
                  className="p-1 text-slate-400 hover:text-slate-100"
                >
                  <ArrowLeft className="w-4 h-4" />
                </button>
                <h4 className="font-serif font-bold text-xs">Open Corporate Positions</h4>
              </div>

              <div className="space-y-2">
                {jobListings.map((job) => (
                  <div
                    key={job.id}
                    className="p-3 rounded-xl bg-[#121622] border border-[#20273a] flex items-center justify-between text-xs"
                  >
                    <div>
                      <h5 className="font-serif font-bold text-slate-100">{job.title}</h5>
                      <p className="text-[11px] text-slate-400">{job.company} · {job.salary}</p>
                      <p className="text-[10px] text-amber-400/80 font-mono mt-0.5">Req: {job.req}</p>
                    </div>
                    <button
                      type="button"
                      onClick={() => {
                        onExecuteAction(`I submit a formal job application for the ${job.title} position at ${job.company}.`);
                        onClose();
                      }}
                      disabled={isLoading}
                      className="bg-emerald-600 hover:bg-emerald-500 text-slate-950 px-3 py-1.5 rounded-xl font-serif font-bold text-xs transition-colors cursor-pointer"
                    >
                      Apply Now
                    </button>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Company Incorporation Window */}
          {activeWindow === 'incorporation' && (
            <form onSubmit={handleRegisterCompany} className="space-y-4">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-2">
                <button
                  type="button"
                  onClick={() => setActiveWindow('desktop')}
                  className="p-1 text-slate-400 hover:text-slate-100"
                >
                  <ArrowLeft className="w-4 h-4" />
                </button>
                <h4 className="font-serif font-bold text-xs">Corporate Affairs & Business Registration</h4>
              </div>

              <div className="space-y-3 text-xs">
                <div>
                  <label className="block text-slate-400 font-serif mb-1">Proposed Company Name</label>
                  <input
                    type="text"
                    value={companyName}
                    onChange={(e) => setCompanyName(e.target.value)}
                    placeholder="e.g. Apex Frontier Technologies Ltd"
                    required
                    className="w-full bg-[#121622] border border-[#20273a] rounded-xl px-3 py-2 text-slate-100 focus:outline-none focus:border-amber-500/60"
                  />
                </div>

                <div>
                  <label className="block text-slate-400 font-serif mb-1">Legal Structure</label>
                  <select
                    value={structure}
                    onChange={(e) => setStructure(e.target.value)}
                    className="w-full bg-[#121622] border border-[#20273a] rounded-xl px-3 py-2 text-slate-100 focus:outline-none"
                  >
                    <option>Limited Liability Company (LLC)</option>
                    <option>Sole Proprietorship</option>
                    <option>General Partnership</option>
                  </select>
                </div>

                <div>
                  <label className="block text-slate-400 font-serif mb-1">Authorized Share Capital ({currencySymbol})</label>
                  <input
                    type="text"
                    value={capital}
                    onChange={(e) => setCapital(e.target.value)}
                    className="w-full bg-[#121622] border border-[#20273a] rounded-xl px-3 py-2 text-slate-100 focus:outline-none"
                  />
                </div>

                <div className="pt-2">
                  <button
                    type="submit"
                    disabled={!companyName.trim() || isLoading}
                    className="w-full bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 text-slate-950 font-serif font-bold py-2.5 rounded-xl text-xs flex items-center justify-center gap-2 cursor-pointer disabled:opacity-50"
                  >
                    <CheckCircle className="w-4 h-4" />
                    <span>Submit Formal Incorporation Filing</span>
                  </button>
                </div>
              </div>
            </form>
          )}
        </div>

        {/* Bottom Bar */}
        <div className="pt-3 border-t border-[#1c2234] flex justify-between items-center text-xs">
          <span className="text-[11px] text-slate-500 font-mono">Workstation Online</span>
          <button
            type="button"
            onClick={onClose}
            className="text-slate-400 hover:text-slate-200 font-serif px-3 py-1 cursor-pointer"
          >
            Step Away
          </button>
        </div>
      </div>
    </div>
  );
};
