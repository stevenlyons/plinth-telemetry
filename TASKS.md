# Plinth Video QoE SDK Implementation

A proof-of-concept SDK framework for measuring Video Quality of Experience across native and web platforms. Architecture: Rust cross-platform core → platform-specific framework → player-specific integration.

---

## Completed Tasks

### Phase 1 — Rust Core (`plinth-core`)

- [x] Set up Rust crate with Cargo workspace structure
- [x] Define public API types: `VideoMetadata`, `SdkMetadata`, `PlayerEvent`, `BeaconPayload`
- [x] Implement player state machine (Idle → Loading → Ready → PlayAttempt → Buffering → Playing → Paused → Seeking → Rebuffering → Ended → Error)
- [x] Implement time tracking: Video Start Time, Played Time, Rebuffer Time, Watched Time
- [x] Implement beacon sequence number and play session ID generation (UUID v4)
- [x] Implement heartbeat timer (platform calls `tick(now_ms)`; core checks elapsed interval)
- [x] Build HTTP beacon payload serialization (JSON via serde)
- [x] Make HTTP endpoint configurable (default: `http://localhost:3000/beacon`)
- [x] Write unit tests for state machine transitions (42 tests, all passing)
- [x] Write unit tests for time accumulators
- [x] Write unit tests for beacon payload serialization

---

## In Progress Tasks

*(none)*

---

## Completed Tasks (continued)

### Phase 2 — JavaScript/Web Platform Framework (`plinth-js`)

- [x] Set up bun workspace (`package.json`, `packages/plinth-js/`)
- [x] Add `cdylib` crate-type + `wasm-bindgen` dep to `plinth-core`
- [x] Add `Serialize/Deserialize` to `Config`, `SessionMeta`, `PlayerEvent`
- [x] Implement `WasmSession` in `src/wasm.rs` (JSON-over-wasm-bindgen boundary)
- [x] Build Wasm via wasm-pack → `packages/plinth-js/wasm/`
- [x] Implement `PlinthSession` TypeScript class (heartbeat, fire-and-forget POST)
- [x] Write `src/types.ts`, `src/poster.ts`, `src/index.ts`
- [x] Write 14 unit tests with mock Wasm module (all passing)

---

## Future Tasks

### Phase 3 — Hls.js Player Integration (`plinth-hlsjs`)

- [x] Set up TypeScript package for Hls.js integration
- [x] Implement `PlinthHlsJs` class with `initialize(hls, video, videoMeta, options?)` and `destroy()` API
- [x] Map Hls.js events to core PlayerEvents: play, pause, ended, waiting (rebuffer start/end), first frame, error, seek start/end
- [x] Forward playback position (currentTime) to core via `setPlayhead`
- [x] Pass User Agent, Video Title, Video ID metadata
- [x] Write 19 unit tests with FakeHls + FakeVideo test doubles (all passing)

### Phase 6 — Browser Demo + Dev Server (`dev/`)

- [x] Create `dev/` workspace (`@plinth/dev`) with Bun
- [x] Implement `dev/server.ts`: builds `main.ts`, copies wasm binary to dist, serves static files, receives and logs `POST /beacon`
- [x] Implement `dev/index.html`: dark-themed UI with video element, HLS URL input, Load button, live event log panel
- [x] Implement `dev/main.ts`: wires Hls.js + PlinthHlsJs with `loggingSessionFactory` that mirrors each event to the page log
- [x] Verify end-to-end: build succeeds, wasm served, beacon POST returns 200 and logs payload

### Phase 4 — Swift/iOS Platform Framework (`plinth-swift`)

- [ ] Set up Swift Package with XCFramework dependency on Rust native lib
- [ ] Implement Swift FFI bindings to Rust core C API
- [ ] Implement platform bindings: URLSession HTTP, Timer, Date
- [ ] Expose platform framework protocol/interface

### Phase 5 — AVPlayer Integration (iOS)

- [ ] Implement `PlintheAVPlayer` with `initialize`, `updateMetadata`, `destroy` API
- [ ] Map AVPlayer/AVPlayerItem KVO and notifications to core PlayerEvents
- [ ] Forward scrubber position (currentTime) to core
- [ ] Verify beacon submission end-to-end on device/simulator

### Phase 7 — Documentation & Developer Experience

- [ ] Write quick-start guide for Hls.js integration
- [ ] Document player-specific SDK API (initialize, updateMetadata, destroy)
- [ ] Document beacon payload schema
- [ ] Document how to add a new player integration

---

## Implementation Plan

### Architecture

```
plinth-core (Rust)
  ├── compiled → wasm  → plinth-js (TypeScript framework)
  │                          └── plinth-hlsjs (Hls.js integration)
  └── compiled → .a/.dylib → plinth-swift (Swift framework)
                                  └── plinth-avplayer (AVPlayer integration)
```

### Data Flow

1. Player fires native event → player integration maps to `PlayerEvent`
2. `PlayerEvent` sent to platform framework → forwarded to Rust core via Wasm/FFI
3. Core updates state machine and time accumulators
4. Core emits beacon payload on event or heartbeat tick
5. Platform framework transmits beacon via HTTP POST to configured endpoint

### Beacon Payload Fields

- `seq` — sequence number (integer, 0-based per session)
- `play_id` — UUID generated at play start
- `ts` — reliable client timestamp (ms)
- `event` — event type string
- `state` — current player state
- `video_start_time_ms` — ms from play() to first frame
- `rebuffer_time_ms` — cumulative rebuffer ms
- `watched_time_ms` — cumulative watched ms
- `played_time_ms` — cumulative played ms
- `project_key` — `p123456789` (hardcoded for PoC)
- `meta` — `{ ua, title, video_id, core_version, framework_version, sdk_version, api_version }`

### Relevant Files

- `docs/prd.md` — product requirements
- `TASKS.md` — this file
