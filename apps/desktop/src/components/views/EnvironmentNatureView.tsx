import React from 'react';
import { Sun, CloudRain, ShieldAlert, Hammer, Wind, TreePine } from 'lucide-react';

interface EnvironmentNatureViewProps {
  season: string;
  condition: string;
  temperatureCelsius: number;
  airQualityIndex: number;
  activeDisastersCount: number;
  disasterType?: string;
  damageCost?: number;
  onSimulateWeather: () => void;
  onTriggerDisaster: () => void;
  onRebuildInfrastructure: () => void;
}

export const EnvironmentNatureView: React.FC<EnvironmentNatureViewProps> = ({
  season,
  condition,
  temperatureCelsius,
  airQualityIndex,
  activeDisastersCount,
  disasterType,
  damageCost,
  onSimulateWeather,
  onTriggerDisaster,
  onRebuildInfrastructure,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Sun size={22} color="var(--accent-amber)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Environment, Nature & Disasters
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onSimulateWeather}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-cyan)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <CloudRain size={16} />
            <span>Simulate Weather Shift</span>
          </button>

          <button
            onClick={onTriggerDisaster}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-rose)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <ShieldAlert size={16} />
            <span>Disaster Emergency Alert</span>
          </button>

          <button
            onClick={onRebuildInfrastructure}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-emerald)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Hammer size={16} />
            <span>Rebuild Infrastructure</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CURRENT SEASON</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)' }}>
            {season || 'SUMMER'} ({condition || 'SUNNY'})
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>REGIONAL TEMPERATURE</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-rose)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Sun size={18} />
            <span>{temperatureCelsius.toFixed(1)}°C</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>AIR QUALITY INDEX (AQI)</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Wind size={18} />
            <span>{airQualityIndex} (Good)</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>ACTIVE DISASTERS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: activeDisastersCount > 0 ? 'var(--accent-rose)' : 'var(--accent-emerald)' }}>
            {activeDisastersCount} Active Emergency
          </div>
        </div>
      </div>

      {activeDisastersCount > 0 && (
        <div style={{ backgroundColor: 'rgba(244, 63, 94, 0.1)', padding: '18px', borderRadius: 'var(--radius-md)', border: '1px solid var(--accent-rose)' }}>
          <div style={{ fontSize: '14px', fontWeight: 700, color: 'var(--accent-rose)', marginBottom: '6px', display: 'flex', alignItems: 'center', gap: '8px' }}>
            <ShieldAlert size={18} />
            <span>ACTIVE EMERGENCY: {disasterType || 'FLOOD'} DISASTER</span>
          </div>
          <div style={{ fontSize: '13px', color: 'var(--text-primary)' }}>
            Infrastructure damage evaluation: £{(damageCost || 50000).toLocaleString()}. Local emergency relief funding required.
          </div>
        </div>
      )}

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <TreePine size={18} color="var(--accent-emerald)" />
          <span>Regional Climate & Sustainability Evaluation</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Environmental sustainability score is rated at 82.0/100 with 35.0% green space coverage. Seasonal weather patterns impact local health, agriculture, and infrastructure.
        </p>
      </div>
    </div>
  );
};
