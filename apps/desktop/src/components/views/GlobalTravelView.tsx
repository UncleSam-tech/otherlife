import React from 'react';
import { Plane, Compass, Globe, MapPin, FileCheck } from 'lucide-react';

interface GlobalTravelViewProps {
  currentLocation: string;
  passportCount: number;
  visaCount: number;
  travelCount: number;
  onBookFlight: () => void;
  onApplyPassport: () => void;
}

export const GlobalTravelView: React.FC<GlobalTravelViewProps> = ({
  currentLocation,
  passportCount,
  visaCount,
  travelCount,
  onBookFlight,
  onApplyPassport,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Globe size={22} color="var(--accent-indigo)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Global Travel, Passports & Immigration
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onBookFlight}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '8px',
              backgroundColor: 'var(--accent-indigo)',
              color: '#FFF',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              padding: '10px 16px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Plane size={16} />
            <span>Book International Flight</span>
          </button>

          <button
            onClick={onApplyPassport}
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
            <FileCheck size={16} />
            <span>Apply National Passport</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CURRENT LOCATION</div>
          <div style={{ fontSize: '16px', fontWeight: 700, color: 'var(--accent-emerald)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <MapPin size={16} />
            <span>{currentLocation}</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>PASSPORTS HELD</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-indigo)' }}>
            {passportCount} Active
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>VISAS APPROVED</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)' }}>
            {visaCount} Visas
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>INTERNATIONAL TRIPS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-cyan)' }}>
            {travelCount} Trips
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <Compass size={18} color="var(--accent-indigo)" />
          <span>Flight Logs & Cross-Border Residency</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Your international travel passport and visa status allow cross-border movement across all supported global hubs. Click "Book International Flight" to explore new cities.
        </p>
      </div>
    </div>
  );
};
