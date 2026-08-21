import React, { useState } from 'react';
import { FileText, ShieldCheck, Stamp } from 'lucide-react';

export interface DocumentDTO {
  id: string;
  title: string;
  document_type: string;
  issue_date: string;
  issuing_authority: string;
  registration_number: string;
  fields: Record<string, string>;
  is_verified: boolean;
}

interface DocumentViewerModalProps {
  documents: DocumentDTO[];
  onClose: () => void;
}

export const DocumentViewerModal: React.FC<DocumentViewerModalProps> = ({
  documents,
  onClose,
}) => {
  const [selectedDocId, setSelectedDocId] = useState<string>(
    documents[0]?.id || ''
  );

  const activeDoc = documents.find((d) => d.id === selectedDocId) || documents[0];

  return (
    <div className="fixed inset-0 bg-black/85 backdrop-blur-md z-50 flex items-center justify-center p-4 font-sans select-none text-slate-100">
      <div className="bg-[#0b0e17] border border-amber-500/30 rounded-3xl max-w-3xl w-full p-6 shadow-2xl flex flex-col md:flex-row gap-6 max-h-[90vh] animate-fadeIn">
        {/* Document Sidebar List */}
        <div className="w-full md:w-64 border-b md:border-b-0 md:border-r border-[#1c2234] pb-4 md:pb-0 md:pr-4 space-y-3 flex flex-col">
          <div className="flex items-center gap-2 text-amber-400">
            <FileText className="w-4 h-4" />
            <h3 className="font-serif font-bold text-sm tracking-wide uppercase">
              Official Records ({documents.length})
            </h3>
          </div>

          <div className="space-y-1.5 flex-1 overflow-y-auto pr-1">
            {documents.map((doc) => {
              const isSelected = doc.id === (activeDoc?.id || '');
              return (
                <button
                  key={doc.id}
                  type="button"
                  onClick={() => setSelectedDocId(doc.id)}
                  className={`w-full text-left p-3 rounded-2xl border transition-all text-xs font-serif ${
                    isSelected
                      ? 'bg-amber-500/15 border-amber-500/40 text-amber-200 shadow-sm'
                      : 'bg-[#121622] border-[#20273a] text-slate-300 hover:border-amber-500/30'
                  }`}
                >
                  <p className="font-bold truncate">{doc.title}</p>
                  <p className="text-[10px] text-slate-400 font-mono mt-0.5">{doc.registration_number}</p>
                </button>
              );
            })}
          </div>
        </div>

        {/* Active Document Canvas (Diegetic Certificate) */}
        {activeDoc ? (
          <div className="flex-1 flex flex-col justify-between overflow-y-auto">
            <div className="bg-[#fcfaf4] text-slate-900 rounded-2xl p-6 sm:p-8 space-y-6 shadow-2xl border-4 border-[#d4af37]/40 relative overflow-hidden font-serif">
              {/* Background Watermark */}
              <div className="absolute inset-0 flex items-center justify-center opacity-5 pointer-events-none">
                <Stamp className="w-72 h-72 text-[#d4af37]" />
              </div>

              {/* Certificate Header */}
              <div className="text-center space-y-1 border-b-2 border-slate-900/15 pb-4 relative z-10">
                <p className="text-[10px] font-mono uppercase tracking-widest text-slate-600">
                  {activeDoc.issuing_authority}
                </p>
                <h2 className="text-xl sm:text-2xl font-black tracking-tight text-slate-950 uppercase">
                  {activeDoc.title}
                </h2>
                <div className="flex items-center justify-center gap-1.5 text-xs text-emerald-800 font-semibold pt-1">
                  <ShieldCheck className="w-4 h-4 text-emerald-700" />
                  <span>Verified Public Registry Entry</span>
                </div>
              </div>

              {/* Certificate Fields Grid */}
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs relative z-10">
                {Object.entries(activeDoc.fields).map(([label, value]) => (
                  <div key={label} className="border-b border-slate-300/60 pb-1.5">
                    <span className="text-[10px] font-mono uppercase tracking-wider text-slate-500 block">
                      {label}
                    </span>
                    <span className="text-sm font-bold text-slate-900">{value}</span>
                  </div>
                ))}
              </div>

              {/* Certificate Footer / Seal */}
              <div className="pt-4 border-t-2 border-slate-900/15 flex items-center justify-between text-[11px] font-mono text-slate-600 relative z-10">
                <div>
                  <p>Issue Date: {activeDoc.issue_date}</p>
                  <p>Reg No: {activeDoc.registration_number}</p>
                </div>
                <div className="text-right">
                  <div className="w-16 h-16 rounded-full border-2 border-dashed border-[#b8860b] flex flex-col items-center justify-center p-1 text-[9px] font-black uppercase text-[#8b6508] bg-[#fffaf0] rotate-[-8deg] shadow-sm">
                    <span>OFFICIAL</span>
                    <span>SEAL</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Bottom Actions */}
            <div className="pt-4 flex justify-end">
              <button
                type="button"
                onClick={onClose}
                className="bg-[#121622] hover:bg-[#1a2030] text-slate-300 hover:text-slate-100 px-5 py-2 rounded-xl text-xs font-serif border border-[#20273a] transition-colors"
              >
                Close Document Viewer
              </button>
            </div>
          </div>
        ) : (
          <div className="flex-1 flex items-center justify-center text-slate-400 font-serif italic text-sm">
            No document selected.
          </div>
        )}
      </div>
    </div>
  );
};
