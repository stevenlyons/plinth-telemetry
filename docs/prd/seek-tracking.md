# Feature PRD: Seek Tracking

## Overview

The current seek implementation emits one `seek_start`/`seek_end` pair per browser `seeking`/`seeked` event, filtered by a 250ms distance threshold. This works for a single click on the seek bar, but fails for continuous scrubbing: a user dragging the playhead across the timeline generates many rapid `seeking`/`seeked` pairs, which produces multiple seek events and potentially multiple stall events for what is conceptually one user action.

This feature defines the correct behavior for seek tracking across both interaction styles.

---

## Goals

- Emit exactly one `seek_start`/`seek_end` pair per user seek action, regardless of how many browser seeking events it generates.
- Suppress buffering events (`waiting`/`stall`) while the user is actively scrubbing.
- Accurately record the origin and destination of each seek action.

---

## Personas

- **Developers**: Want clean per-seek data without noise from intermediate scrub positions.
- **Product Managers**: Want accurate seek counts and seek buffer time; inflated counts from scrubbing distort KPIs.

---

## Seek Interaction Types

### Click seek

The viewer clicks a point on the seek bar. The browser fires one `seeking` event followed by one `seeked` event. The distance between the origin and destination is typically large (> 250ms).

**Expected behavior:** One `seek_start`/`seek_end` pair is emitted.

### Continuous scrubbing

The viewer clicks and drags the seek bar, pausing at intermediate positions before settling on a final one. The browser fires many `seeking` events (one per position change) and the corresponding `seeked` events while the viewer is dragging.

**Expected behavior:** One `seek_start`/`seek_end` pair is emitted for the entire scrub, from the playhead position when scrubbing began to the position where the viewer released.

---

## Behavioral Rules

### Seek event emission

1. A seek action begins when the first `seeking` event fires. Record the playhead position at this moment as `seek_from_ms`.
2. A seek action ends when no new `seeking` event has fired for a debounce window (e.g., 300ms) after the last `seeked` event.
3. On end of seek, emit `seek_start` (with the original `seek_from_ms`) followed by `seek_end` (with the final `to_ms` and `buffer_ready` state).
4. If the total distance between `seek_from_ms` and the final `to_ms` is ≤ 250ms, do not emit any seek events (this filters player-internal nudge seeks).

### Buffering suppression during scrub

5. While a seek action is in progress (between the first `seeking` event and the end-of-seek debounce), `waiting` events must not emit `stall` or `waiting` player events. The viewer is intentionally seeking; buffering during that time is not a stall.
6. After the seek action ends (`seek_end` has been emitted), if the player is still waiting for content (`buffer_ready: false`), normal seek-buffer tracking resumes and the state machine transitions to `Rebuffering` via the `SeekEnd` event.

### Interaction with existing distance filter

Rule 4 (distance ≤ 250ms = no emit) still applies after debouncing. This handles cases where the viewer scrubs and returns to nearly the same position.

---

## Out of Scope

- Server-side seek event validation.
- Seek bar UI or player chrome (this is purely SDK instrumentation).
- Distinguishing keyboard seeks (arrow keys) from pointer seeks.
