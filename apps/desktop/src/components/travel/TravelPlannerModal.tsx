import React, { useMemo, useState } from 'react';
import { ArrowLeft, MapPin, Plane, TicketCheck, X } from 'lucide-react';
import { StructuredGameplayAction } from '../../types/gameplay';

interface TravelPlannerModalProps {
  currentLocation: string;
  currencySymbol: string;
  isLoading: boolean;
  onClose: () => void;
  onStructuredAction: (action: StructuredGameplayAction) => Promise<boolean>;
}

const destinations = [
  { id: 'city:real:lagos', name: 'Lagos, Nigeria' },
  { id: 'city:real:abuja', name: 'Abuja, Nigeria' },
  { id: 'city:real:kano', name: 'Kano, Nigeria' },
  { id: 'city:real:ibadan', name: 'Ibadan, Nigeria' },
  { id: 'city:real:port_harcourt', name: 'Port Harcourt, Nigeria' },
  { id: 'city:real:london', name: 'London, United Kingdom' },
  { id: 'city:real:glasgow', name: 'Glasgow, United Kingdom' },
  { id: 'city:real:edinburgh', name: 'Edinburgh, United Kingdom' },
  { id: 'city:real:new_york', name: 'New York City, United States' },
  { id: 'city:real:san_francisco', name: 'San Francisco, United States' },
  { id: 'city:real:houston', name: 'Houston, United States' },
];

const fares: Record<string, number> = { 'Intercity Bus': 80, Train: 90, 'Private Car': 120, Flight: 180 };

export const TravelPlannerModal: React.FC<TravelPlannerModalProps> = ({
  currentLocation,
  currencySymbol,
  isLoading,
  onClose,
  onStructuredAction,
}) => {
  const [destinationCityId, setDestinationCityId] = useState('city:real:abuja');
  const [transportMode, setTransportMode] = useState('Intercity Bus');
  const [stayDays, setStayDays] = useState(7);
  const destination = useMemo(
    () => destinations.find((item) => item.id === destinationCityId),
    [destinationCityId]
  );

  const handleBook = async (event: React.FormEvent) => {
    event.preventDefault();
    const success = await onStructuredAction({
      type: 'TRAVEL',
      destinationCityId,
      transportMode,
      stayDays,
    });
    if (success) onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4 text-slate-100">
      <form onSubmit={handleBook} className="w-full max-w-lg rounded-3xl border border-[#27304a] bg-[#0a0d14] p-6 shadow-2xl space-y-5">
        <div className="flex items-center justify-between border-b border-[#1c2234] pb-3">
          <div className="flex items-center gap-3">
            <Plane className="h-5 w-5 text-cyan-400" />
            <div>
              <h3 className="font-serif font-bold">Travel Desk</h3>
              <p className="text-[11px] text-slate-400">Book transport and reserve accommodation</p>
            </div>
          </div>
          <button type="button" onClick={onClose} aria-label="Close travel planner" className="p-2 text-slate-400 hover:text-white">
            <X className="h-4 w-4" />
          </button>
        </div>

        <div className="rounded-2xl border border-[#20283c] bg-[#121622] p-4 text-xs">
          <p className="text-slate-500">Departing from</p>
          <p className="mt-1 flex items-center gap-2 font-serif text-slate-100"><MapPin className="h-4 w-4 text-amber-400" />{currentLocation}</p>
        </div>

        <label className="block text-xs text-slate-300">
          Destination
          <select value={destinationCityId} onChange={(event) => setDestinationCityId(event.target.value)} className="mt-1.5 w-full rounded-xl border border-[#27304a] bg-[#121622] px-3 py-2.5 text-slate-100">
            {destinations.map((item) => <option key={item.id} value={item.id}>{item.name}</option>)}
          </select>
        </label>

        <div className="grid grid-cols-2 gap-3">
          <label className="text-xs text-slate-300">
            Transport
            <select value={transportMode} onChange={(event) => setTransportMode(event.target.value)} className="mt-1.5 w-full rounded-xl border border-[#27304a] bg-[#121622] px-3 py-2.5 text-slate-100">
              {Object.keys(fares).map((mode) => <option key={mode}>{mode}</option>)}
            </select>
          </label>
          <label className="text-xs text-slate-300">
            Accommodation
            <select value={stayDays} onChange={(event) => setStayDays(Number(event.target.value))} className="mt-1.5 w-full rounded-xl border border-[#27304a] bg-[#121622] px-3 py-2.5 text-slate-100">
              <option value={0}>No reservation</option>
              <option value={1}>1 night</option>
              <option value={3}>3 nights</option>
              <option value={7}>1 week</option>
              <option value={14}>2 weeks</option>
            </select>
          </label>
        </div>

        <div className="flex items-center justify-between rounded-2xl border border-cyan-500/20 bg-cyan-500/5 p-4 text-xs">
          <div>
            <p className="font-serif text-slate-100">{destination?.name}</p>
            <p className="mt-1 text-slate-400">A saved itinerary and travel process will be created.</p>
          </div>
          <p className="font-mono text-cyan-300">{currencySymbol}{fares[transportMode].toLocaleString()}</p>
        </div>

        <div className="flex gap-3">
          <button type="button" onClick={onClose} className="flex items-center gap-2 rounded-xl border border-[#27304a] px-4 py-2.5 text-xs text-slate-300"><ArrowLeft className="h-4 w-4" />Cancel</button>
          <button type="submit" disabled={isLoading} className="flex flex-1 items-center justify-center gap-2 rounded-xl bg-cyan-500 px-4 py-2.5 font-serif text-xs font-bold text-slate-950 disabled:opacity-50"><TicketCheck className="h-4 w-4" />Pay, book, and travel</button>
        </div>
      </form>
    </div>
  );
};
