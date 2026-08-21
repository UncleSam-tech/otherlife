import React from 'react';

interface EnvironmentLayerProps {
  lifeStage: string;
  age: number;
  locationFormatted: string;
}

export const EnvironmentLayer: React.FC<EnvironmentLayerProps> = ({
  lifeStage,
  age,
}) => {
  const stageLower = lifeStage.toLowerCase();

  const renderSceneIllustration = () => {
    if (stageLower.includes('infant') || age <= 3) {
      // Age 0-3: Family Home, Bedroom, Crib, Toys & Sunlit Rug
      return (
        <svg viewBox="0 0 800 320" className="w-full h-full object-cover">
          <defs>
            <linearGradient id="infantWall" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#221c24" />
              <stop offset="100%" stopColor="#141118" />
            </linearGradient>
            <linearGradient id="infantSun" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stopColor="#fde68a" stopOpacity="0.4" />
              <stop offset="60%" stopColor="#f59e0b" stopOpacity="0.1" />
              <stop offset="100%" stopColor="#f59e0b" stopOpacity="0.0" />
            </linearGradient>
            <linearGradient id="rugPattern" x1="0" y1="0" x2="1" y2="0">
              <stop offset="0%" stopColor="#9a3412" />
              <stop offset="50%" stopColor="#c2410c" />
              <stop offset="100%" stopColor="#7c2d12" />
            </linearGradient>
          </defs>

          {/* Wall & Floor */}
          <rect width="800" height="235" fill="url(#infantWall)" />
          <rect y="235" width="800" height="85" fill="#0f0c12" />

          {/* Window & Curtains */}
          <rect x="70" y="25" width="170" height="160" rx="6" fill="#fef3c7" fillOpacity="0.12" stroke="#3e3542" strokeWidth="4" />
          <line x1="155" y1="25" x2="155" y2="185" stroke="#3e3542" strokeWidth="3" />
          <line x1="70" y1="105" x2="240" y2="105" stroke="#3e3542" strokeWidth="3" />
          
          {/* Curtains */}
          <path d="M55,20 Q70,90 60,190 L75,190 Q85,90 70,20 Z" fill="#881337" opacity="0.85" />
          <path d="M255,20 Q240,90 250,190 L235,190 Q225,90 240,20 Z" fill="#881337" opacity="0.85" />

          {/* Sunbeams Streaming In */}
          <polygon points="70,25 240,25 540,320 200,320" fill="url(#infantSun)" />

          {/* Wooden Baby Crib */}
          <rect x="290" y="110" width="130" height="120" rx="4" fill="none" stroke="#78350f" strokeWidth="4" />
          <line x1="310" y1="110" x2="310" y2="230" stroke="#92400e" strokeWidth="2.5" />
          <line x1="330" y1="110" x2="330" y2="230" stroke="#92400e" strokeWidth="2.5" />
          <line x1="350" y1="110" x2="350" y2="230" stroke="#92400e" strokeWidth="2.5" />
          <line x1="370" y1="110" x2="370" y2="230" stroke="#92400e" strokeWidth="2.5" />
          <line x1="390" y1="110" x2="390" y2="230" stroke="#92400e" strokeWidth="2.5" />
          <line x1="410" y1="110" x2="410" y2="230" stroke="#92400e" strokeWidth="2.5" />
          <rect x="295" y="180" width="120" height="45" rx="3" fill="#fef08a" opacity="0.8" />

          {/* Patterned Rug */}
          <ellipse cx="480" cy="275" rx="240" ry="36" fill="url(#rugPattern)" />
          <ellipse cx="480" cy="275" rx="215" ry="28" fill="none" stroke="#fed7aa" strokeWidth="1.5" strokeDasharray="5 3" opacity="0.7" />

          {/* Scattered Wooden Blocks */}
          <rect x="420" y="262" width="22" height="22" rx="2" fill="#ef4444" />
          <rect x="446" y="262" width="22" height="22" rx="2" fill="#10b981" />
          <rect x="433" y="241" width="22" height="22" rx="2" fill="#f59e0b" />
          <rect x="510" y="265" width="26" height="18" rx="2" fill="#3b82f6" />

          {/* Picture Book Open on Rug */}
          <path d="M560,268 Q580,262 600,268 L602,282 Q580,276 560,282 Z" fill="#fefce8" />
          <path d="M600,268 Q620,262 640,268 L642,282 Q620,276 600,282 Z" fill="#fef9c3" />
          <line x1="600" y1="268" x2="601" y2="282" stroke="#71717a" strokeWidth="1.5" />

          {/* Parent Tea Cups on Side Table */}
          <rect x="680" y="180" width="80" height="80" rx="4" fill="#451a03" />
          <rect x="700" y="165" width="16" height="16" rx="2" fill="#e2e8f0" />
          <path d="M708,155 Q712,145 706,140 Q710,135 708,130" stroke="#f8fafc" strokeWidth="1" fill="none" opacity="0.6" />
        </svg>
      );
    } else if (stageLower.includes('child') || (age > 3 && age <= 12)) {
      // Age 4-12: Primary School Classroom & Outdoor Courtyard
      return (
        <svg viewBox="0 0 800 320" className="w-full h-full object-cover">
          <defs>
            <linearGradient id="childWall" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#1e293b" />
              <stop offset="100%" stopColor="#0f172a" />
            </linearGradient>
            <linearGradient id="chalkboardGrad" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#14532d" />
              <stop offset="100%" stopColor="#052e16" />
            </linearGradient>
          </defs>

          {/* Classroom Wall & Floor */}
          <rect width="800" height="225" fill="url(#childWall)" />
          <rect y="225" width="800" height="95" fill="#171717" />

          {/* Large Chalkboard */}
          <rect x="220" y="25" width="370" height="145" rx="6" fill="url(#chalkboardGrad)" stroke="#78350f" strokeWidth="6" />
          <text x="245" y="65" fill="#86efac" fontSize="14" fontFamily="Georgia, serif" opacity="0.95">
            Primary Education · Arithmetic & Reading
          </text>
          <text x="245" y="100" fill="#fef08a" fontSize="16" fontFamily="Courier, monospace">
            25 × 16 = 400     |     √169 = 13
          </text>
          <text x="245" y="132" fill="#f1f5f9" fontSize="13" fontFamily="Courier, monospace" opacity="0.8">
            "Diligent practice leads to mastery."
          </text>

          {/* Classroom Window Looking Out to Courtyard Trees */}
          <rect x="45" y="30" width="140" height="140" rx="4" fill="#38bdf8" fillOpacity="0.15" stroke="#475569" strokeWidth="3" />
          <line x1="115" y1="30" x2="115" y2="170" stroke="#475569" strokeWidth="2" />
          {/* Courtyard Greenery */}
          <circle cx="95" cy="110" r="28" fill="#15803d" opacity="0.8" />
          <circle cx="130" cy="120" r="24" fill="#16a34a" opacity="0.7" />

          {/* Wooden Student Desks */}
          <rect x="190" y="240" width="170" height="60" rx="4" fill="#78350f" />
          <rect x="440" y="240" width="170" height="60" rx="4" fill="#78350f" />

          {/* Stacked Textbooks & Pencils */}
          <rect x="220" y="234" width="45" height="8" rx="1" fill="#2563eb" />
          <rect x="225" y="227" width="40" height="8" rx="1" fill="#d97706" />
          <line x1="280" y1="238" x2="305" y2="238" stroke="#dc2626" strokeWidth="2.5" strokeLinecap="round" />

          {/* Football Resting Near Desk */}
          <circle cx="130" cy="275" r="22" fill="#f8fafc" stroke="#18181b" strokeWidth="2.5" />
          <polygon points="130,263 139,271 136,282 124,282 121,271" fill="#18181b" />
        </svg>
      );
    } else {
      // Age 13+: Adolescence & Adulthood - Evening Desk, Sports Ground & Urban Horizons
      return (
        <svg viewBox="0 0 800 320" className="w-full h-full object-cover">
          <defs>
            <linearGradient id="teenSky" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#090d16" />
              <stop offset="50%" stopColor="#111827" />
              <stop offset="100%" stopColor="#1e1b4b" />
            </linearGradient>
            <linearGradient id="lampGlow" x1="0" y1="0" x2="1" y2="1">
              <stop offset="0%" stopColor="#fbbf24" stopOpacity="0.45" />
              <stop offset="60%" stopColor="#f59e0b" stopOpacity="0.12" />
              <stop offset="100%" stopColor="#d97706" stopOpacity="0.0" />
            </linearGradient>
          </defs>

          {/* Room Background */}
          <rect width="800" height="235" fill="url(#teenSky)" />
          <rect y="235" width="800" height="85" fill="#080a0f" />

          {/* Window Overlooking City & Distant Stadium Floodlights */}
          <rect x="55" y="25" width="230" height="155" rx="6" fill="#020617" stroke="#334155" strokeWidth="4" />
          {/* City Silhouettes */}
          <rect x="85" y="85" width="32" height="95" fill="#1e293b" />
          <rect x="125" y="55" width="45" height="125" fill="#0f172a" />
          <rect x="180" y="75" width="38" height="105" fill="#1e293b" />
          <rect x="228" y="105" width="38" height="75" fill="#0f172a" />
          {/* Glowing Windows */}
          <circle cx="140" cy="70" r="2" fill="#fef08a" />
          <circle cx="150" cy="90" r="2" fill="#fef08a" />
          <circle cx="95" cy="105" r="2" fill="#fef08a" />
          <circle cx="195" cy="95" r="2" fill="#fef08a" />

          {/* Study Desk Surface */}
          <rect x="330" y="175" width="460" height="145" fill="#1a1c23" />

          {/* Desk Lamp */}
          <path d="M410,240 L435,115 L475,125" fill="none" stroke="#64748b" strokeWidth="4" strokeLinecap="round" />
          <path d="M465,110 L505,130 L480,150 Z" fill="#cbd5e1" stroke="#94a3b8" strokeWidth="2" />
          <polygon points="485,135 690,310 370,310" fill="url(#lampGlow)" />

          {/* Laptop / Computer Workspace */}
          <rect x="520" y="190" width="115" height="75" rx="3" fill="#0f172a" stroke="#475569" strokeWidth="2" />
          <rect x="526" y="195" width="103" height="63" rx="2" fill="#0284c7" fillOpacity="0.35" />
          <line x1="535" y1="210" x2="575" y2="210" stroke="#38bdf8" strokeWidth="2" strokeLinecap="round" />
          <line x1="535" y1="222" x2="610" y2="222" stroke="#4ade80" strokeWidth="2" strokeLinecap="round" />
          <line x1="535" y1="234" x2="565" y2="234" stroke="#fef08a" strokeWidth="2" strokeLinecap="round" />
          <polygon points="505,268 650,268 660,276 495,276" fill="#334155" />

          {/* Exam Question Papers & Textbooks */}
          <rect x="660" y="222" width="95" height="15" rx="2" fill="#991b1b" />
          <rect x="665" y="206" width="88" height="15" rx="2" fill="#1e40af" />
          <rect x="662" y="190" width="92" height="15" rx="2" fill="#065f46" />
          <text x="672" y="202" fill="#ffffff" fontSize="9" fontFamily="sans-serif" opacity="0.9">MATHEMATICS</text>
        </svg>
      );
    }
  };

  return (
    <div className="relative w-full h-48 md:h-60 rounded-2xl overflow-hidden shadow-2xl bg-[#090b10] border border-amber-500/20">
      {renderSceneIllustration()}
      <div className="absolute inset-0 bg-gradient-to-t from-[#090b10] via-transparent to-transparent pointer-events-none" />
    </div>
  );
};
