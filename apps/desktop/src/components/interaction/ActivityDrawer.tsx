import React, { useState } from 'react';
import { 
  Users, BookOpen, Heart, Activity, Briefcase, Rocket, Globe, 
  TrendingUp, Home, Smile, ShieldAlert, Vote, Award, ChevronRight, X, Sparkles, Send
} from 'lucide-react';

interface ActionItem {
  id: string;
  title: string;
  desc: string;
  intentText: string;
}

interface ActivityCategory {
  id: string;
  title: string;
  icon: React.ReactNode;
  minAge: number;
  maxAge?: number;
  badge?: string;
  actions: ActionItem[];
}

interface ActivityDrawerProps {
  playerAge: number;
  onSubmitIntent: (intentText: string) => void;
  isLoading: boolean;
}

export const ActivityDrawer: React.FC<ActivityDrawerProps> = ({
  playerAge,
  onSubmitIntent,
  isLoading,
}) => {
  const [selectedCategory, setSelectedCategory] = useState<ActivityCategory | null>(null);

  const categories: ActivityCategory[] = [
    // --- INFANCY CATEGORIES (Ages 0 - 3) ---
    {
      id: 'infant_family',
      title: 'Family & Nursery',
      icon: <Users className="w-4 h-4 text-amber-400" />,
      minAge: 0,
      maxAge: 3,
      actions: [
        { id: 'mom_cuddle', title: 'Cuddle with Mother', desc: 'Seek comfort and warmth in your mother’s embrace.', intentText: 'I cuddle with my mother and enjoy the comfort of her embrace.' },
        { id: 'dad_play', title: 'Spend Time with Father', desc: 'Listen to your father’s voice and gentle encouragement.', intentText: 'I spend quiet time with my father as he holds me and speaks softly.' },
        { id: 'first_words', title: 'Practice Babbling & First Words', desc: 'Try repeating sounds and names you hear around the room.', intentText: 'I practice babbling and attempting to speak my first words.' },
      ],
    },
    {
      id: 'infant_play',
      title: 'Play & Discovery',
      icon: <Smile className="w-4 h-4 text-emerald-400" />,
      minAge: 0,
      maxAge: 3,
      actions: [
        { id: 'toy_blocks', title: 'Play with Colorful Wooden Blocks', desc: 'Stack blocks and discover hand-eye coordination.', intentText: 'I play with colorful wooden blocks on the rug, stacking and toppling them.' },
        { id: 'picture_book', title: 'Flip through Picture Book', desc: 'Gaze at the brightly painted illustrations of animals and landscapes.', intentText: 'I point at the colorful picture book and look closely at the illustrations.' },
        { id: 'first_steps', title: 'Practice Crawling & Walking', desc: 'Push up on your feet and test your balance across the room.', intentText: 'I hold onto the furniture and practice taking careful first steps across the living room.' },
      ],
    },
    {
      id: 'infant_health',
      title: 'Rest & Care',
      icon: <Heart className="w-4 h-4 text-rose-400" />,
      minAge: 0,
      maxAge: 3,
      actions: [
        { id: 'infant_nap', title: 'Take a Restful Nap', desc: 'Drift off to sleep in your crib under the gentle daylight.', intentText: 'I take a peaceful afternoon nap in my crib to restore my energy.' },
        { id: 'pediatric_checkup', title: 'Pediatric Health Checkup', desc: 'Visit the clinic with your mother for routine vaccinations.', intentText: 'I visit the neighborhood clinic with my mother for routine health checkups and vaccination.' },
      ],
    },

    // --- CHILDHOOD CATEGORIES (Ages 4 - 12) ---
    {
      id: 'child_school',
      title: 'School & Academics',
      icon: <BookOpen className="w-4 h-4 text-blue-400" />,
      minAge: 4,
      maxAge: 12,
      actions: [
        { id: 'study_arithmetic', title: 'Practice Arithmetic & Reading', desc: 'Solve mathematics problems and practice reading literature.', intentText: 'I spend the afternoon doing arithmetic exercises and reading my schoolbooks carefully.' },
        { id: 'school_club', title: 'Join School Science & Debate Club', desc: 'Participate in after-school intellectual discussions and puzzles.', intentText: 'I attend the school Science and Debate Club to learn with fellow curious students.' },
        { id: 'befriend_classmate', title: 'Chat & Make Friends with Classmates', desc: 'Share snacks and stories during morning break time.', intentText: 'I share snacks and talk with my classmates during break time to build strong friendships.' },
        { id: 'school_prank', title: 'Play a Classroom Mischief / Prank', desc: 'Hide chalk or tell jokes when the teacher turns their back.', intentText: 'I pull a harmless, lighthearted joke during break time to make classmates laugh.' },
      ],
    },
    {
      id: 'child_sports',
      title: 'Youth Sports & Athletics',
      icon: <Activity className="w-4 h-4 text-orange-400" />,
      minAge: 4,
      maxAge: 12,
      actions: [
        { id: 'youth_football', title: 'Join Youth Football Team', desc: 'Practice passing drills and participate in neighborhood matches.', intentText: 'I join the youth football training session on the community field and practice ball control.' },
        { id: 'athletic_sprint', title: 'Track & Sprint Training', desc: 'Work on sprint stamina and speed across the grass field.', intentText: 'I run sprint intervals across the sports field to improve my stamina and physical fitness.' },
      ],
    },
    {
      id: 'child_family',
      title: 'Family, Chores & Birthdays',
      icon: <Smile className="w-4 h-4 text-amber-400" />,
      minAge: 4,
      maxAge: 12,
      actions: [
        { id: 'birthday_party', title: 'Celebrate Birthday with Family & Friends', desc: 'Organize a special celebration with cake, music, and gifts.', intentText: 'I celebrate my birthday with a joyful gathering of family and friends, sharing food and laughter.' },
        { id: 'help_chores', title: 'Help Parents with Household Chores', desc: 'Sweep the floor, wash dishes, and assist your mother in the kitchen.', intentText: 'I help my parents with household chores and tidying up the family home.' },
        { id: 'ask_pocket_money', title: 'Ask Parents for Pocket Money', desc: 'Politely request a small allowance for school snacks and stationery.', intentText: 'I politely ask my parents for a small pocket money allowance for stationery and treats.' },
      ],
    },

    // --- ADOLESCENCE CATEGORIES (Ages 13 - 17) ---
    {
      id: 'teen_academics',
      title: 'Exams & Mentorship',
      icon: <Award className="w-4 h-4 text-amber-400" />,
      minAge: 13,
      maxAge: 17,
      actions: [
        { id: 'waec_prep', title: 'Intensive National Exam Revision (WAEC / GCSE)', desc: 'Solve past examination papers and master core science & arts subjects.', intentText: 'I dedicate intensive evening study sessions to past examination papers in preparation for national certification.' },
        { id: 'private_tutor', title: 'Study with Private Tutor in Library', desc: 'Work through complex mathematics and physics theorems.', intentText: 'I spend Saturday mornings in the central library revising advanced curriculum subjects.' },
        { id: 'teacher_mentor', title: 'Seek Guidance from Lead Teacher', desc: 'Ask your teacher for academic advice and university recommendations.', intentText: 'I speak with my lead teacher after class to seek mentorship regarding university qualifications.' },
      ],
    },
    {
      id: 'teen_talent',
      title: 'Academy Trials & Coding',
      icon: <Activity className="w-4 h-4 text-emerald-400" />,
      minAge: 13,
      maxAge: 17,
      actions: [
        { id: 'football_scout_trial', title: 'Attend Football Academy Scouting Trials', desc: 'Perform in front of youth club talent scouts under stadium floodlights.', intentText: 'I lace up my boots and attend competitive youth football trials before academy scouts.' },
        { id: 'learn_programming', title: 'Practice Computer Programming & Algorithms', desc: 'Build software projects and write code late into the night.', intentText: 'I spend late nights at the computer studying software algorithms and building software projects.' },
      ],
    },
    {
      id: 'teen_social',
      title: 'Social Circle & Romance',
      icon: <Heart className="w-4 h-4 text-rose-400" />,
      minAge: 13,
      maxAge: 17,
      actions: [
        { id: 'ask_out_crush', title: 'Ask Out Someone You Like', desc: 'Invite your crush for ice cream or a walk after school.', intentText: 'I gather my courage and ask out my crush to spend time together after school.' },
        { id: 'hangout_friends', title: 'Weekend Hangout with Friends', desc: 'Gather with your peer group for music, games, and street food.', intentText: 'I meet up with close friends on the weekend to enjoy street food and listen to music.' },
      ],
    },

    // --- ADULTHOOD CATEGORIES (Ages 18+) ---
    {
      id: 'adult_career',
      title: 'Careers & Employment',
      icon: <Briefcase className="w-4 h-4 text-indigo-400" />,
      minAge: 18,
      actions: [
        { id: 'job_search', title: 'Apply for Professional Job Listings', desc: 'Submit curriculum vitae for corporate, engineering, or public sector roles.', intentText: 'I submit formal applications for open professional positions aligned with my qualifications.' },
        { id: 'work_overtime', title: 'Work Overtime & Demonstrate Diligence', desc: 'Put in extra hours to drive results and gain recognition.', intentText: 'I put in extra hours at work to complete key company deliverables and prove my dedication.' },
        { id: 'ask_promotion', title: 'Request Salary Raise & Promotion', desc: 'Schedule a performance meeting with your manager to negotiate compensation.', intentText: 'I schedule a formal performance review with my manager to request a salary increase and promotion.' },
        { id: 'resign_job', title: 'Resign to Pursue New Horizons', desc: 'Hand in your notice to pivot careers or start your own venture.', intentText: 'I submit my resignation letter professionally to explore new life opportunities.' },
      ],
    },
    {
      id: 'adult_business',
      title: 'Business & Entrepreneurship',
      icon: <Rocket className="w-4 h-4 text-amber-400" />,
      minAge: 18,
      actions: [
        { id: 'register_company', title: 'Register a New Company / LLC', desc: 'Incorporate your business entity (CAC, Companies House, or State Filing).', intentText: 'I formally incorporate a new limited liability company with commercial authorities.' },
        { id: 'pitch_investors', title: 'Pitch Investors & Apply for Business Loan', desc: 'Present your business plan to angel investors and commercial banks for capital.', intentText: 'I pitch our commercial growth plan to potential investors and apply for growth capital.' },
        { id: 'hire_employees', title: 'Hire Key Talent & Scale Operations', desc: 'Recruit engineers, sales leads, and operations managers.', intentText: 'I recruit and hire skilled team members to scale business operations and product delivery.' },
        { id: 'launch_product', title: 'Launch Major Marketing Campaign', desc: 'Run advertising campaigns to drive market adoption and revenue.', intentText: 'I launch a comprehensive marketing and distribution campaign for our commercial products.' },
      ],
    },
    {
      id: 'adult_travel',
      title: 'Travel, Visas & Emigration',
      icon: <Globe className="w-4 h-4 text-cyan-400" />,
      minAge: 18,
      actions: [
        { id: 'vacation_trip', title: 'Take a Relaxing Vacation Trip', desc: 'Book tickets for a scenic holiday to recharge and experience new cultures.', intentText: 'I travel on a well-deserved vacation to explore new sights and refresh my mind.' },
        { id: 'apply_visa', title: 'Apply for International Student / Work Visa', desc: 'Submit embassy visa paperwork for international relocation or study abroad.', intentText: 'I submit official visa documentation at the embassy to pursue international opportunities.' },
        { id: 'relocate_city', title: 'Relocate to Another Major City', desc: 'Move your residence to a bustling commercial hub (Lagos, London, New York).', intentText: 'I pack my belongings and relocate to a new major city for expanded economic horizons.' },
      ],
    },
    {
      id: 'adult_wealth',
      title: 'Wealth & Investments',
      icon: <TrendingUp className="w-4 h-4 text-emerald-400" />,
      minAge: 18,
      actions: [
        { id: 'stock_market', title: 'Invest in Equities & Index Funds', desc: 'Allocate capital into blue-chip stocks and diversified market funds.', intentText: 'I invest surplus savings into high-quality equities and index funds for long-term compound growth.' },
        { id: 'fixed_deposit', title: 'Open High-Yield Fixed Deposit Account', desc: 'Lock funds in secure interest-bearing bank instruments.', intentText: 'I deposit capital into a high-yield treasury account to generate passive interest income.' },
      ],
    },
    {
      id: 'adult_assets',
      title: 'Real Estate & Assets',
      icon: <Home className="w-4 h-4 text-orange-400" />,
      minAge: 18,
      actions: [
        { id: 'rent_apartment', title: 'Rent Modern City Apartment', desc: 'Lease a comfortable home in a desirable neighborhood.', intentText: 'I sign a lease for a modern, well-located apartment in the city.' },
        { id: 'buy_property', title: 'Purchase Real Estate Property', desc: 'Acquire residential or commercial land to build wealth and collect rental yield.', intentText: 'I purchase real estate property to expand my asset portfolio and establish permanent roots.' },
        { id: 'buy_vehicle', title: 'Purchase Personal Vehicle', desc: 'Buy a reliable car for daily commuting and regional travel.', intentText: 'I purchase a reliable automobile for personal and family transport.' },
      ],
    },
    {
      id: 'adult_romance',
      title: 'Love, Family & Marriage',
      icon: <Heart className="w-4 h-4 text-rose-400" />,
      minAge: 18,
      actions: [
        { id: 'date_partner', title: 'Go on Romantic Dinner Date', desc: 'Spend quality evening hours with your partner at a fine restaurant.', intentText: 'I take my partner out for a romantic dinner to deepen our bond and share life aspirations.' },
        { id: 'propose_marriage', title: 'Propose Marriage & Plan Wedding', desc: 'Ask your partner to share life together and plan a celebration.', intentText: 'I propose marriage to my partner and begin planning a beautiful wedding with family.' },
        { id: 'start_family', title: 'Welcome Children into the Family', desc: 'Begin the journey of parenthood and raise the next generation.', intentText: 'We decide to grow our family and prepare a loving home to raise children.' },
      ],
    },
    {
      id: 'adult_civic',
      title: 'Politics, Charity & Civic Life',
      icon: <Vote className="w-4 h-4 text-purple-400" />,
      minAge: 18,
      actions: [
        { id: 'charity_donation', title: 'Donate to Community Philanthropy', desc: 'Support local orphanages, schools, and medical clinics.', intentText: 'I donate financial resources and supplies to local community charity initiatives.' },
        { id: 'run_office', title: 'Run for Local / Regional Political Office', desc: 'Launch a civic campaign to advocate for public infrastructure and education.', intentText: 'I launch a civic campaign to run for public office and serve our community.' },
      ],
    },
    {
      id: 'adult_risk',
      title: 'Underworld & High Risk',
      icon: <ShieldAlert className="w-4 h-4 text-red-400" />,
      minAge: 18,
      actions: [
        { id: 'smuggling_deal', title: 'Attempt High-Risk Commercial Arbitrage', desc: 'Pursue an illicit trade deal with high return and legal peril.', intentText: 'I engage in an aggressive, high-risk commercial deal on the fringes of the market.' },
        { id: 'hire_defense_attorney', title: 'Retain Top Legal Defense Counsel', desc: 'Hire prestigious attorneys to protect your rights and handle disputes.', intentText: 'I retain experienced legal defense counsel to protect my assets and handle disputes.' },
      ],
    },
  ];

  // Filter categories available at player's current age
  const availableCategories = categories.filter(
    (c) => playerAge >= c.minAge && (c.maxAge === undefined || playerAge <= c.maxAge)
  );

  return (
    <div className="bg-[#0a0c12] border-t border-[#1c2130] p-4 lg:px-10 select-none font-sans z-30 shadow-2xl">
      <div className="max-w-4xl mx-auto space-y-3">
        {/* Header Label */}
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-xs font-serif text-amber-300">
            <Sparkles className="w-3.5 h-3.5 text-amber-400" />
            <span className="tracking-wider uppercase font-mono text-[11px]">Available Life Activities (Age {playerAge})</span>
          </div>
          <span className="text-[11px] text-slate-500 font-serif italic">Select an activity to take action</span>
        </div>

        {/* Horizontal Category Cards */}
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2.5">
          {availableCategories.map((cat) => (
            <button
              key={cat.id}
              type="button"
              onClick={() => setSelectedCategory(cat)}
              disabled={isLoading}
              className="bg-[#121622] hover:bg-[#181e2e] border border-[#20273a] hover:border-amber-500/50 p-3 rounded-2xl flex items-center justify-between text-left transition-all duration-200 group shadow-sm"
            >
              <div className="flex items-center gap-2.5 overflow-hidden">
                <div className="p-2 rounded-xl bg-[#181d2c] border border-[#242c3e] group-hover:scale-105 transition-transform">
                  {cat.icon}
                </div>
                <div className="truncate">
                  <div className="font-serif font-bold text-xs text-slate-100 group-hover:text-amber-200 truncate">
                    {cat.title}
                  </div>
                  <div className="text-[10px] text-slate-500 font-sans truncate">
                    {cat.actions.length} actions
                  </div>
                </div>
              </div>
              <ChevronRight className="w-4 h-4 text-slate-600 group-hover:text-amber-400 group-hover:translate-x-0.5 transition-all flex-shrink-0" />
            </button>
          ))}
        </div>
      </div>

      {/* Action Selection Modal */}
      {selectedCategory && (
        <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
          <div className="bg-[#0e1118] border border-amber-500/30 rounded-3xl max-w-lg w-full p-6 space-y-5 shadow-2xl animate-fadeIn">
            {/* Modal Header */}
            <div className="flex items-center justify-between border-b border-[#1c2130] pb-4">
              <div className="flex items-center gap-3">
                <div className="p-2.5 rounded-2xl bg-[#141824] border border-[#22283a]">
                  {selectedCategory.icon}
                </div>
                <div>
                  <h3 className="font-serif font-bold text-lg text-slate-100">{selectedCategory.title}</h3>
                  <p className="text-xs text-slate-400 font-sans">Choose your next life intention</p>
                </div>
              </div>
              <button
                type="button"
                onClick={() => setSelectedCategory(null)}
                className="p-1.5 text-slate-400 hover:text-slate-100 rounded-xl hover:bg-slate-800 transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Action Items List */}
            <div className="space-y-2.5 max-h-80 overflow-y-auto pr-1">
              {selectedCategory.actions.map((act) => (
                <button
                  key={act.id}
                  type="button"
                  onClick={() => {
                    onSubmitIntent(act.intentText);
                    setSelectedCategory(null);
                  }}
                  disabled={isLoading}
                  className="w-full bg-[#121622] hover:bg-[#1a2133] border border-[#20273a] hover:border-amber-500/50 p-4 rounded-2xl text-left transition-all duration-200 space-y-1 group shadow-sm"
                >
                  <div className="flex justify-between items-center">
                    <span className="font-serif font-bold text-sm text-slate-100 group-hover:text-amber-200">
                      {act.title}
                    </span>
                    <Send className="w-3.5 h-3.5 text-slate-600 group-hover:text-amber-400 group-hover:translate-x-0.5 transition-all" />
                  </div>
                  <p className="text-xs text-slate-400 font-sans leading-relaxed">
                    {act.desc}
                  </p>
                </button>
              ))}
            </div>

            {/* Modal Footer */}
            <div className="pt-2 border-t border-[#1c2130] flex justify-end">
              <button
                type="button"
                onClick={() => setSelectedCategory(null)}
                className="text-xs text-slate-400 hover:text-slate-200 font-serif px-4 py-2"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
