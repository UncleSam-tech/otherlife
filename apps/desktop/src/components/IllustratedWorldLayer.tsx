import React from 'react';

interface IllustratedWorldLayerProps {
  lifeStage: string;
  age: number;
  locationFormatted: string;
  timeFormatted: string;
}

export const IllustratedWorldLayer: React.FC<IllustratedWorldLayerProps> = ({
  lifeStage,
  age,
  locationFormatted,
  timeFormatted,
}) => {
  const stageLower = lifeStage.toLowerCase();

  // Render SVG 2D Illustrated Environment according to Life Stage
  const renderStageIllustration = () => {
    if (stageLower.includes('infant') || age <= 3) {
      // Infancy: Sunlit Family Nursery & Living Room
      return (
        <svg viewBox="0 0 800 320" className="w-full h-full object-cover">
          <defs>
            <linearGradient id="infantSky" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#2a1f1d" />
              <stop offset="100%" stopColor="#151216" />
            </linearGradient>
            <linearGradient id="sunbeam" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stopColor="#f5c068" stopOpacity="0.45" />
              <stop offset="50%" stopColor="#e5a950" stopOpacity="0.15" />
              <stop offset="100%" stopColor="#f5c068" stopOpacity="0.0" />
            </linearGradient>
            <linearGradient id="rugGrad" x1="0" y1="0" x2="1" y2="0">
              <stop offset="0%" stopColor="#9b4222" />
              <stop offset="50%" stopColor="#c25e2e" />
              <stop offset="100%" stopColor="#873519" />
            </linearGradient>
            <linearGradient id="wallGrad" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#2c2730" />
              <stop offset="100%" stopColor="#19161c" />
            </linearGradient>
          </defs>

          {/* Room Background Wall */}
          <rect width="800" height="230" fill="url(#wallGrad)" />
          <rect y="230" width="800" height="90" fill="#141114" />

          {/* Window Frame with Morning Light */}
          <rect x="80" y="30" width="160" height="150" rx="6" fill="#fdf3dc" fillOpacity="0.15" stroke="#4a3e3d" strokeWidth="4" />
          <line x1="160" y1="30" x2="160" y2="180" stroke="#4a3e3d" strokeWidth="3" />
          <line x1="80" y1="105" x2="240" y2="105" stroke="#4a3e3d" strokeWidth="3" />

          {/* Sunbeams casting across the room */}
          <polygon points="80,30 240,30 520,320 220,320" fill="url(#sunbeam)" />

          {/* Woven Warm Pattern Rug */}
          <ellipse cx="400" cy="275" rx="260" ry="38" fill="url(#rugGrad)" />
          <ellipse cx="400" cy="275" rx="230" ry="30" fill="none" stroke="#f6d396" strokeWidth="2" strokeDasharray="6 4" opacity="0.6" />

          {/* Wooden Building Blocks on the Rug */}
          <rect x="340" y="260" width="22" height="22" rx="2" fill="#e76f51" />
          <rect x="366" y="260" width="22" height="22" rx="2" fill="#2a9d8f" />
          <rect x="353" y="239" width="22" height="22" rx="2" fill="#e9c46a" />
          <rect x="420" y="264" width="24" height="18" rx="2" fill="#457b9d" />

          {/* Picture Book Open on Rug */}
          <path d="M470,268 Q490,262 510,268 L512,282 Q490,276 470,282 Z" fill="#fefae0" />
          <path d="M510,268 Q530,262 550,268 L552,282 Q530,276 510,282 Z" fill="#fcf6bd" />
          <line x1="510" y1="268" x2="511" y2="282" stroke="#6b705c" strokeWidth="1.5" />

          {/* Soft Living Room Sofa in Background */}
          <rect x="560" y="140" width="180" height="95" rx="10" fill="#3a3036" />
          <rect x="580" y="120" width="140" height="50" rx="8" fill="#483d44" />
          <ellipse cx="610" cy="180" rx="25" ry="15" fill="#e5a950" opacity="0.8" />
          <ellipse cx="680" cy="180" rx="25" ry="15" fill="#c25e2e" opacity="0.8" />

          {/* Small Potted Plant on Window Sill */}
          <rect x="200" y="160" width="24" height="20" rx="3" fill="#8d5b4c" />
          <path d="M204,160 Q212,140 200,135 Q212,145 212,160" fill="#40916c" />
          <path d="M216,160 Q225,138 238,142 Q225,148 220,160" fill="#52b788" />
        </svg>
      );
    } else if (stageLower.includes('child') || (age > 3 && age <= 12)) {
      // Childhood: Sunlit Primary School Classroom & Outdoor Courtyard
      return (
        <svg viewBox="0 0 800 320" className="w-full h-full object-cover">
          <defs>
            <linearGradient id="childSky" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#1e3a5f" />
              <stop offset="100%" stopColor="#101c2b" />
            </linearGradient>
            <linearGradient id="boardGrad" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#1b4332" />
              <stop offset="100%" stopColor="#081c15" />
            </linearGradient>
          </defs>

          {/* Classroom Wall & Floor */}
          <rect width="800" height="220" fill="url(#childSky)" />
          <rect y="220" width="800" height="100" fill="#1b1d22" />

          {/* Large Chalkboard */}
          <rect x="220" y="30" width="360" height="140" rx="6" fill="url(#boardGrad)" stroke="#6b5b45" strokeWidth="6" />
          {/* Chalkboard Mathematics & Lesson */}
          <text x="245" y="70" fill="#d8f3dc" fontSize="15" fontFamily="Georgia, serif" opacity="0.9">
            Primary Mathematics · Arithmetic
          </text>
          <text x="245" y="105" fill="#fefae0" fontSize="16" fontFamily="Courier, monospace" opacity="0.85">
            24 × 15 = 360     |     √144 = 12
          </text>
          <text x="245" y="135" fill="#fefae0" fontSize="14" fontFamily="Courier, monospace" opacity="0.75">
            "Practice builds mastery."
          </text>

          {/* Classroom Window Viewing Courtyard */}
          <rect x="50" y="40" width="130" height="130" rx="4" fill="#a0c4e2" fillOpacity="0.2" stroke="#4a5568" strokeWidth="3" />
          <line x1="115" y1="40" x2="115" y2="170" stroke="#4a5568" strokeWidth="2" />
          {/* Trees visible through window */}
          <circle cx="100" cy="110" r="28" fill="#2d6a4f" opacity="0.7" />
          <circle cx="130" cy="115" r="22" fill="#40916c" opacity="0.6" />

          {/* Student Desks */}
          <rect x="200" y="235" width="160" height="55" rx="4" fill="#5c4033" />
          <rect x="440" y="235" width="160" height="55" rx="4" fill="#5c4033" />

          {/* Books & Pencils on Desk */}
          <rect x="230" y="230" width="40" height="8" rx="1" fill="#3a86ff" />
          <rect x="235" y="224" width="35" height="7" rx="1" fill="#ffbe0b" />
          <line x1="285" y1="234" x2="310" y2="234" stroke="#e63946" strokeWidth="2.5" strokeLinecap="round" />

          {/* Football resting near desk */}
          <circle cx="140" cy="275" r="20" fill="#f8f9fa" stroke="#212529" strokeWidth="2" />
          <polygon points="140,265 148,272 145,282 135,282 132,272" fill="#212529" />
        </svg>
      );
    } else {
      // Adolescence & Youth: Evening Study Lamp, Sports Ground & University Horizon
      return (
        <svg viewBox="0 0 800 320" className="w-full h-full object-cover">
          <defs>
            <linearGradient id="teenSky" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#0f172a" />
              <stop offset="60%" stopColor="#1e1b4b" />
              <stop offset="100%" stopColor="#2e1065" />
            </linearGradient>
            <linearGradient id="deskGlow" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stopColor="#fbbf24" stopOpacity="0.5" />
              <stop offset="50%" stopColor="#f59e0b" stopOpacity="0.15" />
              <stop offset="100%" stopColor="#d97706" stopOpacity="0.0" />
            </linearGradient>
          </defs>

          {/* Evening Room Background */}
          <rect width="800" height="230" fill="url(#teenSky)" />
          <rect y="230" width="800" height="90" fill="#0b0e14" />

          {/* Window Showing City Skyline and Distant Stadium Lights */}
          <rect x="60" y="30" width="220" height="150" rx="6" fill="#020617" stroke="#334155" strokeWidth="4" />
          {/* Distant building silhouettes */}
          <rect x="90" y="90" width="30" height="90" fill="#1e293b" />
          <rect x="130" y="60" width="40" height="120" fill="#0f172a" />
          <rect x="180" y="80" width="35" height="100" fill="#1e293b" />
          <rect x="225" y="110" width="35" height="70" fill="#0f172a" />

          {/* Little lit windows in city */}
          <circle cx="145" cy="75" r="2" fill="#fef08a" />
          <circle cx="155" cy="95" r="2" fill="#fef08a" />
          <circle cx="100" cy="110" r="2" fill="#fef08a" />
          <circle cx="195" cy="100" r="2" fill="#fef08a" />

          {/* Desk Surface */}
          <rect x="340" y="180" width="440" height="140" fill="#1e1e24" />

          {/* Warm Study Desk Lamp */}
          <path d="M420,240 L440,120 L480,130" fill="none" stroke="#64748b" strokeWidth="4" strokeLinecap="round" />
          <path d="M470,115 L510,135 L485,155 Z" fill="#e2e8f0" stroke="#94a3b8" strokeWidth="2" />
          <polygon points="490,140 680,310 380,310" fill="url(#deskGlow)" />

          {/* Laptop / Computer on Desk */}
          <rect x="520" y="195" width="110" height="70" rx="3" fill="#0f172a" stroke="#475569" strokeWidth="2" />
          <rect x="526" y="200" width="98" height="58" rx="2" fill="#0284c7" fillOpacity="0.4" />
          <line x1="535" y1="215" x2="570" y2="215" stroke="#38bdf8" strokeWidth="2" strokeLinecap="round" />
          <line x1="535" y1="225" x2="600" y2="225" stroke="#a7f3d0" strokeWidth="2" strokeLinecap="round" />
          <line x1="535" y1="235" x2="560" y2="235" stroke="#fef08a" strokeWidth="2" strokeLinecap="round" />
          <polygon points="505,270 645,270 655,278 495,278" fill="#334155" />

          {/* Stacks of Exam Textbooks & Notes */}
          <rect x="660" y="225" width="90" height="14" rx="2" fill="#b91c1c" />
          <rect x="665" y="210" width="82" height="14" rx="2" fill="#1d4ed8" />
          <rect x="662" y="195" width="86" height="14" rx="2" fill="#047857" />
          <text x="670" y="206" fill="#ffffff" fontSize="9" fontFamily="sans-serif" opacity="0.9">MATHEMATICS</text>
        </svg>
      );
    }
  };

  return (
    <div className="relative w-full h-48 md:h-56 rounded-2xl overflow-hidden shadow-2xl border border-amber-500/20 bg-slate-950 group">
      {/* Background Illustrated SVG Canvas */}
      {renderStageIllustration()}

      {/* Atmospheric Vignette & Soft Gradient Overlay */}
      <div className="absolute inset-0 bg-gradient-to-t from-slate-950 via-slate-950/40 to-transparent pointer-events-none" />

      {/* Floating Header Card atop Illustration */}
      <div className="absolute bottom-3 left-4 right-4 flex items-center justify-between text-xs pointer-events-none">
        <div className="flex items-center gap-2 bg-slate-950/85 backdrop-blur-md px-3.5 py-1.5 rounded-full border border-amber-500/30 text-amber-200 font-serif">
          <span className="w-2 h-2 rounded-full bg-amber-400 animate-pulse" />
          <span>{locationFormatted}</span>
          <span className="text-slate-500">·</span>
          <span className="text-slate-300 font-sans">{timeFormatted}</span>
        </div>

        <div className="bg-slate-950/85 backdrop-blur-md px-3 py-1.5 rounded-full border border-slate-800 text-slate-300 font-serif tracking-wide text-xs">
          Age {age} · {lifeStage}
        </div>
      </div>
    </div>
  );
};
