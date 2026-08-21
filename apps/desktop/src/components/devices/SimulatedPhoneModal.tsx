import React, { useState } from 'react';
import { MessageSquare, PhoneCall, CreditCard, Send, ArrowLeft, ArrowRight, PhoneOff } from 'lucide-react';
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
  const [activeApp, setActiveApp] = useState<'home' | 'messages' | 'banking' | 'calls'>('home');
  const [selectedContact, setSelectedContact] = useState<ContextNpcDTO | null>(null);
  const [chatInput, setChatInput] = useState('');
  const [threads, setThreads] = useState<Record<string, Array<{ sender: 'player' | 'npc'; text: string; time: string }>>>({
    'person:sim:mother': [
      { sender: 'npc', text: 'Please remember to take care of yourself today. Proud of your focus!', time: '8:30 AM' },
    ],
    'person:sim:father': [
      { sender: 'npc', text: 'Let me know if you need any guidance with your plans.', time: 'Yesterday' },
    ],
  });

  // Call State
  const [callingContact, setCallingContact] = useState<ContextNpcDTO | null>(null);
  const [callStatus, setCallStatus] = useState<'RINGING' | 'CONNECTED' | null>(null);
  const [callLog, setCallLog] = useState<string[]>([]);
  const [callInput, setCallInput] = useState('');

  const handleStartCall = (contact: ContextNpcDTO) => {
    setCallingContact(contact);
    setCallStatus('RINGING');
    setCallLog([`Calling ${contact.name}...`]);

    setTimeout(() => {
      setCallStatus('CONNECTED');
      setCallLog((prev) => [
        ...prev,
        `${contact.name}: "Hello! Good to hear from you. How are things going?"`,
      ]);
    }, 1200);
  };

  const handleSendCallDialogue = () => {
    if (!callInput.trim() || !callingContact || isLoading) return;
    const text = callInput.trim();
    setCallLog((prev) => [
      ...prev,
      `You: "${text}"`,
      `${callingContact.name}: "I hear you clearly. Let's make sure we stay disciplined and focused on the goal."`,
    ]);
    setCallInput('');
    onExecuteAction(`I speak with ${callingContact.name} over the phone: "${text}"`);
  };

  const handleEndCall = () => {
    setCallStatus(null);
    setCallingContact(null);
    setCallLog([]);
  };

  const handleSendMessage = () => {
    if (!chatInput.trim() || !selectedContact || isLoading) return;
    const msg = chatInput.trim();
    const contactId = selectedContact.id;

    setThreads((prev) => ({
      ...prev,
      [contactId]: [
        ...(prev[contactId] || []),
        { sender: 'player', text: msg, time: 'Just now' },
        { sender: 'npc', text: `Got your message. Always here if you need anything!`, time: 'Just now' },
      ],
    }));

    onExecuteAction(`I send a mobile text message to ${selectedContact.name}: "${msg}"`);
    setChatInput('');
  };

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      {/* Smartphone Device Frame */}
      <div className="bg-[#0e1118] border-2 border-[#2a344d] rounded-[40px] max-w-sm w-full p-4 space-y-4 shadow-2xl relative flex flex-col h-[620px] animate-fadeIn">
        {/* Phone Top Notch & Status Bar */}
        <div className="flex items-center justify-between px-4 pt-1 text-[10px] font-mono text-slate-400 border-b border-[#1c2234] pb-2">
          <span>9:41 AM</span>
          <div className="w-16 h-3 bg-black/80 rounded-full mx-auto" />
          <span>5G · 100%</span>
        </div>

        {/* Active Phone Call Overlay */}
        {callStatus && callingContact && (
          <div className="flex-1 flex flex-col justify-between bg-[#0a0d14] rounded-3xl p-5 space-y-4 border border-emerald-500/30 animate-fadeIn">
            <div className="text-center space-y-1">
              <div className="w-14 h-14 rounded-full bg-emerald-500/20 border border-emerald-500/40 text-emerald-400 flex items-center justify-center mx-auto text-lg font-bold font-serif">
                {callingContact.name[0]}
              </div>
              <h3 className="font-serif font-bold text-base text-slate-100">{callingContact.name}</h3>
              <p className="text-[11px] font-mono text-emerald-400">
                {callStatus === 'RINGING' ? 'Ringing...' : 'Connected · Active Call'}
              </p>
            </div>

            <div className="flex-1 overflow-y-auto space-y-2 bg-[#121622] rounded-2xl p-3 text-xs font-serif text-slate-200 min-h-[140px] max-h-[220px]">
              {callLog.map((log, i) => (
                <p key={i} className="leading-relaxed">{log}</p>
              ))}
            </div>

            {callStatus === 'CONNECTED' && (
              <div className="flex gap-2">
                <input
                  type="text"
                  value={callInput}
                  onChange={(e) => setCallInput(e.target.value)}
                  placeholder="Speak into the phone..."
                  className="flex-1 bg-[#141926] border border-[#222c42] rounded-xl px-3 py-2 text-xs focus:outline-none text-slate-100"
                />
                <button
                  type="button"
                  onClick={handleSendCallDialogue}
                  disabled={!callInput.trim() || isLoading}
                  className="p-2 bg-emerald-500 text-slate-950 rounded-xl font-bold"
                >
                  <Send className="w-3.5 h-3.5" />
                </button>
              </div>
            )}

            <div className="pt-2 flex justify-center">
              <button
                type="button"
                onClick={handleEndCall}
                className="w-12 h-12 rounded-full bg-red-600 hover:bg-red-500 text-white flex items-center justify-center shadow-lg transition-transform hover:scale-105 cursor-pointer"
              >
                <PhoneOff className="w-5 h-5" />
              </button>
            </div>
          </div>
        )}

        {/* App Content Area (when not on a call) */}
        {!callStatus && (
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
                    className="flex flex-col items-center gap-1.5 p-3 rounded-2xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] transition-colors cursor-pointer"
                  >
                    <MessageSquare className="w-6 h-6 text-emerald-400" />
                    <span className="text-[10px] font-serif text-slate-200">Messages</span>
                  </button>

                  <button
                    type="button"
                    onClick={() => setActiveApp('banking')}
                    className="flex flex-col items-center gap-1.5 p-3 rounded-2xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] transition-colors cursor-pointer"
                  >
                    <CreditCard className="w-6 h-6 text-blue-400" />
                    <span className="text-[10px] font-serif text-slate-200">Banking</span>
                  </button>

                  <button
                    type="button"
                    onClick={() => setActiveApp('calls')}
                    className="flex flex-col items-center gap-1.5 p-3 rounded-2xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] transition-colors cursor-pointer"
                  >
                    <PhoneCall className="w-6 h-6 text-amber-400" />
                    <span className="text-[10px] font-serif text-slate-200">Calls</span>
                  </button>
                </div>

                {/* Account Balance Widget */}
                <div className="bg-[#131724] border border-[#20283c] p-4 rounded-2xl space-y-1">
                  <span className="text-[10px] font-mono text-slate-400">Checking Balance</span>
                  <p className="font-serif font-bold text-lg text-emerald-400">
                    {currencySymbol}{cash.toLocaleString()}
                  </p>
                  <p className="text-[11px] text-slate-400 font-serif">Verified local currency account.</p>
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
                    {selectedContact ? selectedContact.name : 'Message Threads'}
                  </h4>
                </div>

                {selectedContact ? (
                  <div className="space-y-3 pt-1">
                    <div className="space-y-2 max-h-[300px] overflow-y-auto pr-1">
                      {(threads[selectedContact.id] || []).map((msg, idx) => (
                        <div
                          key={idx}
                          className={`flex flex-col ${
                            msg.sender === 'player' ? 'items-end' : 'items-start'
                          }`}
                        >
                          <div
                            className={`p-2.5 rounded-2xl text-xs font-serif leading-relaxed max-w-[85%] ${
                              msg.sender === 'player'
                                ? 'bg-amber-600/30 text-amber-100 border border-amber-500/40'
                                : 'bg-[#141926] text-slate-200 border border-[#222c42]'
                            }`}
                          >
                            {msg.text}
                          </div>
                          <span className="text-[9px] text-slate-500 font-mono mt-0.5 px-1">{msg.time}</span>
                        </div>
                      ))}
                    </div>

                    <div className="flex gap-2 pt-2">
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
                  <h4 className="font-serif font-bold text-xs">Mobile Banking & Ledger</h4>
                </div>
                <div className="bg-[#141926] border border-[#222c42] p-4 rounded-2xl space-y-2">
                  <span className="text-[10px] font-mono text-slate-400">Total Checking Account</span>
                  <p className="font-serif font-bold text-xl text-emerald-400">
                    {currencySymbol}{cash.toLocaleString()}
                  </p>
                  <p className="text-[11px] text-slate-300 font-serif">Verified personal account ledger.</p>
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
                  <h4 className="font-serif font-bold text-xs">Contacts to Call</h4>
                </div>
                <div className="space-y-2">
                  {npcs.map((npc) => (
                    <div
                      key={npc.id}
                      onClick={() => handleStartCall(npc)}
                      className="p-2.5 rounded-xl bg-[#141926] hover:bg-[#1d2438] border border-[#222c42] cursor-pointer flex justify-between items-center text-xs group"
                    >
                      <div>
                        <p className="font-serif font-bold text-slate-200 group-hover:text-amber-200">{npc.name}</p>
                        <p className="text-[10px] text-amber-400/80 font-serif">{npc.relationship_type}</p>
                      </div>
                      <PhoneCall className="w-3.5 h-3.5 text-emerald-400 group-hover:scale-110 transition-transform" />
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {/* Phone Bottom Home Bar */}
        <div className="pt-2 border-t border-[#1c2234] flex items-center justify-between px-2">
          <button
            type="button"
            onClick={() => setActiveApp('home')}
            className="w-1/3 py-1 text-center text-slate-400 hover:text-slate-100 text-xs font-mono cursor-pointer"
          >
            ● Home
          </button>
          <button
            type="button"
            onClick={onClose}
            className="w-1/3 py-1 text-center text-amber-400 hover:text-amber-300 text-xs font-serif cursor-pointer"
          >
            Put Down
          </button>
        </div>
      </div>
    </div>
  );
};
