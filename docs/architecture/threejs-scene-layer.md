# Three.js scene-layer decision

## Decision

Use Three.js as an optional, progressively enhanced scene renderer. Do not make it the simulation engine, application shell, form system, conversation UI, or source of world truth.

The Rust simulation remains authoritative for people, places, time, money, processes, documents, travel, messages, and causality. React remains authoritative for readable and accessible interface elements. A Three.js canvas may render the current physical setting from a small immutable scene snapshot supplied by the simulation.

## Implemented first slice

The desktop life scene now dynamically loads a real Three.js `WebGLRenderer` and constructs a playable low-poly home interior. The phone, computer, document folder, exit door, and visible NPCs are raycast targets. Each target opens the same React workflow as its labelled keyboard-accessible control beneath the canvas.

The scene also maps the current location and weather into its presentation, including a rain particle layer where appropriate. Rendering pauses while the document is hidden, resizes with its container, honors reduced motion, and disposes scene resources on unmount. A readable DOM fallback remains available if WebGL cannot start.

## Why it fits

Three.js provides a mature scene graph, cameras, lighting, animation, asset loaders, picking, WebGL rendering, and an emerging WebGPU path. Those capabilities can make rooms, streets, campuses, offices, terminals, weather, and time-of-day transitions feel inhabited without rebuilding the game architecture around a 3D engine.

Useful first scenes include:

- a low-poly home interior with clickable phone, computer, documents, doors, and household members;
- city exterior establishing shots whose lighting and weather reflect simulation time;
- travel terminals and arrival transitions;
- schools, offices, clinics, and government counters composed from reusable scene kits;
- subtle ambient motion, spatial audio hooks, and camera parallax.

## Boundary

The renderer receives a `SceneVisualState` projection and emits only semantic interaction IDs.

```ts
interface SceneVisualState {
  sceneId: string;
  locationId: string;
  localHour: number;
  weather: string;
  season: string;
  people: Array<{ id: string; role: string; pose: string }>;
  interactables: Array<{ id: string; kind: string; state: string }>;
  qualityTier: 'low' | 'medium' | 'high';
}
```

A click such as `interactable:personal_computer` opens the existing React computer interface. It must not directly change money, location, relationships, or time. Those changes go through typed simulation commands and are autosaved.

## Delivery sequence

1. **Complete:** create a dynamically imported scene canvas behind the existing `SceneWorkspace` contract.
2. **Complete:** ship one home scene with primitive geometry, camera, lighting, resize, disposal, raycasting, and accessible interaction parity.
3. **In progress:** expand the environment kit and deepen simulation weather/time mapping beyond the initial rain and ambient-state treatment.
4. Add glTF assets only after memory, bundle-size, and loading budgets are measured in the Tauri WebView.
5. Expand one location family at a time while preserving the DOM version as a fallback.

## Runtime requirements

- Install the published `three` package at an explicitly pinned version; do not copy or vendor the upstream repository.
- Begin with `WebGLRenderer`. Treat WebGPU as an opt-in experiment until support is verified across supported Tauri WebViews.
- Load the renderer and scene assets only while the life scene is visible.
- Pause animation when the window is hidden and dispose of geometries, materials, textures, controls, and event listeners on unmount.
- Provide low, medium, and high quality tiers plus a reduced-motion mode.
- Keep phone, computer, documents, forms, messages, captions, and conversations in accessible React DOM.
- Every canvas interaction must have a keyboard-accessible DOM equivalent and visible focus state.
- Show explicit loading, asset-failure, unsupported-renderer, and recovery states.

## Acceptance gate for the first scene

- The scene loads without blocking the menu or saved-life restoration.
- Resizing and fullscreen changes preserve aspect ratio and interaction accuracy.
- Clicking a rendered object opens the same feature as its DOM equivalent.
- No render-frame callback mutates simulation state.
- The app remains fully playable when the canvas is disabled.
- Idle scene rendering stays within the agreed CPU/GPU and memory budgets on representative Macs.
- Automated build checks and a manual visual pass cover small, medium, and large desktop windows.

## Upstream references

- Repository and overview: <https://github.com/mrdoob/three.js>
- Package exports and current release metadata: <https://github.com/mrdoob/three.js/blob/master/package.json>
- MIT license: <https://github.com/mrdoob/three.js/blob/master/LICENSE>
