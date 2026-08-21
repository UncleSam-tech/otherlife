import React from 'react';
import { Smartphone, Laptop, Wallet, FileText, X, Award } from 'lucide-react';

interface DiegeticDeviceModalProps {
  deviceType: 'phone' | 'computer' | 'wallet' | 'documents' | 'mail' | null;
  onClose: () => void;
  playerAge?: number;
  cash: number;
  currencySymbol: string;
  onExecuteAction: (intentText: string) => void;
  isLoading: boolean;
}

export const DiegeticDeviceModal: React.FC<DiegeticDeviceModalProps> = ({
  deviceType,
  onClose,
  cash,
  currencySymbol,
  onExecuteAction,
  isLoading,
}) => {
  if (!deviceType) return null;

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      <div className="bg-[#0e1118] border border-amber-500/30 rounded-3xl max-w-lg w-full p-6 space-y-5 shadow-2xl animate-fadeIn">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-2xl bg-[#141824] border border-[#22283a]">
              {deviceType === 'phone' && <Smartphone className="w-5 h-5 text-blue-400" />}
              {deviceType === 'computer' && <Laptop className="w-5 h-5 text-indigo-400" />}
              {deviceType === 'wallet' && <Wallet className="w-5 h-5 text-emerald-400" />}
              {deviceType === 'documents' && <FileText className="w-5 h-5 text-amber-400" />}
            </div>
            <div>
              <h3 className="font-serif font-bold text-lg text-slate-100 uppercase tracking-wider">
                {deviceType === 'phone' && 'Personal Smartphone'}
                {deviceType === 'computer' && 'Personal Computer'}
                {deviceType === 'wallet' && 'Wallet & Financial Cards'}
                {deviceType === 'documents' && 'Official Documents & Credentials'}
              </h3>
              <p className="text-xs text-amber-300/80 font-mono">DIEGETIC SIMULATION DEVICE</p>
            </div>
          </div>
          <button
            type="button"
            onClick={onClose}
            className="p-1.5 text-slate-400 hover:text-slate-100 rounded-xl hover:bg-slate-800 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content per Device */}
        {deviceType === 'phone' && (
          <div className="space-y-3">
            <div className="bg-[#121622] border border-[#20273a] p-4 rounded-2xl space-y-2">
              <div className="flex items-center justify-between text-xs">
                <span className="text-slate-400">Mobile Banking App</span>
                <span className="font-mono text-emerald-400 font-bold">{currencySymbol}{cash.toLocaleString()}</span>
              </div>
              <p className="text-xs text-slate-300 font-serif">Instant balance check and digital payments.</p>
            </div>
            <button
              type="button"
              onClick={() => {
                onExecuteAction('I open my phone and message close friends to check in.');
                onClose();
              }}
              disabled={isLoading}
              className="w-full bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] p-3 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors flex items-center justify-between"
            >
              <span>💬 Message Friends & Contacts</span>
            </button>
          </div>
        )}

        {deviceType === 'computer' && (
          <div className="space-y-3">
            <button
              type="button"
              onClick={() => {
                onExecuteAction('I spend time on the computer practicing coding algorithms and software development.');
                onClose();
              }}
              disabled={isLoading}
              className="w-full bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] p-3 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors"
            >
              💻 Practice Software Development & Algorithms
            </button>
            <button
              type="button"
              onClick={() => {
                onExecuteAction('I search online job portals and submit applications for entry-level positions.');
                onClose();
              }}
              disabled={isLoading}
              className="w-full bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] p-3 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors"
            >
              🌐 Browse Online Career Portals
            </button>
          </div>
        )}

        {deviceType === 'wallet' && (
          <div className="space-y-3">
            <div className="bg-[#121622] border border-[#20273a] p-4 rounded-2xl flex justify-between items-center">
              <span className="text-slate-400 text-xs">Cash in Hand</span>
              <span className="font-serif font-bold text-amber-300 text-base">{currencySymbol}{cash.toLocaleString()}</span>
            </div>
            <p className="text-xs text-slate-400 font-serif italic">
              Contains your national identity card, public transit card, and personal banknotes.
            </p>
          </div>
        )}

        {deviceType === 'documents' && (
          <div className="space-y-3">
            <div className="bg-[#121622] border border-[#20273a] p-4 rounded-2xl space-y-2">
              <div className="flex items-center gap-2 text-amber-400 font-serif font-bold text-sm">
                <Award className="w-4 h-4" />
                <span>Civic Birth Certificate</span>
              </div>
              <p className="text-xs text-slate-300 font-serif">
                Official verified record of birth and citizenship in the living world.
              </p>
            </div>
          </div>
        )}

        <div className="pt-2 border-t border-[#1c2130] flex justify-end">
          <button
            type="button"
            onClick={onClose}
            className="text-xs text-slate-400 hover:text-slate-200 font-serif px-4 py-2"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
