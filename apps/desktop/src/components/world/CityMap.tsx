import React, { useEffect, useRef, useState } from 'react';
import type { Material, Mesh, Scene, WebGLRenderer } from 'three';
import { Bike, BusFront, Clock3, MapPin, Navigation, Users, X } from 'lucide-react';
import { WorldMapPlaceDTO } from '../../types/gameplay';

interface CityMapProps {
  cityName: string;
  currencySymbol: string;
  places: WorldMapPlaceDTO[];
  isLoading: boolean;
  onCommute: (placeId: string, transportMode: string) => Promise<boolean>;
  onArrive: () => void;
}

const placeColors: Record<string, number> = {
  Residence: 0xd4a354,
  Workplace: 0x4da6ff,
  Education: 0x8b7dff,
  CommercialVenue: 0xe66eaa,
  CivicCenter: 0xb98bea,
  MedicalClinic: 0x4dd6a7,
  AthleticField: 0x68bd5b,
  TrainStation: 0xf18c55,
};

export const CityMap: React.FC<CityMapProps> = ({
  cityName,
  currencySymbol,
  places,
  isLoading,
  onCommute,
  onArrive,
}) => {
  const mountRef = useRef<HTMLDivElement>(null);
  const selectPlaceRef = useRef<(placeId: string) => void>(() => undefined);
  const [selectedId, setSelectedId] = useState(() => places.find((place) => place.is_current)?.id ?? places[0]?.id ?? '');
  const selected = places.find((place) => place.id === selectedId) ?? places[0];

  selectPlaceRef.current = setSelectedId;

  useEffect(() => {
    const mount = mountRef.current;
    if (!mount || places.length === 0) return;
    let cancelled = false;
    let frameId = 0;
    let renderer: WebGLRenderer | undefined;
    let scene: Scene | undefined;
    let resizeObserver: ResizeObserver | undefined;
    let removeListeners: (() => void) | undefined;

    const initialize = async () => {
      const THREE = await import('three');
      if (cancelled) return;
      scene = new THREE.Scene();
      scene.background = new THREE.Color(0x07101a);
      scene.fog = new THREE.Fog(0x07101a, 24, 55);
      const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 100);
      camera.position.set(0, 31, 29);
      camera.lookAt(0, 0, 0);
      renderer = new THREE.WebGLRenderer({ antialias: true, powerPreference: 'high-performance' });
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 1.5));
      renderer.shadowMap.enabled = true;
      renderer.outputColorSpace = THREE.SRGBColorSpace;
      renderer.domElement.style.width = '100%';
      renderer.domElement.style.height = '100%';
      renderer.domElement.style.display = 'block';
      renderer.domElement.style.cursor = 'pointer';
      renderer.domElement.setAttribute('aria-label', `Interactive three-dimensional map of ${cityName}`);
      mount.appendChild(renderer.domElement);

      scene.add(new THREE.HemisphereLight(0xa9d7ff, 0x172012, 2.2));
      const sun = new THREE.DirectionalLight(0xffd99a, 3.4);
      sun.position.set(-12, 24, 10);
      sun.castShadow = true;
      scene.add(sun);

      const ground = new THREE.Mesh(
        new THREE.PlaneGeometry(42, 30),
        new THREE.MeshStandardMaterial({ color: 0x12251f, roughness: 0.96 }),
      );
      ground.rotation.x = -Math.PI / 2;
      ground.receiveShadow = true;
      scene.add(ground);

      const water = new THREE.Mesh(
        new THREE.PlaneGeometry(42, 4.2),
        new THREE.MeshStandardMaterial({ color: 0x123d50, roughness: 0.28, metalness: 0.2 }),
      );
      water.rotation.x = -Math.PI / 2;
      water.position.set(0, 0.02, -12.6);
      scene.add(water);

      const roadMaterial = new THREE.MeshStandardMaterial({ color: 0x27303a, roughness: 0.9 });
      [-9, 0, 9].forEach((x) => {
        const road = new THREE.Mesh(new THREE.PlaneGeometry(2.2, 30), roadMaterial.clone());
        road.rotation.x = -Math.PI / 2;
        road.position.set(x, 0.025, 0);
        scene!.add(road);
      });
      [-6, 5].forEach((z) => {
        const road = new THREE.Mesh(new THREE.PlaneGeometry(42, 2.1), roadMaterial.clone());
        road.rotation.x = -Math.PI / 2;
        road.position.set(0, 0.03, z);
        scene!.add(road);
      });

      const markingMaterial = new THREE.MeshBasicMaterial({ color: 0xd8c978 });
      [-9, 0, 9].forEach((x) => {
        for (let z = -13; z <= 13; z += 3.2) {
          const marking = new THREE.Mesh(new THREE.PlaneGeometry(0.08, 1.4), markingMaterial.clone());
          marking.rotation.x = -Math.PI / 2;
          marking.position.set(x, 0.045, z);
          scene!.add(marking);
        }
      });
      [-6, 5].forEach((z) => {
        for (let x = -19; x <= 19; x += 3.2) {
          const marking = new THREE.Mesh(new THREE.PlaneGeometry(1.4, 0.08), markingMaterial.clone());
          marking.rotation.x = -Math.PI / 2;
          marking.position.set(x, 0.05, z);
          scene!.add(marking);
        }
      });

      const treePositions: Array<[number, number]> = [
        [-16, -8], [-13, 8], [-5, -10], [4, -9], [13, -8], [16, 3], [13, 11],
        [5, 10], [-4, 10], [-14, 1], [7, 1], [-6, 1],
      ];
      treePositions.forEach(([x, z], index) => {
        const trunk = new THREE.Mesh(
          new THREE.CylinderGeometry(0.12, 0.18, 1.35, 8),
          new THREE.MeshStandardMaterial({ color: 0x765035, roughness: 1 }),
        );
        trunk.position.set(x, 0.68, z);
        trunk.castShadow = true;
        scene!.add(trunk);
        const crown = new THREE.Mesh(
          new THREE.IcosahedronGeometry(0.8 + (index % 3) * 0.1, 1),
          new THREE.MeshStandardMaterial({ color: index % 2 ? 0x2f774f : 0x3c8b57, roughness: 0.94 }),
        );
        crown.position.set(x, 1.75, z);
        crown.castShadow = true;
        scene!.add(crown);
      });

      const interactives: Mesh[] = [];
      places.forEach((place, index) => {
        const x = (place.map_x / 100 - 0.5) * 36;
        const z = (place.map_y / 100 - 0.5) * 25;
        const height = place.category === 'Workplace' || place.category === 'CivicCenter' ? 4.4 : place.category === 'Education' ? 3.2 : 2.4;
        const footprint = place.category === 'AthleticField' ? [4.4, 0.55, 3.2] : [2.8, height, 2.5];
        const building = new THREE.Mesh(
          new THREE.BoxGeometry(footprint[0], footprint[1], footprint[2]),
          new THREE.MeshStandardMaterial({
            color: placeColors[place.category] ?? 0x8192a5,
            emissive: place.is_current ? 0x5d4310 : 0x000000,
            emissiveIntensity: place.is_current ? 1.2 : 0,
            roughness: 0.72,
          }),
        );
        building.position.set(x, footprint[1] / 2, z);
        building.castShadow = true;
        building.receiveShadow = true;
        building.userData.placeId = place.id;
        building.userData.baseY = building.position.y;
        building.userData.phase = index * 0.7;
        interactives.push(building);
        scene!.add(building);

        if (place.category !== 'AthleticField') {
          const roof = new THREE.Mesh(
            new THREE.BoxGeometry(footprint[0] + 0.28, 0.18, footprint[2] + 0.28),
            new THREE.MeshStandardMaterial({ color: 0x202b39, roughness: 0.62, metalness: 0.18 }),
          );
          roof.position.set(x, footprint[1] + 0.1, z);
          roof.castShadow = true;
          scene!.add(roof);

          const windowMaterial = new THREE.MeshStandardMaterial({
            color: 0x9bdcff,
            emissive: 0x2673a3,
            emissiveIntensity: 0.8,
            roughness: 0.18,
            metalness: 0.25,
          });
          const windowRows = Math.max(1, Math.floor(height / 1.3));
          for (let row = 0; row < windowRows; row += 1) {
            [-0.72, 0, 0.72].forEach((offset) => {
              const windowPane = new THREE.Mesh(new THREE.PlaneGeometry(0.42, 0.34), windowMaterial.clone());
              windowPane.position.set(x + offset, 0.72 + row * 1.05, z + footprint[2] / 2 + 0.012);
              scene!.add(windowPane);
            });
          }
        } else {
          const field = new THREE.Mesh(
            new THREE.PlaneGeometry(3.7, 2.5),
            new THREE.MeshStandardMaterial({ color: 0x2e8b57, roughness: 0.9 }),
          );
          field.rotation.x = -Math.PI / 2;
          field.position.set(x, footprint[1] + 0.03, z);
          scene!.add(field);
        }

        if (place.is_current) {
          const beacon = new THREE.Mesh(
            new THREE.CylinderGeometry(0.18, 0.65, 2.1, 18),
            new THREE.MeshBasicMaterial({ color: 0xffcf66, transparent: true, opacity: 0.82 }),
          );
          beacon.position.set(x, footprint[1] + 1.2, z);
          scene!.add(beacon);
        }
      });

      const raycaster = new THREE.Raycaster();
      const pointer = new THREE.Vector2();
      const cast = (event: PointerEvent) => {
        if (!renderer) return undefined;
        const rect = renderer.domElement.getBoundingClientRect();
        pointer.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
        pointer.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
        raycaster.setFromCamera(pointer, camera);
        return raycaster.intersectObjects(interactives, false)[0]?.object as Mesh | undefined;
      };
      const handleMove = (event: PointerEvent) => {
        const hit = cast(event);
        renderer!.domElement.style.cursor = hit ? 'pointer' : 'grab';
      };
      const handleClick = (event: PointerEvent) => {
        const placeId = cast(event)?.userData.placeId as string | undefined;
        if (placeId) selectPlaceRef.current(placeId);
      };
      renderer.domElement.addEventListener('pointermove', handleMove);
      renderer.domElement.addEventListener('click', handleClick);
      removeListeners = () => {
        renderer?.domElement.removeEventListener('pointermove', handleMove);
        renderer?.domElement.removeEventListener('click', handleClick);
      };

      const resize = () => {
        if (!renderer || !mount.clientWidth || !mount.clientHeight) return;
        camera.aspect = mount.clientWidth / mount.clientHeight;
        camera.updateProjectionMatrix();
        renderer.setSize(mount.clientWidth, mount.clientHeight, false);
      };
      resizeObserver = new ResizeObserver(resize);
      resizeObserver.observe(mount);
      resize();
      const clock = new THREE.Clock();
      const render = () => {
        frameId = window.requestAnimationFrame(render);
        if (!renderer || !scene || document.hidden) return;
        const elapsed = clock.getElapsedTime();
        interactives.forEach((mesh) => {
          if (mesh.userData.placeId === places.find((place) => place.is_current)?.id) {
            mesh.position.y = mesh.userData.baseY + Math.sin(elapsed * 1.4 + mesh.userData.phase) * 0.08;
          }
        });
        renderer.render(scene, camera);
      };
      render();
    };

    void initialize();
    return () => {
      cancelled = true;
      window.cancelAnimationFrame(frameId);
      resizeObserver?.disconnect();
      removeListeners?.();
      scene?.traverse((object) => {
        const mesh = object as Mesh;
        mesh.geometry?.dispose();
        if (mesh.material) {
          const materials: Material[] = Array.isArray(mesh.material) ? mesh.material : [mesh.material];
          materials.forEach((material) => material.dispose());
        }
      });
      renderer?.dispose();
      if (renderer?.domElement.parentElement === mount) mount.removeChild(renderer.domElement);
    };
  }, [cityName, places]);

  const commute = async (mode: string) => {
    if (!selected || selected.is_current) return;
    if (await onCommute(selected.id, mode)) onArrive();
  };

  return (
    <main className="flex-1 overflow-y-auto bg-[#07090e] p-4 md:p-7">
      <div className="mx-auto grid w-full max-w-7xl gap-5 xl:grid-cols-[minmax(0,1fr)_360px]">
        <section className="space-y-3">
          <header className="flex items-end justify-between">
            <div><p className="text-[10px] font-mono uppercase tracking-[0.2em] text-cyan-300">Living city map</p><h2 className="mt-1 font-serif text-2xl font-bold">Move through {cityName}</h2></div>
            <p className="text-xs text-slate-500">Select a building, choose transport, then enter it</p>
          </header>
          <div className="relative min-h-[620px] overflow-hidden rounded-3xl border border-[#26344a] bg-[#07101a] shadow-2xl">
            <div ref={mountRef} className="absolute inset-0" />
            <div className="pointer-events-none absolute inset-x-0 top-0 flex justify-between bg-gradient-to-b from-black/75 to-transparent p-4 text-[10px] font-mono uppercase text-slate-300"><span>3D district view</span><span className="text-amber-300">Gold beacon: your location</span></div>
            {places.map((place) => (
              <button
                key={place.id}
                type="button"
                onClick={() => setSelectedId(place.id)}
                aria-pressed={selected?.id === place.id}
                style={{ left: `${place.map_x}%`, top: `${place.map_y}%` }}
                className={`absolute z-10 -translate-x-1/2 -translate-y-1/2 rounded-full border p-1.5 shadow-lg transition hover:scale-110 focus-visible:outline focus-visible:outline-2 focus-visible:outline-amber-300 ${place.is_current ? 'border-amber-300 bg-amber-300 text-slate-950' : selected?.id === place.id ? 'border-cyan-300 bg-cyan-300 text-slate-950' : 'border-white/30 bg-black/70 text-white'}`}
                aria-label={`${place.name}${place.is_current ? ', current location' : ''}`}
              ><MapPin className="h-4 w-4" /></button>
            ))}
          </div>
        </section>

        {selected ? (
          <aside className="self-start rounded-3xl border border-[#26344a] bg-[#0b1019] p-5 shadow-xl xl:sticky xl:top-4">
            <div className="flex items-start justify-between gap-3"><div><p className="text-[10px] font-mono uppercase text-cyan-300">{selected.category} · {selected.district_name}</p><h3 className="mt-2 font-serif text-xl font-bold text-white">{selected.name}</h3></div>{selected.is_current ? <span className="rounded-full bg-amber-300 px-2 py-1 text-[9px] font-bold text-slate-950">YOU ARE HERE</span> : null}</div>
            <p className="mt-4 text-sm leading-relaxed text-slate-300">{selected.description}</p>
            <div className="mt-4 grid grid-cols-3 gap-2 text-center text-[10px]"><div className="rounded-xl border border-[#26344a] bg-[#111827] p-3"><Clock3 className="mx-auto h-4 w-4 text-cyan-300" /><p className="mt-1">{selected.travel_minutes} min</p></div><div className="rounded-xl border border-[#26344a] bg-[#111827] p-3"><Users className="mx-auto h-4 w-4 text-cyan-300" /><p className="mt-1">{selected.present_people_count} here</p></div><div className="rounded-xl border border-[#26344a] bg-[#111827] p-3"><Navigation className="mx-auto h-4 w-4 text-cyan-300" /><p className={`mt-1 ${selected.is_open ? 'text-emerald-300' : 'text-red-300'}`}>{selected.is_open ? 'Open' : 'Closed'}</p></div></div>
            {selected.is_current ? <button type="button" onClick={onArrive} className="mt-5 w-full rounded-xl bg-amber-400 py-3 text-xs font-bold text-slate-950">Enter this location</button> : <div className="mt-5 space-y-2"><p className="text-[10px] font-mono uppercase text-slate-500">Choose how to get there</p><button type="button" onClick={() => commute('Walk')} disabled={isLoading} className="flex w-full items-center justify-between rounded-xl border border-[#2b3850] bg-[#111827] px-4 py-3 text-xs hover:border-cyan-400 disabled:opacity-40"><span className="flex items-center gap-2"><Bike className="h-4 w-4 text-emerald-300" />Walk</span><span>free · slower</span></button><button type="button" onClick={() => commute('Public Transit')} disabled={isLoading} className="flex w-full items-center justify-between rounded-xl border border-[#2b3850] bg-[#111827] px-4 py-3 text-xs hover:border-cyan-400 disabled:opacity-40"><span className="flex items-center gap-2"><BusFront className="h-4 w-4 text-cyan-300" />Public transit</span><span>{currencySymbol}{selected.travel_cost.toFixed(2)}</span></button><button type="button" onClick={() => commute('Taxi')} disabled={isLoading} className="flex w-full items-center justify-between rounded-xl border border-[#2b3850] bg-[#111827] px-4 py-3 text-xs hover:border-cyan-400 disabled:opacity-40"><span className="flex items-center gap-2"><Navigation className="h-4 w-4 text-amber-300" />Taxi</span><span>{currencySymbol}{(selected.travel_cost * 3).toFixed(2)}</span></button></div>}
          </aside>
        ) : <div className="flex items-center justify-center text-slate-500"><X className="h-5 w-5" /> No city places available</div>}
      </div>
    </main>
  );
};
