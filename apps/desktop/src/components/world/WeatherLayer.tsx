import React from 'react';

interface WeatherLayerProps {
  weatherName?: string;
  isNight?: boolean;
}

export const WeatherLayer: React.FC<WeatherLayerProps> = ({
  weatherName = 'Harmattan Haze',
  isNight = false,
}) => {
  const wLower = weatherName.toLowerCase();
  const isRain = wLower.includes('rain') || wLower.includes('monsoon');
  const isHarmattan = wLower.includes('harmattan') || wLower.includes('dust') || wLower.includes('dry');

  return (
    <div className="absolute inset-0 pointer-events-none overflow-hidden z-10">
      {/* Sunlight Ray / Golden Hour Vignette */}
      {!isNight && !isRain && (
        <div
          className="absolute inset-0 opacity-40 mix-blend-screen"
          style={{
            background: 'radial-gradient(circle at 80% 20%, rgba(251, 191, 36, 0.25) 0%, rgba(245, 158, 11, 0.08) 40%, transparent 70%)',
          }}
        />
      )}

      {/* Harmattan Golden Dust Particles */}
      {isHarmattan && (
        <div
          className="absolute inset-0 opacity-30 mix-blend-color-dodge animate-pulse"
          style={{
            background: 'radial-gradient(ellipse at 50% 50%, rgba(245, 158, 11, 0.18) 0%, rgba(217, 119, 6, 0.05) 60%, transparent 100%)',
          }}
        />
      )}

      {/* Monsoon / Rain Streaks */}
      {isRain && (
        <div
          className="absolute inset-0 opacity-25"
          style={{
            backgroundImage: 'repeating-linear-gradient(105deg, transparent, transparent 15px, rgba(56, 189, 248, 0.4) 16px, transparent 18px)',
            backgroundSize: '100% 100%',
          }}
        />
      )}

      {/* Night Atmosphere */}
      {isNight && (
        <div
          className="absolute inset-0 opacity-60 mix-blend-multiply"
          style={{
            background: 'linear-gradient(to bottom, rgba(15, 23, 42, 0.7) 0%, rgba(2, 6, 23, 0.9) 100%)',
          }}
        />
      )}
    </div>
  );
};
