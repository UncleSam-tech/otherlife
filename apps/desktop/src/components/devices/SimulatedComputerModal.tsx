import React from 'react';
import { Laptop, Globe, Terminal, Briefcase, X } from 'lucide-react';

interface SimulatedComputerModalProps {
  onClose: () => void;
  playerAge: number;
  onExecuteAction: (intent: string) => void;
  isLoading: boolean;
}

export const SimulatedComputerModal: React.FC<SimulatedComputerModalProps> = ({
  onClose,
  playerAge,
  onExecuteAction,
  isLoading,
}) => {
  const activeWindow = 'desktop';

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      <div className="bg-[#0a0d14] border border-[#20273a] rounded-3xl max-w-2xl w-full p-6 space-y-5 shadow-2xl flex flex-col h-[520px] animate-fadeIn">
        {/* Computer Window Title Bar */}
        <div className="flex items-center justify-between border-b border-[#1c2234] pb-3">
          <div className="flex items-center gap-2.5">
            <div className="p-2 rounded-xl bg-[#121622] border border-[#20273a] text-indigo-400">
              <Laptop className="w-4 h-4" />
            </div>
            <div>
              <h3 className="font-serif font-bold text-sm text-slate-100 uppercase tracking-wider">
                Personal Computer Workspace
              </h3>
              <p className="text-[10px] text-slate-400 font-mono">OS v4.2 · CONNECTED TO LOCAL WORKSTATION</p>
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
                {/* 1. Code Editor */}
                <button
                  type="button"
                  onClick={() => {
                    onExecuteAction('I spend time on the computer practicing software development and programming algorithms.');
                    onClose();
                  }}
                  disabled={isLoading}
                  className="p-4 rounded-2xl bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-indigo-500/50 text-left space-y-2 transition-all cursor-pointer group"
                >
                  <Terminal className="w-6 h-6 text-indigo-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <h4 className="font-serif font-bold text-xs text-slate-100">IDE & Coding Editor</h4>
                    <p className="text-[10px] text-slate-400 font-sans">Practice algorithms and software systems.</p>
                  </div>
                </button>

                {/* 2. Research & Academic Articles */}
                <button
                  type="button"
                  onClick={() => {
                    onExecuteAction('I browse academic journals and study technical papers on the computer.');
                    onClose();
                  }}
                  disabled={isLoading}
                  className="p-4 rounded-2xl bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-amber-500/50 text-left space-y-2 transition-all cursor-pointer group"
                >
                  <Globe className="w-6 h-6 text-amber-400 group-hover:scale-110 transition-transform" />
                  <div>
                    <h4 className="font-serif font-bold text-xs text-slate-100">Web Research Portal</h4>
                    <p className="text-[10px] text-slate-400 font-sans">Read research papers and world news.</p>
                  </div>
                </button>

                {/* 3. Career & Job Applications */}
                {playerAge >= 16 && (
                  <button
                    type="button"
                    onClick={() => {
                      onExecuteAction('I search online job portals and submit applications for professional career opportunities.');
                      onClose();
                    }}
                    disabled={isLoading}
                    className="p-4 rounded-2xl bg-[#121622] hover:bg-[#181f30] border border-[#20273a] hover:border-emerald-500/50 text-left space-y-2 transition-all cursor-pointer group"
                  >
                    <Briefcase className="w-6 h-6 text-emerald-400 group-hover:scale-110 transition-transform" />
                    <div>
                      <h4 className="font-serif font-bold text-xs text-slate-100">Career & Jobs Portal</h4>
                      <p className="text-[10px] text-slate-400 font-sans">Browse listings and submit applications.</p>
                    </div>
                  </button>
                )}
              </div>
            </div>
          )}
        </div>

        {/* Bottom Bar */}
        <div className="pt-3 border-t border-[#1c2234] flex justify-between items-center text-xs">
          <span className="text-[11px] text-slate-500 font-mono">Diegetic In-World Computer</span>
          <button
            type="button"
            onClick={onClose}
            className="text-slate-400 hover:text-slate-200 font-serif px-3 py-1"
          >
            Step Away from Computer
          </button>
        </div>
      </div>
    </div>
  );
};
