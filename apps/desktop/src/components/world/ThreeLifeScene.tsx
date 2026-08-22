import React, { useEffect, useRef, useState } from 'react';
import type { Material, Mesh, Object3D, Points, Scene, WebGLRenderer } from 'three';
import { BriefcaseBusiness, ChevronDown, ChevronLeft, ChevronRight, ChevronUp, DoorOpen, FileText, Laptop, Move3d, Smartphone, Users } from 'lucide-react';
import { ContextNpcDTO } from '../characters/NPCDisplay';

interface ThreeLifeSceneProps {
  age: number;
  location: string;
  weatherName: string;
  npcs: ContextNpcDTO[];
  onOpenPhone: () => void;
  onOpenComputer: () => void;
  onOpenDocuments: () => void;
  onOpenTravel: () => void;
  onSelectNpc: (npc: ContextNpcDTO) => void;
}

type InteractionId = 'phone' | 'computer' | 'documents' | 'travel' | `npc:${string}`;
type NavigationDirection = 'left' | 'right' | 'forward' | 'back';

const actionButtons = [
  { id: 'phone' as const, label: 'Use phone', Icon: Smartphone },
  { id: 'computer' as const, label: 'Use computer', Icon: Laptop },
  { id: 'documents' as const, label: 'Inspect documents', Icon: FileText },
  { id: 'travel' as const, label: 'Leave and travel', Icon: DoorOpen },
];

export const ThreeLifeScene: React.FC<ThreeLifeSceneProps> = ({
  age,
  location,
  weatherName,
  npcs,
  onOpenPhone,
  onOpenComputer,
  onOpenDocuments,
  onOpenTravel,
  onSelectNpc,
}) => {
  const mountRef = useRef<HTMLDivElement>(null);
  const callbacksRef = useRef({ onOpenPhone, onOpenComputer, onOpenDocuments, onOpenTravel, onSelectNpc });
  const npcsRef = useRef(npcs);
  const navigateRef = useRef<(direction: NavigationDirection) => void>(() => undefined);
  const [hoveredLabel, setHoveredLabel] = useState('Move through the room and select an object');
  const [renderError, setRenderError] = useState(false);

  callbacksRef.current = { onOpenPhone, onOpenComputer, onOpenDocuments, onOpenTravel, onSelectNpc };
  npcsRef.current = npcs;

  const activate = (interactionId: InteractionId) => {
    if (interactionId === 'phone') callbacksRef.current.onOpenPhone();
    else if (interactionId === 'computer') callbacksRef.current.onOpenComputer();
    else if (interactionId === 'documents') callbacksRef.current.onOpenDocuments();
    else if (interactionId === 'travel') callbacksRef.current.onOpenTravel();
    else if (interactionId.startsWith('npc:')) {
      const npc = npcsRef.current.find((person) => person.id === interactionId.slice(4));
      if (npc) callbacksRef.current.onSelectNpc(npc);
    }
  };

  useEffect(() => {
    const mount = mountRef.current;
    if (!mount) return;

    let cancelled = false;
    let frameId = 0;
    let renderer: WebGLRenderer | undefined;
    let scene: Scene | undefined;
    let resizeObserver: ResizeObserver | undefined;
    let removeInputListeners: (() => void) | undefined;

    const initialize = async () => {
      try {
        const THREE = await import('three');
        if (cancelled) return;

        scene = new THREE.Scene();
        scene.background = new THREE.Color(0x08111d);
        scene.fog = new THREE.Fog(0x08111d, 12, 28);

        const camera = new THREE.PerspectiveCamera(46, 1, 0.1, 80);
        camera.position.set(8.2, 6.1, 9.6);
        const cameraFocus = new THREE.Vector3(0, 1.2, 0);
        camera.lookAt(cameraFocus);

        renderer = new THREE.WebGLRenderer({ antialias: true, alpha: false, powerPreference: 'high-performance' });
        renderer.setPixelRatio(Math.min(window.devicePixelRatio, 1.5));
        renderer.shadowMap.enabled = true;
        renderer.shadowMap.type = THREE.PCFSoftShadowMap;
        renderer.outputColorSpace = THREE.SRGBColorSpace;
        renderer.domElement.setAttribute('aria-label', `Interactive three-dimensional room in ${location}`);
        renderer.domElement.setAttribute('role', 'application');
        renderer.domElement.setAttribute('tabindex', '0');
        renderer.domElement.style.width = '100%';
        renderer.domElement.style.height = '100%';
        renderer.domElement.style.display = 'block';
        renderer.domElement.style.cursor = 'grab';
        mount.appendChild(renderer.domElement);

        const ambient = new THREE.HemisphereLight(0x9fc8ff, 0x18130e, 1.7);
        scene.add(ambient);
        const sun = new THREE.DirectionalLight(0xffdfaa, 3.2);
        sun.position.set(-4, 8, 5);
        sun.castShadow = true;
        scene.add(sun);
        const practical = new THREE.PointLight(0xffb65c, 18, 11, 2);
        practical.position.set(2.5, 3.8, -1.5);
        scene.add(practical);

        const room = new THREE.Group();
        scene.add(room);
        const interactiveMeshes: Mesh[] = [];
        const animatedObjects: Object3D[] = [];

        const box = (
          size: [number, number, number],
          position: [number, number, number],
          color: number,
          roughness = 0.75,
          metalness = 0.05
        ) => {
          const mesh = new THREE.Mesh(
            new THREE.BoxGeometry(...size),
            new THREE.MeshStandardMaterial({ color, roughness, metalness })
          );
          mesh.position.set(...position);
          mesh.castShadow = true;
          mesh.receiveShadow = true;
          room.add(mesh);
          return mesh;
        };

        const makeInteractive = (mesh: Mesh, id: InteractionId, label: string) => {
          mesh.userData.interactionId = id;
          mesh.userData.label = label;
          const material = mesh.material;
          if (material instanceof THREE.MeshStandardMaterial) {
            mesh.userData.baseEmissive = material.emissive.getHex();
          }
          interactiveMeshes.push(mesh);
          return mesh;
        };

        const floor = new THREE.Mesh(
          new THREE.PlaneGeometry(18, 13),
          new THREE.MeshStandardMaterial({ color: 0x17202b, roughness: 0.95 })
        );
        floor.rotation.x = -Math.PI / 2;
        floor.receiveShadow = true;
        room.add(floor);
        box([18, 6, 0.25], [0, 3, -5.9], 0x152131);
        box([0.25, 6, 12], [-8.9, 3, 0], 0x111c2b);

        const rug = new THREE.Mesh(
          new THREE.PlaneGeometry(6.8, 4.4),
          new THREE.MeshStandardMaterial({ color: 0x253f56, roughness: 0.9 })
        );
        rug.rotation.x = -Math.PI / 2;
        rug.position.set(0.2, 0.015, 0.6);
        room.add(rug);

        box([4.8, 0.7, 1.7], [-3.3, 0.62, -2.7], 0x3b4657);
        box([0.45, 1.4, 1.7], [-5.6, 1.15, -2.7], 0x313b4b);
        box([0.45, 1.4, 1.7], [-1.0, 1.15, -2.7], 0x313b4b);
        box([3.5, 0.18, 1.5], [-3.3, 1.35, -2.86], 0xa96534);

        box([3.7, 0.18, 1.65], [3.9, 2.05, -3.35], 0x6f4a2e);
        box([0.18, 2.05, 0.18], [2.3, 1.02, -3.35], 0x4d321f);
        box([0.18, 2.05, 0.18], [5.5, 1.02, -3.35], 0x4d321f);

        const monitor = makeInteractive(box([1.8, 1.12, 0.18], [3.9, 2.85, -3.38], 0x18263b, 0.25, 0.35), 'computer', 'Open the computer');
        const monitorScreen = new THREE.Mesh(
          new THREE.PlaneGeometry(1.55, 0.88),
          new THREE.MeshBasicMaterial({ color: 0x2d87bd })
        );
        monitorScreen.position.set(0, 0, 0.1);
        monitor.add(monitorScreen);
        animatedObjects.push(monitor);

        const phone = makeInteractive(box([0.42, 0.08, 0.78], [1.25, 0.82, 0.3], 0x17243a, 0.22, 0.6), 'phone', 'Pick up the phone');
        phone.rotation.y = -0.24;
        const phoneScreen = new THREE.Mesh(
          new THREE.PlaneGeometry(0.32, 0.62),
          new THREE.MeshBasicMaterial({ color: 0x39c5a3 })
        );
        phoneScreen.rotation.x = -Math.PI / 2;
        phoneScreen.position.set(0, 0.05, 0);
        phone.add(phoneScreen);

        box([2.1, 0.65, 1.25], [1.15, 0.38, 0.25], 0x60422d);
        const papers = makeInteractive(box([1.05, 0.06, 0.72], [0.85, 0.76, 0.1], 0xe7dfc7, 1), 'documents', 'Review personal documents');
        papers.rotation.y = 0.16;

        const door = makeInteractive(box([2.15, 4.6, 0.2], [6.9, 2.3, -5.72], 0x59412f), 'travel', 'Open the door and plan a journey');
        const doorHandle = new THREE.Mesh(
          new THREE.SphereGeometry(0.09, 12, 12),
          new THREE.MeshStandardMaterial({ color: 0xd6a44e, metalness: 0.8, roughness: 0.25 })
        );
        doorHandle.position.set(-0.75, 0, 0.18);
        door.add(doorHandle);

        const windowFrame = box([4.2, 2.5, 0.12], [-4.4, 3.7, -5.68], 0x304960, 0.5, 0.2);
        const windowGlass = new THREE.Mesh(
          new THREE.PlaneGeometry(3.7, 2.05),
          new THREE.MeshStandardMaterial({ color: 0x6fa4be, emissive: 0x183b54, emissiveIntensity: 0.8, transparent: true, opacity: 0.72 })
        );
        windowGlass.position.set(0, 0, 0.08);
        windowFrame.add(windowGlass);

        npcs.slice(0, 3).forEach((npc, index) => {
          const group = new THREE.Group();
          const colors = [0xc57b57, 0x7d91c9, 0x9b6fb1];
          const body = new THREE.Mesh(
            new THREE.CapsuleGeometry(0.34, 1.05, 6, 10),
            new THREE.MeshStandardMaterial({ color: colors[index % colors.length], roughness: 0.75 })
          );
          const head = new THREE.Mesh(
            new THREE.SphereGeometry(0.31, 18, 18),
            new THREE.MeshStandardMaterial({ color: 0x9f684b, roughness: 0.9 })
          );
          head.position.y = 1.05;
          body.add(head);
          group.add(body);
          group.position.set(-1.8 + index * 1.8, 1.05, 2.55 + (index % 2) * 0.45);
          group.userData.interactionId = `npc:${npc.id}` satisfies InteractionId;
          group.userData.label = `Speak with ${npc.name}`;
          body.userData.interactionId = group.userData.interactionId;
          body.userData.label = group.userData.label;
          body.castShadow = true;
          interactiveMeshes.push(body);
          animatedObjects.push(group);
          room.add(group);
        });

        let rain: Points | undefined;
        if (/rain|monsoon|storm/i.test(weatherName)) {
          const positions = new Float32Array(180 * 3);
          for (let index = 0; index < 180; index += 1) {
            positions[index * 3] = -7 + Math.random() * 14;
            positions[index * 3 + 1] = Math.random() * 8;
            positions[index * 3 + 2] = -5.45 + Math.random() * 0.4;
          }
          const geometry = new THREE.BufferGeometry();
          geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));
          rain = new THREE.Points(geometry, new THREE.PointsMaterial({ color: 0x84c7ff, size: 0.055, transparent: true, opacity: 0.72 }));
          room.add(rain);
        }

        const raycaster = new THREE.Raycaster();
        const pointer = new THREE.Vector2();
        let hovered: Mesh | null = null;
        let targetYaw = 0;
        let targetPitch = 0;
        const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;

        const restoreMaterial = (mesh: Mesh | null) => {
          if (!mesh || !(mesh.material instanceof THREE.MeshStandardMaterial)) return;
          mesh.material.emissive.setHex(mesh.userData.baseEmissive ?? 0x000000);
        };

        const setHighlighted = (mesh: Mesh | null) => {
          restoreMaterial(hovered);
          hovered = mesh;
          if (mesh?.material instanceof THREE.MeshStandardMaterial) {
            mesh.material.emissive.setHex(0x6a4514);
          }
          renderer!.domElement.style.cursor = mesh ? 'pointer' : 'grab';
          setHoveredLabel(mesh?.userData.label || 'Move through the room and select an object');
        };

        const cast = (event: PointerEvent) => {
          const rect = renderer!.domElement.getBoundingClientRect();
          pointer.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
          pointer.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
          raycaster.setFromCamera(pointer, camera);
          return (raycaster.intersectObjects(interactiveMeshes, false)[0]?.object as Mesh | undefined) ?? null;
        };

        const handlePointerMove = (event: PointerEvent) => {
          setHighlighted(cast(event));
          const rect = renderer!.domElement.getBoundingClientRect();
          targetYaw = ((event.clientX - rect.left) / rect.width - 0.5) * 0.36;
          targetPitch = ((event.clientY - rect.top) / rect.height - 0.5) * 0.12;
        };
        const handleClick = (event: PointerEvent) => {
          renderer?.domElement.focus();
          const hit = cast(event);
          const interactionId = hit?.userData.interactionId as InteractionId | undefined;
          if (interactionId) activate(interactionId);
        };
        const navigate = (direction: NavigationDirection) => {
          const relative = camera.position.clone().sub(cameraFocus);
          if (direction === 'left' || direction === 'right') {
            relative.applyAxisAngle(new THREE.Vector3(0, 1, 0), direction === 'left' ? 0.16 : -0.16);
          } else {
            const distance = relative.length();
            const nextDistance = THREE.MathUtils.clamp(distance + (direction === 'back' ? 1.15 : -1.15), 6.5, 17);
            relative.setLength(nextDistance);
          }
          camera.position.copy(cameraFocus).add(relative);
          camera.lookAt(cameraFocus);
        };
        navigateRef.current = navigate;
        const handleKeyDown = (event: KeyboardEvent) => {
          const direction = ({
            ArrowLeft: 'left',
            a: 'left',
            ArrowRight: 'right',
            d: 'right',
            ArrowUp: 'forward',
            w: 'forward',
            ArrowDown: 'back',
            s: 'back',
          } as Record<string, NavigationDirection>)[event.key];
          if (!direction) return;
          event.preventDefault();
          navigate(direction);
        };
        const handlePointerLeave = () => setHighlighted(null);
        renderer.domElement.addEventListener('pointermove', handlePointerMove);
        renderer.domElement.addEventListener('click', handleClick);
        renderer.domElement.addEventListener('pointerleave', handlePointerLeave);
        renderer.domElement.addEventListener('keydown', handleKeyDown);
        removeInputListeners = () => {
          renderer?.domElement.removeEventListener('pointermove', handlePointerMove);
          renderer?.domElement.removeEventListener('click', handleClick);
          renderer?.domElement.removeEventListener('pointerleave', handlePointerLeave);
          renderer?.domElement.removeEventListener('keydown', handleKeyDown);
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
        const renderFrame = () => {
          frameId = window.requestAnimationFrame(renderFrame);
          if (!renderer || !scene || document.hidden) return;
          const elapsed = clock.getElapsedTime();
          if (!reducedMotion) {
            room.rotation.y += (targetYaw - room.rotation.y) * 0.025;
            room.rotation.x += (targetPitch - room.rotation.x) * 0.025;
            animatedObjects.forEach((object, index) => {
              object.position.y += Math.sin(elapsed * 0.8 + index) * 0.00045;
            });
            if (rain) {
              rain.position.y -= 0.055;
              if (rain.position.y < -3) rain.position.y = 3;
            }
            practical.intensity = 17 + Math.sin(elapsed * 1.4) * 1.2;
          }
          renderer.render(scene, camera);
        };
        renderFrame();
      } catch (error) {
        console.warn('[ThreeLifeScene] WebGL scene unavailable:', error);
        setRenderError(true);
      }
    };

    void initialize();

    return () => {
      cancelled = true;
      window.cancelAnimationFrame(frameId);
      resizeObserver?.disconnect();
      removeInputListeners?.();
      scene?.traverse((object) => {
        const mesh = object as Mesh;
        if (mesh.geometry) mesh.geometry.dispose();
        if (mesh.material) {
          const materials: Material[] = Array.isArray(mesh.material) ? mesh.material : [mesh.material];
          materials.forEach((material) => material.dispose());
        }
      });
      renderer?.dispose();
      navigateRef.current = () => undefined;
      if (renderer?.domElement.parentElement === mount) mount.removeChild(renderer.domElement);
    };
  }, [age, location, weatherName]);

  return (
    <section className="relative min-h-[480px] overflow-hidden rounded-2xl border border-[#26344a] bg-[#08111d] shadow-2xl md:min-h-[560px]" aria-label="Interactive 3D life scene">
      <div ref={mountRef} className="absolute inset-0" aria-hidden={renderError} />
      <div className="pointer-events-none absolute inset-x-0 top-0 flex items-start justify-between bg-gradient-to-b from-black/70 to-transparent p-4">
        <div>
          <p className="text-[10px] font-mono uppercase tracking-[0.18em] text-cyan-300">Live Three.js Environment</p>
          <p className="mt-1 font-serif text-sm text-white">{location}</p>
        </div>
        <p className="rounded-full border border-white/15 bg-black/35 px-3 py-1 text-[10px] text-slate-200">{weatherName}</p>
      </div>
      <div className="pointer-events-none absolute inset-x-0 bottom-14 flex justify-center px-4">
        <p className="rounded-full border border-amber-400/25 bg-black/65 px-4 py-1.5 text-[11px] font-serif text-amber-100 backdrop-blur">{hoveredLabel}</p>
      </div>
      {renderError ? (
        <div className="absolute inset-0 flex items-center justify-center p-8 text-center">
          <div>
            <BriefcaseBusiness className="mx-auto h-7 w-7 text-amber-400" />
            <p className="mt-3 font-serif text-sm text-slate-200">The 3D room could not start on this device.</p>
            <p className="mt-1 text-xs text-slate-500">All scene actions remain available below.</p>
          </div>
        </div>
      ) : null}
      <div className="absolute right-4 top-1/2 grid -translate-y-1/2 grid-cols-3 gap-1 rounded-xl border border-white/10 bg-black/55 p-2 backdrop-blur" aria-label="3D room camera controls">
        <span />
        <button type="button" onClick={() => navigateRef.current('forward')} aria-label="Move camera closer" className="rounded-md p-2 text-slate-200 hover:bg-white/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-amber-400"><ChevronUp className="h-4 w-4" /></button>
        <span />
        <button type="button" onClick={() => navigateRef.current('left')} aria-label="Look left" className="rounded-md p-2 text-slate-200 hover:bg-white/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-amber-400"><ChevronLeft className="h-4 w-4" /></button>
        <span className="flex items-center justify-center text-amber-300" title="Use arrow keys or WASD"><Move3d className="h-3.5 w-3.5" /></span>
        <button type="button" onClick={() => navigateRef.current('right')} aria-label="Look right" className="rounded-md p-2 text-slate-200 hover:bg-white/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-amber-400"><ChevronRight className="h-4 w-4" /></button>
        <span />
        <button type="button" onClick={() => navigateRef.current('back')} aria-label="Move camera back" className="rounded-md p-2 text-slate-200 hover:bg-white/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-amber-400"><ChevronDown className="h-4 w-4" /></button>
        <span />
      </div>
      <div className="absolute inset-x-0 bottom-0 grid grid-cols-4 border-t border-white/10 bg-black/70 backdrop-blur-md">
        {actionButtons.map(({ id, label, Icon }) => (
          <button key={id} type="button" onClick={() => activate(id)} className="flex items-center justify-center gap-1.5 border-r border-white/10 px-2 py-3 text-[10px] text-slate-200 last:border-r-0 hover:bg-white/10 focus-visible:outline focus-visible:outline-2 focus-visible:outline-amber-400">
            <Icon className="h-3.5 w-3.5 text-amber-300" />
            <span className="hidden sm:inline">{label}</span>
          </button>
        ))}
      </div>
      <div className="sr-only">
        <Users /> Age {age}. People present: {npcs.map((npc) => npc.name).join(', ')}.
      </div>
    </section>
  );
};
