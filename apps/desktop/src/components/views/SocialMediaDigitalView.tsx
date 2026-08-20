import React from 'react';
import { Smartphone, Users, ThumbsUp, DollarSign, ShieldAlert, Share2 } from 'lucide-react';

interface SocialMediaDigitalViewProps {
  platform: string;
  handle: string;
  followers: number;
  influencerTier: string;
  postsCount: number;
  onCreateAccount: () => void;
  onPostContent: () => void;
  onAcceptSponsorship: () => void;
  onCyberSecurityAudit: () => void;
}

export const SocialMediaDigitalView: React.FC<SocialMediaDigitalViewProps> = ({
  platform,
  handle,
  followers,
  influencerTier,
  postsCount,
  onCreateAccount,
  onPostContent,
  onAcceptSponsorship,
  onCyberSecurityAudit,
}) => {
  return (
    <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px', overflowY: 'auto' }}>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
          <Smartphone size={22} color="var(--accent-cyan)" />
          <h2 style={{ fontSize: '20px', fontWeight: 700, fontFamily: 'var(--font-serif)' }}>
            Internet, Social Media & Digital Life
          </h2>
        </div>

        <div style={{ display: 'flex', gap: '10px' }}>
          <button
            onClick={onCreateAccount}
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
            <Users size={16} />
            <span>Create Profile Account</span>
          </button>

          <button
            onClick={onPostContent}
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
            <Share2 size={16} />
            <span>Publish Digital Content</span>
          </button>

          <button
            onClick={onAcceptSponsorship}
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
            <DollarSign size={16} />
            <span>Brand Sponsor Deal</span>
          </button>

          <button
            onClick={onCyberSecurityAudit}
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
            <span>Cyber Security Audit</span>
          </button>
        </div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr 1fr 1fr', gap: '16px' }}>
        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>DIGITAL HANDLE / PLATFORM</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-cyan)' }}>
            @{handle || 'alexmorgan'} ({platform || 'YOUTUBE'})
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>TOTAL FOLLOWERS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-emerald)', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <Users size={18} />
            <span>{followers.toLocaleString()}</span>
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>INFLUENCER TIER</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-amber)' }}>
            {influencerTier} Influencer
          </div>
        </div>

        <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '16px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)' }}>
          <div style={{ fontSize: '12px', color: 'var(--text-muted)', marginBottom: '4px' }}>CONTENT POSTS</div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--accent-indigo)' }}>
            {postsCount} Published
          </div>
        </div>
      </div>

      <div style={{ backgroundColor: 'var(--bg-surface-1)', padding: '24px', borderRadius: 'var(--radius-md)', border: '1px solid var(--border-subtle)', display: 'flex', flexDirection: 'column', gap: '12px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text-primary)', display: 'flex', alignItems: 'center', gap: '8px' }}>
          <ThumbsUp size={18} color="var(--accent-cyan)" />
          <span>Social Media Engagement & Content Monetization</span>
        </div>
        <p style={{ fontSize: '13px', color: 'var(--text-secondary)', lineHeight: 1.5 }}>
          Your digital profile generates online followers, viral post impressions, brand sponsorship deals, and digital content monetization while navigating cybersecurity hazards.
        </p>
      </div>
    </div>
  );
};
