import React, { useState } from 'react';
import { Calendar as CalendarIcon, Clock, Moon, AlertTriangle, X, CheckCircle } from 'lucide-react';

interface CalendarModalProps {
  timeFormatted: string;
  playerAge: number;
  onClose: () => void;
  onAdvanceTime: (intentText: string) => void;
  isLoading: boolean;
}

export const CalendarModal: React.FC<CalendarModalProps> = ({
  timeFormatted,
  playerAge,
  onClose,
  onAdvanceTime,
  isLoading,
}) => {
  const [pendingAdvance, setPendingAdvance] = useState<{ intent: string; label: string; warning?: string } | null>(null);

  const getUpcomingEvents = () => {
    if (playerAge < 4) {
      return [
        { title: 'Pediatric Growth & Vaccine Review', time: 'Next Month', type: 'Health' },
        { title: 'Family Home Milestones', time: 'Ongoing', type: 'Family' },
      ];
    } else if (playerAge < 13) {
      return [
        { title: 'Term Arithmetic Progress Review', time: 'End of Term', type: 'Education' },
        { title: 'Youth Community Football Drills', time: 'Every Saturday', type: 'Athletics' },
      ];
    } else if (playerAge < 18) {
      return [
        { title: 'National Certificate Examinations (WAEC / JAMB)', time: 'Upcoming Term', type: 'Academic' },
        { title: 'Youth Academy Scouting Selection Match', time: 'Next Weekend', type: 'Sports' },
      ];
    } else {
      return [
        { title: 'Monthly Living Expenses & Rent Settlement', time: 'End of Month', type: 'Finance' },
        { title: 'Career Development & Enterprise Milestones', time: 'Continuous', type: 'Career' },
      ];
    }
  };

  const upcomingEvents = getUpcomingEvents();

  const handleAdvanceClick = (intent: string, label: string, warning?: string) => {
    if (warning) {
      setPendingAdvance({ intent, label, warning });
    } else {
      onAdvanceTime(intent);
      onClose();
    }
  };

  const handleConfirmAdvance = () => {
    if (pendingAdvance) {
      onAdvanceTime(pendingAdvance.intent);
      setPendingAdvance(null);
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      <div className="bg-[#0b0e17] border border-amber-500/30 rounded-3xl max-w-lg w-full p-6 space-y-5 shadow-2xl animate-fadeIn">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-[#1c2234] pb-4">
          <div className="flex items-center gap-3">
            <div className="p-2.5 rounded-2xl bg-[#141824] border border-[#22283a] text-amber-400">
              <CalendarIcon className="w-5 h-5" />
            </div>
            <div>
              <h3 className="font-serif font-bold text-lg text-slate-100 uppercase tracking-wider">
                Calendar & Agenda
              </h3>
              <p className="text-xs text-amber-300/80 font-serif">{timeFormatted}</p>
            </div>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close calendar"
            className="p-1.5 text-slate-400 hover:text-slate-100 rounded-xl hover:bg-slate-800 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Pending Advance Confirmation Warning */}
        {pendingAdvance ? (
          <div className="p-4 rounded-2xl bg-amber-500/10 border border-amber-500/30 space-y-3">
            <div className="flex items-center gap-2 text-amber-400 font-serif font-bold text-xs">
              <AlertTriangle className="w-4 h-4" />
              <span>Time Advancement Notice</span>
            </div>
            <p className="text-xs text-slate-200 font-serif leading-relaxed">
              {pendingAdvance.warning}
            </p>
            <div className="flex gap-2 justify-end pt-2">
              <button
                type="button"
                onClick={() => setPendingAdvance(null)}
                className="px-3 py-1.5 text-xs text-slate-300 hover:text-slate-100 font-serif"
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={handleConfirmAdvance}
                disabled={isLoading}
                className="bg-gradient-to-r from-amber-600 to-amber-500 text-slate-950 px-4 py-1.5 rounded-xl font-serif font-bold text-xs shadow-sm"
              >
                Proceed to Advance
              </button>
            </div>
          </div>
        ) : (
          <>
            {/* Upcoming Agenda Commitments */}
            <div className="space-y-2">
              <span className="text-[11px] font-serif text-slate-400 uppercase tracking-wider">
                Upcoming Life Commitments:
              </span>
              <div className="space-y-2">
                {upcomingEvents.map((ev, i) => (
                  <div
                    key={i}
                    className="bg-[#121622] border border-[#20273a] p-3 rounded-xl flex items-center justify-between text-xs"
                  >
                    <div>
                      <p className="font-serif font-bold text-slate-200">{ev.title}</p>
                      <p className="text-[11px] text-amber-400/80 font-mono">{ev.type}</p>
                    </div>
                    <span className="text-[11px] font-serif text-slate-400">{ev.time}</span>
                  </div>
                ))}
              </div>
            </div>

            {/* Time Advancement Options */}
            <div className="space-y-2 pt-2 border-t border-[#181e2e]">
              <span className="text-[11px] font-serif text-slate-400 uppercase tracking-wider">
                Advance Simulation Time:
              </span>
              <div className="grid grid-cols-2 gap-2">
                <button
                  type="button"
                  onClick={() => handleAdvanceClick('I spend an hour quietly reading and resting.', 'Wait 1 Hour')}
                  disabled={isLoading}
                  className="bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] p-2.5 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors flex items-center gap-2"
                >
                  <Clock className="w-3.5 h-3.5 text-slate-400" />
                  <span>Wait 1 Hour</span>
                </button>

                <button
                  type="button"
                  onClick={() => handleAdvanceClick('I sleep peacefully through the night and wake up refreshed in the morning.', 'Sleep until Morning')}
                  disabled={isLoading}
                  className="bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] p-2.5 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors flex items-center gap-2"
                >
                  <Moon className="w-3.5 h-3.5 text-slate-400" />
                  <span>Sleep</span>
                </button>

                <button
                  type="button"
                  onClick={() => handleAdvanceClick('I spend the entire day attending to quiet personal routines.', 'Advance 1 Day')}
                  disabled={isLoading}
                  className="bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] p-2.5 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors flex items-center gap-2"
                >
                  <CalendarIcon className="w-3.5 h-3.5 text-slate-400" />
                  <span>Advance 1 Day</span>
                </button>

                <button
                  type="button"
                  onClick={() =>
                    handleAdvanceClick(
                      'I follow my daily routine diligently for the next week.',
                      'Follow Routine (1 Week)',
                      'Advancing one week will progress ongoing commitments and schedules.'
                    )
                  }
                  disabled={isLoading}
                  className="bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] p-2.5 rounded-xl text-left text-xs font-serif text-slate-200 hover:text-amber-200 transition-colors flex items-center gap-2"
                >
                  <CheckCircle className="w-3.5 h-3.5 text-slate-400" />
                  <span>Routine (1 Week)</span>
                </button>
              </div>
            </div>
          </>
        )}

        <div className="pt-2 border-t border-[#1c2234] flex justify-end">
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
