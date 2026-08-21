import React, { useState } from 'react';
import { ContextNpcDTO } from './NPCDisplay';
import { Send, X, CornerDownRight } from 'lucide-react';

interface ConversationModalProps {
  npc: ContextNpcDTO | null;
  onClose: () => void;
  onSendMessage: (messageText: string) => void;
  isLoading: boolean;
}

export const ConversationModal: React.FC<ConversationModalProps> = ({
  npc,
  onClose,
  onSendMessage,
  isLoading,
}) => {
  if (!npc) return null;

  const [inputMessage, setInputMessage] = useState('');
  const [messages, setMessages] = useState<Array<{ sender: 'player' | 'npc'; text: string; reaction?: string }>>([
    {
      sender: 'npc',
      text: `Hello there. How are you holding up today?`,
      reaction: 'Smiles gently and pauses what they were doing to give you their full attention.',
    },
  ]);

  const handleSend = (textToSend: string) => {
    if (!textToSend.trim() || isLoading) return;

    // Add player response to history
    setMessages((prev) => [
      ...prev,
      { sender: 'player', text: textToSend.trim() },
      {
        sender: 'npc',
        text: `I understand where you are coming from. Let us take it one step at a time.`,
        reaction: 'Nods thoughtfully, considering your words with sincere care.',
      },
    ]);

    onSendMessage(`I say to ${npc.name}: "${textToSend.trim()}"`);
    setInputMessage('');
  };

  const suggestions = [
    { label: 'Ask directly', text: 'Could we talk candidly about future plans and finances?' },
    { label: 'Respond cautiously', text: 'I am doing alright, taking things one day at a time.' },
    { label: 'Seek life guidance', text: 'What would you advise I focus on most right now?' },
    { label: 'Express appreciation', text: 'Thank you for always being there to support me.' },
    { label: 'Share a concern', text: 'I have been feeling some pressure about the road ahead.' },
    { label: 'Say nothing', text: '(Sit in comfortable silence and listen attentively)' },
  ];

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      <div className="bg-[#0b0e17] border border-amber-500/30 rounded-3xl max-w-xl w-full p-6 space-y-4 shadow-2xl flex flex-col max-h-[90vh] animate-fadeIn">
        {/* Conversation Header */}
        <div className="flex items-center justify-between border-b border-[#1c2234] pb-4">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-2xl bg-[#141824] border border-[#22283a] flex items-center justify-center text-amber-400 font-serif font-bold text-sm">
              {npc.name[0] || 'N'}
            </div>
            <div>
              <h3 className="font-serif font-bold text-base text-slate-100">{npc.name}</h3>
              <p className="text-xs text-amber-300/80 font-serif italic">
                {npc.relationship_type} · {npc.trust_description}
              </p>
            </div>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close conversation"
            className="p-1.5 text-slate-400 hover:text-slate-100 rounded-xl hover:bg-slate-800 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Message Stream */}
        <div className="flex-1 overflow-y-auto space-y-3 pr-1 min-h-[160px] max-h-[280px]">
          {messages.map((m, idx) => (
            <div key={idx} className={`space-y-1 ${m.sender === 'player' ? 'text-right' : 'text-left'}`}>
              <div
                className={`inline-block p-3.5 rounded-2xl text-xs font-serif leading-relaxed max-w-[85%] ${
                  m.sender === 'player'
                    ? 'bg-amber-600/20 border border-amber-500/40 text-amber-100'
                    : 'bg-[#121622] border border-[#1e2538] text-slate-200'
                }`}
              >
                {m.text}
              </div>
              {m.reaction && (
                <p className="text-[11px] font-serif italic text-slate-400 px-2 flex items-center gap-1">
                  <CornerDownRight className="w-3 h-3 text-amber-400/80 inline" />
                  <span>{m.reaction}</span>
                </p>
              )}
            </div>
          ))}
        </div>

        {/* Varied Response Suggestions */}
        <div className="space-y-1.5 pt-2 border-t border-[#181e2e]">
          <span className="text-[11px] font-serif text-slate-400 italic">Suggested Approaches:</span>
          <div className="flex flex-wrap gap-1.5">
            {suggestions.map((sug, i) => (
              <button
                key={i}
                type="button"
                onClick={() => handleSend(sug.text)}
                disabled={isLoading}
                className="text-[11px] font-serif bg-[#121622] hover:bg-[#1c2336] text-slate-300 hover:text-amber-200 border border-[#20273a] hover:border-amber-500/40 px-2.5 py-1 rounded-full transition-colors cursor-pointer"
              >
                <span className="text-amber-400 font-semibold">{sug.label}: </span>
                <span>{sug.text.slice(0, 32)}...</span>
              </button>
            ))}
          </div>
        </div>

        {/* Free-Text Input */}
        <form
          onSubmit={(e) => {
            e.preventDefault();
            handleSend(inputMessage);
          }}
          className="flex items-center gap-2 pt-2"
        >
          <input
            type="text"
            value={inputMessage}
            onChange={(e) => setInputMessage(e.target.value)}
            placeholder="Speak naturally or state your question..."
            disabled={isLoading}
            className="flex-1 bg-[#121622] border border-[#20273a] focus:border-amber-500/70 rounded-2xl px-4 py-2.5 text-xs text-slate-100 placeholder:text-slate-500 focus:outline-none font-sans"
          />
          <button
            type="submit"
            disabled={!inputMessage.trim() || isLoading}
            className="bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 text-slate-950 font-serif font-bold text-xs px-4 py-2.5 rounded-2xl flex items-center gap-1.5 cursor-pointer disabled:opacity-50"
          >
            <span>Speak</span>
            <Send className="w-3.5 h-3.5" />
          </button>
        </form>
      </div>
    </div>
  );
};
