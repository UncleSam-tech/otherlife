import React, { useState } from 'react';
import { MessageSquare, PhoneCall, CreditCard, Send, ArrowLeft, ArrowRight } from 'lucide-react';
import { ContextNpcDTO } from '../characters/NPCDisplay';

interface SimulatedPhoneModalProps {
  onClose: () => void;
  playerAge?: number;
  cash: number;
  currencySymbol: string;
  npcs: ContextNpcDTO[];
  onExecuteAction: (intent: string) => void;
  isLoading: boolean;
}

export const SimulatedPhoneModal: React.FC<SimulatedPhoneModalProps> = ({
  onClose,
  cash,
  currencySymbol,
  npcs,
  onExecuteAction,
  isLoading,
}) => {
  const [activeApp, setActiveApp] = useState<'home' | 'messages' | 'banking' | 'calls' | 'notes'>('home');
  const [selectedContact, setSelectedContact] = useState<ContextNpcDTO | null>(null);
  const [chatInput, setChatInput] = useState('');

  const handleSendMessage = () => {
    if (!chatInput.trim() || !selectedContact || isLoading) return;
    onExecuteAction(`I send a mobile message to ${selectedContact.name}: "${chatInput.trim()}"`);
    setChatInput('');
    setActiveApp('home');
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      {/* Smartphone Device Frame */}
      <div className="bg-[#0e1118] border-2 border-[#2a344d] rounded-[40px] max-w-sm w-full p-4 space-y-4 shadow-2xl relative flex flex-col h-[600px] animate-fadeIn">
        {/* Phone Top Notch & Status Bar */}
        <div className="flex items-center justify-between px-4 pt-1 text-[10px] font-mono text-slate-400 border-b border-[#1c2234] pb-2">
          <span>9:41 AM</span>
          <div className="w-16 h-3 bg-black/80 rounded-full mx-auto" />
          <span>5G · 100%</span>
        </div>

        {/* App Content Area */}
        <div className="flex-1 overflow-y-auto px-2 space-y-3">
          {activeApp === 'home' && (
            <div className="space-y-4 pt-2">
              <div className="text-center py-2">
                <p className="text-[10px] font-mono text-amber-400 uppercase tracking-widest">Personal Smartphone</p>
                <h4 className="font-serif font-bold text-sm text-slate-100">Home Screen</h4>
              </div>

              {/* App Grid */}
              <div className="grid grid-cols-3 gap-3 text-center">
                <button
                  type="button"
                  onClick={() => setActiveApp('messages')}
                  className="flex flex-col items-center gap-1.5 p-3 rounded-2xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] transition-colors"
                >
                  <MessageSquare className="w-6 h-6 text-emerald-400" />
                  <span className="text-[10px] font-serif text-slate-200">Messages</span>
                </button>

                <button
                  type="button"
                  onClick={() => setActiveApp('banking')}
                  className="flex flex-col items-center gap-1.5 p-3 rounded-2xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] transition-colors"
                >
                  <CreditCard className="w-6 h-6 text-blue-400" />
                  <span className="text-[10px] font-serif text-slate-200">Banking</span>
                </button>

                <button
                  type="button"
                  onClick={() => setActiveApp('calls')}
                  className="flex flex-col items-center gap-1.5 p-3 rounded-2xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] transition-colors"
                >
                  <PhoneCall className="w-6 h-6 text-amber-400" />
                  <span className="text-[10px] font-serif text-slate-200">Calls</span>
                </button>
              </div>

              {/* Quick Widget: Balance */}
              <div className="bg-[#131724] border border-[#20283c] p-3.5 rounded-2xl space-y-1">
                <span className="text-[10px] font-mono text-slate-400">Available Account Balance</span>
                <p className="font-serif font-bold text-base text-emerald-400">
                  {currencySymbol}{cash.toLocaleString()}
                </p>
              </div>
            </div>
          )}

          {activeApp === 'messages' && (
            <div className="space-y-3">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-2">
                <button
                  type="button"
                  onClick={() => {
                    if (selectedContact) setSelectedContact(null);
                    else setActiveApp('home');
                  }}
                  className="p-1 text-slate-400 hover:text-slate-200"
                >
                  <ArrowLeft className="w-4 h-4" />
                </button>
                <h4 className="font-serif font-bold text-xs">
                  {selectedContact ? selectedContact.name : 'Messages & Contacts'}
                </h4>
              </div>

              {selectedContact ? (
                <div className="space-y-3 pt-2">
                  <p className="text-xs text-slate-300 font-serif italic">
                    Messaging with {selectedContact.relationship_type}...
                  </p>
                  <div className="flex gap-2">
                    <input
                      type="text"
                      value={chatInput}
                      onChange={(e) => setChatInput(e.target.value)}
                      placeholder="Type a message..."
                      className="flex-1 bg-[#141926] border border-[#222c42] rounded-xl px-3 py-2 text-xs focus:outline-none text-slate-100"
                    />
                    <button
                      type="button"
                      onClick={handleSendMessage}
                      disabled={!chatInput.trim() || isLoading}
                      className="p-2 bg-amber-500 text-slate-950 rounded-xl font-bold"
                    >
                      <Send className="w-3.5 h-3.5" />
                    </button>
                  </div>
                </div>
              ) : (
                <div className="space-y-2">
                  {npcs.map((npc) => (
                    <div
                      key={npc.id}
                      onClick={() => setSelectedContact(npc)}
                      className="p-2.5 rounded-xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] cursor-pointer flex justify-between items-center text-xs"
                    >
                      <div>
                        <p className="font-serif font-bold text-slate-200">{npc.name}</p>
                        <p className="text-[10px] text-amber-400/80 font-serif">{npc.relationship_type}</p>
                      </div>
                      <ArrowRight className="w-3.5 h-3.5 text-slate-500" />
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}

          {activeApp === 'banking' && (
            <div className="space-y-3">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-2">
                <button
                  type="button"
                  onClick={() => setActiveApp('home')}
                  className="p-1 text-slate-400 hover:text-slate-200"
                >
                  <ArrowLeft className="w-4 h-4" />
                </button>
                <h4 className="font-serif font-bold text-xs">Mobile Banking</h4>
              </div>
              <div className="bg-[#141926] border border-[#222c42] p-4 rounded-2xl space-y-2">
                <span className="text-[10px] font-mono text-slate-400">Total Checking Account</span>
                <p className="font-serif font-bold text-xl text-emerald-400">
                  {currencySymbol}{cash.toLocaleString()}
                </p>
                <p className="text-[11px] text-slate-300 font-serif">Instant transfers and statement monitoring.</p>
              </div>
            </div>
          )}

          {activeApp === 'calls' && (
            <div className="space-y-3">
              <div className="flex items-center gap-2 border-b border-[#1c2234] pb-2">
                <button
                  type="button"
                  onClick={() => setActiveApp('home')}
                  className="p-1 text-slate-400 hover:text-slate-200"
                >
                  <ArrowLeft className="w-4 h-4" />
                </button>
                <h4 className="font-serif font-bold text-xs">Phone Contacts</h4>
              </div>
              <div className="space-y-2">
                {npcs.map((npc) => (
                  <div
                    key={npc.id}
                    onClick={() => {
                      onExecuteAction(`I call ${npc.name} on my smartphone to talk.`);
                      onClose();
                    }}
                    className="p-2.5 rounded-xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] cursor-pointer flex justify-between items-center text-xs"
                  >
                    <div>
                      <p className="font-serif font-bold text-slate-200">{npc.name}</p>
                      <p className="text-[10px] text-amber-400/80 font-serif">{npc.relationship_type}</p>
                    </div>
                    <PhoneCall className="w-3.5 h-3.5 text-emerald-400" />
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Phone Bottom Home Bar */}
        <div className="pt-2 border-t border-[#1c2234] flex items-center justify-between px-2">
          <button
            type="button"
            onClick={() => setActiveApp('home')}
            className="w-1/3 py-1 text-center text-slate-400 hover:text-slate-100 text-xs font-mono"
          >
            ● Home
          </button>
          <button
            type="button"
            onClick={onClose}
            className="w-1/3 py-1 text-center text-amber-400 hover:text-amber-300 text-xs font-serif"
          >
            Put Down
          </button>
        </div>
      </div>
    </div>
  );
};
