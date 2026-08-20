import React from 'react';

interface SkillItem {
  id: string;
  name: string;
  category: string;
  description: string;
}

interface SkillBudgetEditorProps {
  skillsList: SkillItem[];
  selectedSkills: Record<string, number>;
  onChangeSkill: (skillId: string, value: number) => void;
  budgetMode: 'REALISTIC' | 'SANDBOX' | 'RANDOM';
}

export const SkillBudgetEditor: React.FC<SkillBudgetEditorProps> = ({
  skillsList,
  selectedSkills,
  onChangeSkill,
  budgetMode,
}) => {
  const categories = Array.from(new Set(skillsList.map((s) => s.category)));

  const totalPointsUsed = Object.values(selectedSkills).reduce((acc, curr) => acc + curr, 0);
  const budgetLimit = 200;

  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', backgroundColor: 'var(--bg-surface-2)', padding: '12px 16px', borderRadius: 'var(--radius-md)' }}>
        <span style={{ fontSize: '13px', fontWeight: 600, color: 'var(--text-primary)' }}>
          Creation Mode: <strong style={{ color: 'var(--accent-indigo)' }}>{budgetMode}</strong>
        </span>
        {budgetMode === 'REALISTIC' && (
          <span style={{ fontSize: '13px', fontFamily: 'var(--font-mono)', color: totalPointsUsed > budgetLimit ? 'var(--accent-crimson)' : 'var(--accent-emerald)' }}>
            Points Used: {totalPointsUsed} / {budgetLimit}
          </span>
        )}
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '20px', maxHeight: '360px', overflowY: 'auto', paddingRight: '4px' }}>
        {categories.map((cat) => (
          <div key={cat} style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
            <h4 style={{ fontSize: '12px', fontWeight: 700, color: 'var(--text-muted)', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
              {cat}
            </h4>

            {skillsList
              .filter((s) => s.category === cat)
              .map((skill) => {
                const val = selectedSkills[skill.id] || 0;
                return (
                  <div key={skill.id} style={{ display: 'grid', gridTemplateColumns: '160px 1fr 50px', alignItems: 'center', gap: '12px', padding: '6px 10px', backgroundColor: 'var(--bg-surface-1)', borderRadius: 'var(--radius-sm)' }}>
                    <div>
                      <div style={{ fontSize: '13px', fontWeight: 500, color: 'var(--text-primary)' }}>{skill.name}</div>
                      <div style={{ fontSize: '11px', color: 'var(--text-muted)' }}>{skill.description}</div>
                    </div>

                    <input
                      type="range"
                      min="0"
                      max="100"
                      value={val}
                      onChange={(e) => onChangeSkill(skill.id, parseInt(e.target.value, 10))}
                      style={{ accentColor: 'var(--accent-indigo)', cursor: 'pointer' }}
                    />

                    <span style={{ fontSize: '12px', fontFamily: 'var(--font-mono)', textAlign: 'right', fontWeight: 600, color: val > 50 ? 'var(--accent-indigo)' : 'var(--text-secondary)' }}>
                      {val}
                    </span>
                  </div>
                );
              })}
          </div>
        ))}
      </div>
    </div>
  );
};
