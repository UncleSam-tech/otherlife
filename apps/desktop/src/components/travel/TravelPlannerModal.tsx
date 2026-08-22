import React, { useState } from 'react';
import { ArrowLeft, ArrowRight, BedDouble, BusFront, CalendarDays, CheckCircle2, MapPin, Plane, Search, TicketCheck, UserRound, X } from 'lucide-react';
import { StructuredGameplayAction } from '../../types/gameplay';

interface TravelPlannerModalProps {
  currentLocation: string;
  playerName: string;
  currencySymbol: string;
  isLoading: boolean;
  onClose: () => void;
  onStructuredAction: (action: StructuredGameplayAction) => Promise<boolean>;
}

interface RouteOption {
  id: string;
  operator: string;
  departure: string;
  arrival: string;
  service: string;
  price: number;
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

const baseFares: Record<string, number> = { 'Intercity Bus': 80, Train: 90, 'Private Car': 120, Flight: 180 };
const departureChoices = ['Today', 'Tomorrow morning', 'In three days'];
const accommodationChoices = ['No accommodation reservation', 'Central City Lodge', 'Business District Suites', 'Serviced Apartment'];
const stayChoices = [
  { days: 1, label: '1 night' },
  { days: 3, label: '3 nights' },
  { days: 7, label: '1 week' },
  { days: 14, label: '2 weeks' },
];

const routeOptionsFor = (transportMode: string): RouteOption[] => {
  const base = baseFares[transportMode];
  const operators = transportMode === 'Flight'
    ? ['Unity Air', 'Coastal Wings']
    : transportMode === 'Train'
      ? ['National Rail Express', 'CityLink Rail']
      : transportMode === 'Private Car'
        ? ['Self-drive itinerary', 'Licensed car service']
        : ['Peace Mass Transit', 'ABC Intercity'];
  return [
    { id: 'standard', operator: operators[0], departure: '08:30', arrival: transportMode === 'Flight' ? '11:30' : '18:30', service: 'Standard flexible', price: base },
    { id: 'comfort', operator: operators[1], departure: '12:15', arrival: transportMode === 'Flight' ? '15:15' : '22:15', service: 'Comfort priority', price: Math.round(base * 1.2) },
  ];
};

export const TravelPlannerModal: React.FC<TravelPlannerModalProps> = ({
  currentLocation,
  playerName,
  currencySymbol,
  isLoading,
  onClose,
  onStructuredAction,
}) => {
  const [step, setStep] = useState(0);
  const [destinationCityId, setDestinationCityId] = useState('city:real:abuja');
  const [transportMode, setTransportMode] = useState('Intercity Bus');
  const [departureTiming, setDepartureTiming] = useState('Today');
  const [selectedRoute, setSelectedRoute] = useState<RouteOption | null>(null);
  const [stayDays, setStayDays] = useState(7);
  const [accommodation, setAccommodation] = useState('Central City Lodge');
  const destination = destinations.find((item) => item.id === destinationCityId) ?? destinations[1];
  const sameCity = currentLocation.toLowerCase().startsWith(destination.name.split(',')[0].toLowerCase());

  const confirmJourney = async () => {
    if (!selectedRoute) return;
    const success = await onStructuredAction({
      type: 'TRAVEL',
      destinationCityId,
      transportMode,
      stayDays: accommodation === 'No accommodation reservation' ? 0 : stayDays,
      operatorName: selectedRoute.operator,
      serviceClass: selectedRoute.service,
      fare: selectedRoute.price,
      accommodation,
      departureTiming: `${departureTiming} at ${selectedRoute.departure}`,
    });
    if (success) onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 p-4 text-slate-100 backdrop-blur-sm">
      <section className="flex h-[650px] w-full max-w-2xl flex-col space-y-5 rounded-3xl border border-[#27304a] bg-[#0a0d14] p-6 shadow-2xl" aria-label="Travel booking workflow">
        <header className="flex items-center justify-between border-b border-[#1c2234] pb-3">
          <div className="flex items-center gap-3"><Plane className="h-5 w-5 text-cyan-400" /><div><h3 className="font-serif font-bold">Travel Desk</h3><p className="text-[11px] text-slate-400">Search, compare, reserve, review, and depart</p></div></div>
          <button type="button" onClick={onClose} aria-label="Close travel planner" className="p-2 text-slate-400 hover:text-white"><X className="h-4 w-4" /></button>
        </header>

        <ol className="grid grid-cols-4 gap-2" aria-label="Booking progress">
          {['Search', 'Route', 'Stay', 'Review'].map((label, index) => <li key={label} className={`rounded-lg border px-2 py-1.5 text-center text-[9px] font-mono uppercase ${index <= step ? 'border-cyan-400/40 bg-cyan-400/10 text-cyan-200' : 'border-[#263049] text-slate-600'}`}>{index + 1}. {label}</li>)}
        </ol>

        <div className="flex-1 overflow-y-auto">
          {step === 0 ? (
            <div className="space-y-5">
              <div className="rounded-2xl border border-[#20283c] bg-[#121622] p-4 text-xs"><p className="text-slate-500">Origin</p><p className="mt-1 flex items-center gap-2 font-serif text-slate-100"><MapPin className="h-4 w-4 text-amber-400" />{currentLocation}</p></div>
              <fieldset className="space-y-2"><legend className="text-xs text-slate-300">Destination</legend><div className="grid grid-cols-2 gap-2 sm:grid-cols-3">{destinations.map((item) => <button key={item.id} type="button" onClick={() => { setDestinationCityId(item.id); setSelectedRoute(null); }} aria-pressed={destinationCityId === item.id} className={`rounded-xl border px-3 py-2.5 text-left text-[11px] transition ${destinationCityId === item.id ? 'border-cyan-400 bg-cyan-400/10 text-cyan-100' : 'border-[#27304a] bg-[#121622] text-slate-300 hover:border-cyan-400/45'}`}>{item.name}</button>)}</div></fieldset>
              <div className="grid gap-4 sm:grid-cols-2"><fieldset className="space-y-2"><legend className="text-xs text-slate-300">Transport</legend><div className="grid grid-cols-2 gap-2">{Object.keys(baseFares).map((mode) => <button key={mode} type="button" onClick={() => { setTransportMode(mode); setSelectedRoute(null); }} aria-pressed={transportMode === mode} className={`rounded-xl border px-3 py-2.5 text-[11px] transition ${transportMode === mode ? 'border-cyan-400 bg-cyan-400/10 text-cyan-100' : 'border-[#27304a] bg-[#121622] text-slate-300 hover:border-cyan-400/45'}`}>{mode}</button>)}</div></fieldset><fieldset className="space-y-2"><legend className="text-xs text-slate-300">Departure</legend><div className="space-y-2">{departureChoices.map((choice) => <button key={choice} type="button" onClick={() => setDepartureTiming(choice)} aria-pressed={departureTiming === choice} className={`w-full rounded-xl border px-3 py-2 text-left text-[11px] transition ${departureTiming === choice ? 'border-cyan-400 bg-cyan-400/10 text-cyan-100' : 'border-[#27304a] bg-[#121622] text-slate-300 hover:border-cyan-400/45'}`}>{choice}</button>)}</div></fieldset></div>
              {sameCity ? <p className="rounded-xl border border-red-400/25 bg-red-400/5 p-3 text-xs text-red-200">Choose a destination outside your current city.</p> : null}
              <button type="button" onClick={() => setStep(1)} disabled={sameCity} className="flex w-full items-center justify-center gap-2 rounded-xl bg-cyan-500 py-3 text-xs font-bold text-slate-950 disabled:opacity-40"><Search className="h-4 w-4" />Search available routes</button>
            </div>
          ) : null}

          {step === 1 ? (
            <div className="space-y-4">
              <div><p className="text-[10px] font-mono uppercase text-cyan-300">Available services</p><h4 className="mt-1 font-serif text-lg font-bold">{currentLocation.split(',')[0]} → {destination.name.split(',')[0]}</h4><p className="text-xs text-slate-500">{departureTiming} · {transportMode}</p></div>
              {routeOptionsFor(transportMode).map((route) => (
                <button key={route.id} type="button" onClick={() => setSelectedRoute(route)} className={`w-full rounded-2xl border p-4 text-left transition ${selectedRoute?.id === route.id ? 'border-cyan-400 bg-cyan-400/10' : 'border-[#27304a] bg-[#111622] hover:border-cyan-400/50'}`}>
                  <div className="flex items-start justify-between"><div><p className="font-serif text-sm font-bold">{route.operator}</p><p className="mt-1 text-[11px] text-slate-400">{route.service}</p></div><p className="font-mono text-sm text-cyan-300">{currencySymbol}{route.price.toLocaleString()}</p></div>
                  <div className="mt-4 flex items-center gap-3 text-xs text-slate-300"><span>{route.departure}</span><div className="h-px flex-1 bg-[#33405a]" /><BusFront className="h-4 w-4 text-slate-500" /><div className="h-px flex-1 bg-[#33405a]" /><span>{route.arrival}</span></div>
                </button>
              ))}
              <div className="flex gap-3"><button type="button" onClick={() => setStep(0)} className="rounded-xl border border-[#27304a] px-4 py-3 text-xs text-slate-300">Back</button><button type="button" onClick={() => setStep(2)} disabled={!selectedRoute} className="flex flex-1 items-center justify-center gap-2 rounded-xl bg-cyan-500 py-3 text-xs font-bold text-slate-950 disabled:opacity-40">Continue with this route <ArrowRight className="h-4 w-4" /></button></div>
            </div>
          ) : null}

          {step === 2 ? (
            <div className="space-y-5">
              <div className="grid grid-cols-2 gap-3"><div className="rounded-2xl border border-[#27304a] bg-[#111622] p-4"><UserRound className="h-5 w-5 text-cyan-400" /><p className="mt-2 text-[10px] text-slate-500">Passenger</p><p className="mt-1 font-serif text-sm">{playerName}</p></div><div className="rounded-2xl border border-[#27304a] bg-[#111622] p-4"><CalendarDays className="h-5 w-5 text-cyan-400" /><p className="mt-2 text-[10px] text-slate-500">Departure</p><p className="mt-1 font-serif text-sm">{departureTiming} · {selectedRoute?.departure}</p></div></div>
              <fieldset className="space-y-2"><legend className="text-xs text-slate-300">Accommodation</legend><div className="grid grid-cols-2 gap-2">{accommodationChoices.map((choice) => <button key={choice} type="button" onClick={() => setAccommodation(choice)} aria-pressed={accommodation === choice} className={`rounded-xl border px-3 py-3 text-left text-[11px] transition ${accommodation === choice ? 'border-cyan-400 bg-cyan-400/10 text-cyan-100' : 'border-[#27304a] bg-[#121622] text-slate-300 hover:border-cyan-400/45'}`}>{choice}</button>)}</div></fieldset>
              <fieldset className="space-y-2" disabled={accommodation === 'No accommodation reservation'}><legend className="text-xs text-slate-300">Length of stay</legend><div className="grid grid-cols-4 gap-2">{stayChoices.map((choice) => <button key={choice.days} type="button" onClick={() => setStayDays(choice.days)} aria-pressed={stayDays === choice.days} className={`rounded-xl border px-2 py-2.5 text-[11px] transition disabled:opacity-35 ${stayDays === choice.days ? 'border-cyan-400 bg-cyan-400/10 text-cyan-100' : 'border-[#27304a] bg-[#121622] text-slate-300 hover:border-cyan-400/45'}`}>{choice.label}</button>)}</div></fieldset>
              <div className="flex gap-3"><button type="button" onClick={() => setStep(1)} className="rounded-xl border border-[#27304a] px-4 py-3 text-xs text-slate-300">Back</button><button type="button" onClick={() => setStep(3)} className="flex-1 rounded-xl bg-cyan-500 py-3 text-xs font-bold text-slate-950">Review booking</button></div>
            </div>
          ) : null}

          {step === 3 ? (
            <div className="space-y-5">
              <div className="flex items-center gap-2 text-cyan-300"><CheckCircle2 className="h-5 w-5" /><h4 className="font-serif text-base font-bold">Review before purchase</h4></div>
              <dl className="grid grid-cols-2 gap-3 text-xs"><div><dt className="text-slate-500">Passenger</dt><dd className="mt-1 text-slate-200">{playerName}</dd></div><div><dt className="text-slate-500">Route</dt><dd className="mt-1 text-slate-200">{currentLocation.split(',')[0]} → {destination.name.split(',')[0]}</dd></div><div><dt className="text-slate-500">Operator</dt><dd className="mt-1 text-slate-200">{selectedRoute?.operator}</dd></div><div><dt className="text-slate-500">Service</dt><dd className="mt-1 text-slate-200">{selectedRoute?.service}</dd></div><div><dt className="text-slate-500">Departure</dt><dd className="mt-1 text-slate-200">{departureTiming} at {selectedRoute?.departure}</dd></div><div><dt className="text-slate-500">Accommodation</dt><dd className="mt-1 text-slate-200">{accommodation === 'No accommodation reservation' ? accommodation : `${accommodation} · ${stayDays} night(s)`}</dd></div></dl>
              <div className="flex items-center justify-between rounded-2xl border border-cyan-400/25 bg-cyan-400/5 p-4"><div><p className="text-[10px] text-slate-500">Transport total</p><p className="mt-1 text-xs text-slate-300">Payment will be deducted from your in-game account.</p></div><p className="font-mono text-lg text-cyan-300">{currencySymbol}{selectedRoute?.price.toLocaleString()}</p></div>
              <p className="rounded-xl border border-amber-400/20 bg-amber-400/5 p-3 text-[11px] text-amber-100">Confirming creates a ticket and itinerary, deducts the fare, advances the journey duration, and moves your life to {destination.name}.</p>
              <div className="flex gap-3"><button type="button" onClick={() => setStep(2)} className="flex items-center gap-2 rounded-xl border border-[#27304a] px-4 py-3 text-xs text-slate-300"><ArrowLeft className="h-4 w-4" />Edit</button><button type="button" onClick={confirmJourney} disabled={isLoading} className="flex flex-1 items-center justify-center gap-2 rounded-xl bg-cyan-500 py-3 text-xs font-bold text-slate-950 disabled:opacity-40"><TicketCheck className="h-4 w-4" />Confirm purchase and depart</button></div>
            </div>
          ) : null}
        </div>

        <footer className="flex items-center justify-between border-t border-[#1c2234] pt-3 text-[10px] text-slate-600"><span className="flex items-center gap-1.5"><BedDouble className="h-3.5 w-3.5" />Accommodation is optional</span><span>Booking data saves locally</span></footer>
      </section>
    </div>
  );
};
