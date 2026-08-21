import React, { useState } from 'react';
import { Play, Sparkles, FolderOpen, Settings, Trash2, ArrowRight } from 'lucide-react';

export interface SaveMetadata {
  id: string;
  filename: string;
  player_name: string;
  age: number;
  location: string;
  timestamp: string;
}

interface MainMenuProps {
  saves: SaveMetadata[];
  onStartNewLife: () => void;
  onContinueRecentSave: () => void;
  onLoadSave: (filename: string) => void;
  onDeleteSave: (filename: string) => void;
  onOpenSettings: () => void;
}

export const MainMenu: React.FC<MainMenuProps> = ({
  saves,
  onStartNewLife,
  onContinueRecentSave,
  onLoadSave,
  onDeleteSave,
  onOpenSettings,
}) => {
  const [showLoadModal, setShowLoadModal] = useState(false);
  const hasSaves = saves && saves.length > 0;
  const recentSave = hasSaves ? saves[0] : null;

  return (
    <div style={{
      width: '100vw',
      height: '100vh',
      backgroundColor: '#0A0C10',
      color: '#E2E8F0',
      display: 'flex',
      flexDirection: 'column',
      alignItems: 'center',
      justifyContent: 'center',
      position: 'relative',
      overflow: 'hidden',
      fontFamily: 'var(--font-sans, system-ui, -apple-system, sans-serif)',
    }}>
      {/* Ambient background glow */}
      <div style={{
        position: 'absolute',
        top: '20%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        width: '600px',
        height: '400px',
        background: 'radial-gradient(circle, rgba(99, 102, 241, 0.15) 0%, rgba(0, 0, 0, 0) 70%)',
        pointerEvents: 'none',
      }} />

      <div style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        textAlign: 'center',
        zIndex: 1,
        maxWidth: '540px',
        width: '90%',
      }}>
        <div style={{
          fontSize: '11px',
          fontFamily: 'var(--font-mono, monospace)',
          letterSpacing: '0.3em',
          color: 'var(--accent-indigo, #818CF8)',
          textTransform: 'uppercase',
          marginBottom: '12px',
        }}>
          Alternate Life Simulation
        </div>

        <h1 style={{
          fontSize: '52px',
          fontWeight: 900,
          letterSpacing: '0.15em',
          fontFamily: 'var(--font-mono, monospace)',
          color: '#F8FAFC',
          margin: 0,
          lineHeight: 1,
        }}>
          OTHERLIFE
        </h1>

        <p style={{
          fontSize: '18px',
          color: '#94A3B8',
          fontFamily: 'var(--font-serif, Georgia, serif)',
          fontStyle: 'italic',
          marginTop: '12px',
          marginBottom: '40px',
        }}>
          Live another life.
        </p>

        <div style={{
          display: 'flex',
          flexDirection: 'column',
          gap: '14px',
          width: '100%',
          maxWidth: '360px',
        }}>
          <button
            onClick={onStartNewLife}
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '10px',
              backgroundColor: '#059669',
              color: '#FFFFFF',
              border: 'none',
              borderRadius: '8px',
              padding: '14px 24px',
              fontSize: '15px',
              fontWeight: 700,
              cursor: 'pointer',
              boxShadow: '0 4px 14px rgba(5, 150, 105, 0.4)',
              transition: 'all 0.15s ease',
            }}
          >
            <Sparkles size={18} />
            <span>New Life</span>
          </button>

          <button
            onClick={onContinueRecentSave}
            disabled={!hasSaves}
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '10px',
              backgroundColor: hasSaves ? '#1E293B' : '#0F172A',
              color: hasSaves ? '#F8FAFC' : '#475569',
              border: '1px solid',
              borderColor: hasSaves ? '#334155' : '#1E293B',
              borderRadius: '8px',
              padding: '12px 24px',
              fontSize: '14px',
              fontWeight: 600,
              cursor: hasSaves ? 'pointer' : 'not-allowed',
              opacity: hasSaves ? 1 : 0.6,
            }}
          >
            <Play size={16} />
            <span>Continue {recentSave ? `(${recentSave.player_name})` : ''}</span>
          </button>

          <button
            onClick={() => setShowLoadModal(true)}
            disabled={!hasSaves}
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '10px',
              backgroundColor: hasSaves ? '#1E293B' : '#0F172A',
              color: hasSaves ? '#F8FAFC' : '#475569',
              border: '1px solid',
              borderColor: hasSaves ? '#334155' : '#1E293B',
              borderRadius: '8px',
              padding: '12px 24px',
              fontSize: '14px',
              fontWeight: 600,
              cursor: hasSaves ? 'pointer' : 'not-allowed',
              opacity: hasSaves ? 1 : 0.6,
            }}
          >
            <FolderOpen size={16} />
            <span>Load Timeline ({saves.length})</span>
          </button>

          <button
            onClick={onOpenSettings}
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              gap: '10px',
              backgroundColor: 'transparent',
              color: '#94A3B8',
              border: '1px solid #1E293B',
              borderRadius: '8px',
              padding: '10px 24px',
              fontSize: '13px',
              fontWeight: 600,
              cursor: 'pointer',
            }}
          >
            <Settings size={15} />
            <span>Settings</span>
          </button>
        </div>

        {!hasSaves && (
          <div style={{
            marginTop: '32px',
            padding: '14px 20px',
            backgroundColor: '#0F172A',
            borderRadius: '8px',
            border: '1px solid #1E293B',
            fontSize: '13px',
            color: '#64748B',
            lineHeight: 1.5,
          }}>
            <p style={{ margin: 0 }}>
              No life has begun yet. Create someone from birth, begin later in life, or let the world generate everything for you.
            </p>
          </div>
        )}
      </div>

      {/* Save Selection Modal */}
      {showLoadModal && (
        <div style={{
          position: 'fixed',
          inset: 0,
          backgroundColor: 'rgba(0, 0, 0, 0.75)',
          backdropFilter: 'blur(4px)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 100,
        }}>
          <div style={{
            backgroundColor: '#0F172A',
            border: '1px solid #334155',
            borderRadius: '12px',
            padding: '28px',
            width: '90%',
            maxWidth: '520px',
            maxHeight: '80vh',
            display: 'flex',
            flexDirection: 'column',
            gap: '16px',
            boxShadow: '0 20px 25px -5px rgba(0, 0, 0, 0.5)',
          }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <h3 style={{ fontSize: '18px', fontWeight: 700, margin: 0, color: '#F8FAFC' }}>
                Load Life Timeline
              </h3>
              <button
                onClick={() => setShowLoadModal(false)}
                style={{ backgroundColor: 'transparent', border: 'none', color: '#94A3B8', cursor: 'pointer', fontSize: '16px' }}
              >
                ✕
              </button>
            </div>

            <div style={{ display: 'flex', flexDirection: 'column', gap: '10px', overflowY: 'auto', maxHeight: '50vh' }}>
              {saves.map((s) => (
                <div
                  key={s.id}
                  style={{
                    backgroundColor: '#1E293B',
                    border: '1px solid #334155',
                    borderRadius: '8px',
                    padding: '14px 18px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                  }}
                >
                  <div style={{ textAlign: 'left' }}>
                    <div style={{ fontSize: '15px', fontWeight: 700, color: '#F8FAFC' }}>{s.player_name}</div>
                    <div style={{ fontSize: '12px', color: '#94A3B8', marginTop: '2px' }}>
                      Age {s.age} · {s.location.replace('city:real:', '').toUpperCase()}
                    </div>
                    <div style={{ fontSize: '11px', color: '#64748B', marginTop: '4px', fontFamily: 'monospace' }}>
                      {s.timestamp}
                    </div>
                  </div>

                  <div style={{ display: 'flex', gap: '8px' }}>
                    <button
                      onClick={() => {
                        onLoadSave(s.filename);
                        setShowLoadModal(false);
                      }}
                      style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px',
                        backgroundColor: '#4F46E5',
                        color: '#FFF',
                        border: 'none',
                        borderRadius: '6px',
                        padding: '8px 14px',
                        fontSize: '13px',
                        fontWeight: 600,
                        cursor: 'pointer',
                      }}
                    >
                      <span>Load</span>
                      <ArrowRight size={14} />
                    </button>
                    <button
                      onClick={() => onDeleteSave(s.filename)}
                      style={{
                        backgroundColor: '#451A1A',
                        color: '#F87171',
                        border: 'none',
                        borderRadius: '6px',
                        padding: '8px',
                        cursor: 'pointer',
                      }}
                      title="Delete Save"
                    >
                      <Trash2 size={14} />
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
