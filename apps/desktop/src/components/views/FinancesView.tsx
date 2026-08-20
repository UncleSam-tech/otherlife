import React from 'react';
import { Landmark, Home, Key } from 'lucide-react';

interface FinancesViewProps {
  cash: number;
  monthlySalary: number;
  housingType: string;
  onRentApartment: () => void;
  onBuyProperty: () => void;
}

export const FinancesView: React.FC<FinancesViewProps> = ({
  cash,
  monthlySalary,
  housingType,
  onRentApartment,
  onBuyProperty,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
        <Landmark size={22} color="var(--accent-emerald)" />
        <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
          Finances & Housing
        </h2>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CASH & LIQUID SAVINGS</div>
          <div style={{ fontSize: '20px', fontWeight: 800, color: 'var(--accent-emerald)' }}>
            £{cash.toFixed(2)}
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>MONTHLY INCOME</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)' }}>
            £{monthlySalary.toFixed(0)} / mo
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>HOUSING STATUS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--text-primary)' }}>
            {housingType}
          </div>
        </div>
      </div>

      <div style={{ display: 'flex', gap: '12px' }}>
        <button
          onClick={onRentApartment}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            backgroundColor: 'var(--bg-surface-2)',
            border: '1px solid var(--border-strong)',
            color: 'var(--text-primary)',
            borderRadius: 'var(--radius-md)',
            padding: '10px 18px',
            fontSize: '13px',
            fontWeight: 600,
            cursor: 'pointer',
          }}
        >
          <Key size={16} />
          <span>Rent Apartment (£550/mo)</span>
        </button>

        <button
          onClick={onBuyProperty}
          style={{
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            backgroundColor: cash >= 15000 ? 'var(--accent-emerald)' : 'var(--bg-surface-2)',
            color: cash >= 15000 ? '#FFF' : 'var(--text-muted)',
            border: 'none',
            borderRadius: 'var(--radius-md)',
            padding: '10px 18px',
            fontSize: '13px',
            fontWeight: 600,
            cursor: cash >= 15000 ? 'pointer' : 'not-allowed',
          }}
        >
          <Home size={16} />
          <span>Buy Residential Property (£15,000)</span>
        </button>
      </div>
    </div>
  );
};
