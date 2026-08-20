import React from 'react';
import { LayoutDashboard, Users, GraduationCap, Trophy, Briefcase, Landmark, Compass, Globe } from 'lucide-react';

interface LifeNavProps {
  activeTab: string;
  onSelectTab: (tab: string) => void;
  interests?: string[];
}

export const LifeNav: React.FC<LifeNavProps> = ({ activeTab, onSelectTab, interests = [] }) => {
  const baseTabs = [
    { id: 'overview', label: 'Life Overview', icon: LayoutDashboard },
    { id: 'family', label: 'Family & Contacts', icon: Users },
    { id: 'school', label: 'Education & School', icon: GraduationCap },
    { id: 'career', label: 'Career & Work', icon: Briefcase },
    { id: 'money', label: 'Finances & Assets', icon: Landmark },
    { id: 'activities', label: 'Daily Activities', icon: Compass },
    { id: 'world', label: 'World Timeline', icon: Globe },
  ];

  const domainTabs: { id: string; label: string; icon: any }[] = [];

  if (interests.includes('football')) {
    domainTabs.push({ id: 'football', label: 'Football Domain', icon: Trophy });
  }
  if (interests.includes('music')) {
    domainTabs.push({ id: 'music', label: 'Music Domain', icon: Compass });
  }
  if (interests.includes('politics')) {
    domainTabs.push({ id: 'politics', label: 'Politics Domain', icon: Landmark });
  }

  const tabs = [...baseTabs, ...domainTabs];

  return (
    <nav style={{
      backgroundColor: 'var(--bg-surface-1)',
      borderRight: '1px solid var(--border-subtle)',
      padding: '16px 12px',
      display: 'flex',
      flexDirection: 'column',
      gap: '4px',
    }}>
      <div style={{
        fontSize: '11px',
        fontWeight: 700,
        color: 'var(--text-muted)',
        letterSpacing: '0.08em',
        padding: '0 8px 8px 8px',
      }}>
        FACETS
      </div>

      {tabs.map((tab) => {
        const Icon = tab.icon;
        const isActive = activeTab === tab.id;
        return (
          <button
            key={tab.id}
            onClick={() => onSelectTab(tab.id)}
            style={{
              display: 'flex',
              alignItems: 'center',
              gap: '10px',
              padding: '8px 12px',
              borderRadius: 'var(--radius-md)',
              backgroundColor: isActive ? 'var(--bg-surface-2)' : 'transparent',
              color: isActive ? 'var(--text-primary)' : 'var(--text-secondary)',
              border: isActive ? '1px solid var(--border-strong)' : '1px solid transparent',
              cursor: 'pointer',
              fontSize: '13px',
              fontWeight: isActive ? 600 : 400,
              textAlign: 'left',
              transition: 'var(--transition-fast)',
            }}
          >
            <Icon size={16} color={isActive ? 'var(--accent-indigo)' : 'var(--text-muted)'} />
            <span>{tab.label}</span>
          </button>
        );
      })}
    </nav>
  );
};
