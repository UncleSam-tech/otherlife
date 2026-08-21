import React from 'react';
import { MessageCircle, DollarSign, Heart, HelpCircle, Gift, Flame, X, User } from 'lucide-react';
import { ContextNpcDTO } from '../characters/NPCDisplay';

interface PersonInteractionModalProps {
  npc: ContextNpcDTO | null;
  playerAge: number;
  onClose: () => void;
  onExecuteAction: (intentText: string) => void;
  isLoading: boolean;
}

export const PersonInteractionModal: React.FC<PersonInteractionModalProps> = ({
  npc,
  playerAge,
  onClose,
  onExecuteAction,
  isLoading,
}) => {
  if (!npc) return null;

  const role = npc.relationship_type.toLowerCase();
  const isParent = role.includes('parent') || role.includes('mother') || role.includes('father');
  const isTeacher = role.includes('teacher') || role.includes('mentor');
  const isCoach = role.includes('coach');
  const isFriendOrCrush = role.includes('friend') || role.includes('classmate') || role.includes('partner');

  const getAvailableInteractions = () => {
    const actions: { id: string; title: string; desc: string; icon: React.ReactNode; intent: string }[] = [];

    // General conversation available for all
    actions.push({
      id: 'talk',
      title: 'Spend Time & Talk',
      desc: `Have a meaningful, heartfelt conversation with ${npc.name}.`,
      icon: <MessageCircle className="w-4 h-4 text-amber-400" />,
      intent: `I sit down with ${npc.name} to spend quality time and have a meaningful conversation.`,
    });

    if (isParent) {
      actions.push({
        id: 'ask_money',
        title: 'Ask for Pocket Money / Allowance',
        desc: 'Request a small monetary allowance for school stationery, food, or personal needs.',
        icon: <DollarSign className="w-4 h-4 text-emerald-400" />,
        intent: `I politely ask ${npc.name} for a pocket money allowance for school and personal expenses.`,
      });
      actions.push({
        id: 'ask_advice',
        title: 'Seek Parental Guidance & Advice',
        desc: 'Discuss your life choices, challenges, and future ambitions.',
        icon: <HelpCircle className="w-4 h-4 text-blue-400" />,
        intent: `I open up to ${npc.name} about my life aspirations and ask for their guidance and advice.`,
      });
      actions.push({
        id: 'help_parent',
        title: 'Offer Help & Do a Favor',
        desc: 'Help with chores, work errands, or family responsibilities.',
        icon: <Gift className="w-4 h-4 text-purple-400" />,
        intent: `I offer to assist ${npc.name} with their daily chores and family responsibilities to show appreciation.`,
      });
    }

    if (isTeacher) {
      actions.push({
        id: 'academic_help',
        title: 'Request Extra Academic Mentorship',
        desc: 'Ask for additional tutoring on complex examination topics and study strategies.',
        icon: <HelpCircle className="w-4 h-4 text-blue-400" />,
        intent: `I stay after class to ask ${npc.name} for detailed academic mentorship on difficult examination subjects.`,
      });
      actions.push({
        id: 'recommendation',
        title: 'Request Letter of Recommendation',
        desc: 'Ask for an academic recommendation for university admissions or scholarships.',
        icon: <Gift className="w-4 h-4 text-amber-400" />,
        intent: `I request an academic letter of recommendation from ${npc.name} for higher education admissions.`,
      });
    }

    if (isCoach) {
      actions.push({
        id: 'drill_review',
        title: 'Request Tactical Drill Feedback',
        desc: 'Ask the coach how you can improve your position and stamina for upcoming scouting matches.',
        icon: <HelpCircle className="w-4 h-4 text-orange-400" />,
        intent: `I ask Coach ${npc.name} for honest feedback on my tactical positioning and athletic stamina.`,
      });
    }

    if (isFriendOrCrush && playerAge >= 13) {
      actions.push({
        id: 'hangout',
        title: 'Invite to Hang Out',
        desc: 'Spend the weekend together listening to music, sharing snacks, and chatting.',
        icon: <Heart className="w-4 h-4 text-rose-400" />,
        intent: `I invite ${npc.name} to hang out with me over the weekend so we can deepen our connection.`,
      });
      actions.push({
        id: 'confess_feelings',
        title: 'Express Romantic Feelings / Date',
        desc: 'Gather your courage and ask them out on a romantic date.',
        icon: <Flame className="w-4 h-4 text-red-400" />,
        intent: `I tell ${npc.name} how I feel and ask them to be my romantic partner.`,
      });
    }

    return actions;
  };

  const interactions = getAvailableInteractions();

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="bg-[#0e1118] border border-amber-500/30 rounded-3xl max-w-md w-full p-6 space-y-5 shadow-2xl animate-fadeIn font-sans select-none text-slate-100">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-2xl bg-[#141824] border border-[#22283a]">
              <User className="w-5 h-5 text-amber-400" />
            </div>
            <div>
              <h3 className="font-serif font-bold text-lg text-slate-100">{npc.name}</h3>
              <p className="text-xs text-amber-300/80 font-serif italic">{npc.relationship_type}</p>
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

        {/* Current Activity Context */}
        <div className="bg-[#121622] border border-[#20273a] p-3 rounded-xl text-xs text-slate-300 font-sans">
          <span className="text-slate-500 font-serif">Currently: </span>
          {npc.current_activity}
        </div>

        {/* Action Choices */}
        <div className="space-y-2 max-h-80 overflow-y-auto pr-1">
          {interactions.map((act) => (
            <button
              key={act.id}
              type="button"
              onClick={() => {
                onExecuteAction(act.intent);
                onClose();
              }}
              disabled={isLoading}
              className="w-full bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] hover:border-amber-500/50 p-3.5 rounded-2xl text-left transition-all duration-200 flex items-start gap-3 group shadow-sm"
            >
              <div className="p-2 rounded-xl bg-[#181d2c] border border-[#242c3e] mt-0.5 group-hover:scale-105 transition-transform">
                {act.icon}
              </div>
              <div className="flex-1">
                <div className="font-serif font-bold text-xs text-slate-100 group-hover:text-amber-200">
                  {act.title}
                </div>
                <div className="text-[11px] text-slate-400 font-sans leading-snug mt-0.5">
                  {act.desc}
                </div>
              </div>
            </button>
          ))}
        </div>

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
