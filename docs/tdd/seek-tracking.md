# TDD: Seek Tracking

## Overview

The current seek implementation emits one `seek_start`/`seek_end` pair per `seeking`/`seeked` browser event pair, filtered by a 250ms distance threshold. This works for a single click but fails for continuous scrubbing: dragging the scrub bar fires many rapid `seeking`/`seeked` pairs, generating multiple seek events from one user action. Stall events also fire incorrectly during scrubbing because `onWaiting`/`onBuffering` are not suppressed while seeking is in progress. This TDD replaces the current per-event approach with a debounce-based approach that emits exactly one seek event per user action and suppresses stalls during scrubbing.

---

## Architecture

Changes are confined to **Layer 3** (player integrations). plinth-core and plinth-js are unchanged. All three integrations share the same debounce pattern and are changed identically.

| Component | Change |
|---|---|
| `packages/web/plinth-hlsjs/src/index.ts` | Debounce seek logic; suppress `stall` during seeking |
| `packages/web/plinth-shaka/src/index.ts` | Debounce seek logic; suppress `stall` during seeking |
| `packages/web/plinth-dashjs/src/index.ts` | Debounce seek logic; suppress `stall` during seeking |

No changes to `plinth-core`, `plinth-js`, reference docs, or schema.

---

## Data Models

No new types. Two new local variables per integration inside `attachVideoListeners`:

```ts
let _pendingSeekFrom: number | null = null;   // origin; set only on first seeking of a scrub
let _seekDebounceTimer: ReturnType<typeof setTimeout> | null = null;
```

`isSeeking` (existing class field) continues to guard stall suppression and is now set/cleared inside the debounce callback rather than on `seeked`.

---

## Key Flows

### Click seek (single seeking → seeked pair)

1. `seeking` fires. `_pendingSeekFrom === null` → record `_pendingSeekFrom = lastPlayheadMs`. `isSeeking = true`. Cancel any pending debounce timer (none).
2. `seeked` fires. Cancel any pending timer (none). Start debounce timer (300ms).
3. 300ms later — no further `seeking` events. Timer callback fires:
   - `isSeeking = false`
   - Compute `seekDistance = |currentTime - _pendingSeekFrom|`
   - If `seekDistance > 250`: emit `seek_start(from_ms)` then `seek_end(to_ms, buffer_ready)`
   - Reset `_pendingSeekFrom = null`

### Continuous scrubbing (many seeking → seeked pairs)

1. First `seeking` fires. `_pendingSeekFrom === null` → record origin. `isSeeking = true`.
2. First `seeked` fires. Start debounce timer.
3. Second `seeking` fires within 300ms. `_pendingSeekFrom !== null` → do NOT update origin. Cancel debounce timer.
4. Second `seeked` fires. Start new debounce timer.
5. Steps 3–4 repeat for each drag position.
6. User releases. Last `seeked` fires. Start debounce timer.
7. 300ms of silence. Timer callback fires with original origin and final `currentTime`. Emit one seek event if distance > 250ms.

### Stall suppression during seeking

- `onWaiting` / `onBuffering` (stall path): if `isSeeking === true`, do nothing. `isSeeking` remains true from the first `seeking` event until the debounce callback fires, covering the entire scrub.
- After debounce fires (`isSeeking = false`): if the video is still stalled (waiting for buffered content), the `seek_end` event carries `buffer_ready: false`. plinth-core's `Seeking → Rebuffering` transition handles this correctly — no need for a separate `stall` event.

### destroy() cleanup

`destroy()` must clear the debounce timer to prevent callbacks firing after teardown:

```ts
if (_seekDebounceTimer !== null) {
  clearTimeout(_seekDebounceTimer);
  _seekDebounceTimer = null;
}
```

---

## API Design

No public API changes. This is internal event handling logic only.

### Debounce window

300ms. Not configurable. Long enough to absorb rapid scrub events (browsers typically fire seeking/seeked at 100–200ms intervals during drag); short enough that seek reporting is not meaningfully delayed.

---

## Shaka-specific note

Shaka's stall signal comes from the player `buffering` event (not the video `waiting` event). The `onBuffering` handler already guards recovery (`!isSeeking` on the `playing` emit path). The stall path (`buffering: true`) needs the same guard added:

```ts
const onBuffering: EventListener = (e) => {
  if ((e as any).buffering) {
    if (!this.isSeeking) {
      this.emit(this.hasFiredFirstFrame ? { type: "stall" } : { type: "waiting" });
    }
  } else if (!this.isSeeking) {
    this.emit({ type: "playing" });
  }
};
```

---

## Testing Approach

All tests in the existing test files for each integration. Use `mock.timers` from `node:test` to control `setTimeout` without real delays.

### Setup pattern for timer tests

```ts
beforeEach(() => { mock.timers.enable(["setTimeout"]); });
afterEach(() => { mock.timers.reset(); });
```

Advance time with `mock.timers.tick(300)`.

### New tests per integration (same for hlsjs, shaka, dashjs)

| Test | What it verifies |
|---|---|
| `single seek emits after debounce window` | Fire `seeking` + `seeked`; tick 299ms → no emit; tick 1ms → seek emitted |
| `seek origin is captured from first seeking event` | Fire `seeking` at 5s, then `seeking` again (during scrub) at 10s; settle at 20s; verify `seek_start.from_ms === 5000` |
| `scrubbing emits one seek for many seeking/seeked pairs` | Fire 5 seeking/seeked pairs; tick 300ms; verify exactly one `seek_start` and one `seek_end` |
| `stall suppressed while seeking active` | Fire `seeking`; fire `waiting`/`buffering`; tick 299ms; verify `stall` NOT emitted |
| `stall emitted after debounce settles` | Fire `seeking`; tick 300ms (settle); fire `waiting`; verify `stall` emitted |
| `destroy cancels pending debounce` | Fire `seeking` + `seeked`; call `destroy()`; tick 300ms; verify no seek emitted |
| `seek below 250ms distance not emitted after debounce` | Fire `seeking` + `seeked` with distance 100ms; tick 300ms; verify no seek emitted |

### Existing seek tests to update

The existing seek tests (test 10 in each integration) fire `seeking` + `seeked` directly and assert immediately. These must be updated to tick the debounce timer before asserting.

Tests 11, 12 (buffer_ready checks) fire `seeked` without `seeking`. These need:
- A prior `seeking` event to set `_pendingSeekFrom`
- A `mock.timers.tick(300)` before asserting
- OR restructuring to assert that `seek_end` is emitted as part of the debounce callback

---

## Open Questions

None. All decisions resolved from PRD and codebase.

---

## Associated Documents to Update

None. This feature changes no public API, schema, or beacon payload fields.
