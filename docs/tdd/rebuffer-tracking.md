# TDD: Rebuffer and Seek Buffer Tracking

## Overview

This feature adds three new cumulative metrics to every beacon's `metrics` snapshot: `seek_buffer_ms`, `seek_buffer_count`, and `seek_count`. Together with the existing `rebuffer_ms` and `rebuffer_count`, these give a complete picture of mid-session buffering interruptions. A critical behavioral fix is also included: the current implementation incorrectly starts `rebuffer_tracker` and increments `rebuffer_count` when a seek completes into an empty buffer (`Seeking → Rebuffering`). This must be corrected so that transition feeds the new seek-buffer trackers instead.

---

## Architecture

Changes are confined to **Layer 1 (plinth-core)**. All platform layers (plinth-js, plinth-apple, plinth-android) and player integrations are unaffected except for their Metrics type definitions, which must add the three new fields.

| Component | Change |
|---|---|
| `crates/plinth-core/src/metrics.rs` | Add three fields to `Metrics`; add `seek_buffer_tracker` and counters to session state |
| `crates/plinth-core/src/session.rs` | Fix `Seeking → Rebuffering` tracker assignment; add `seek_count` increment on `SeekStart`; reset new fields in `begin_session` |
| `packages/web/plinth-js/src/types.ts` | Add three fields to `Metrics` interface |
| `packages/apple/plinth-apple/` | Add three fields to Swift `Metrics` struct |
| `packages/android/plinth-android/` | Add three fields to Kotlin `Metrics` data class |
| `docs/reference/beacon-payload.md` | Document new metrics fields |
| `docs/reference/beacon-payload.schema.json` | Add new fields to metrics schema |

---

## Data Models

### `Metrics` struct (`metrics.rs`) — changes

Add three fields to the existing `Metrics` struct:

```rust
pub struct Metrics {
    pub vst_ms: Option<u64>,
    pub played_ms: u64,
    pub rebuffer_ms: u64,
    pub watched_ms: u64,
    pub rebuffer_count: u32,
    pub error_count: u32,
    // New:
    pub seek_buffer_ms: u64,      // ms in Rebuffering entered via Seeking → Rebuffering
    pub seek_buffer_count: u32,   // number of seeks that caused buffering
    pub seek_count: u32,          // total seek events (SeekStart)
}
```

`Metrics` uses `#[derive(miniserde::Serialize, miniserde::Deserialize)]` so the new fields are automatically included in JSON output.

### `Session` struct (`session.rs`) — new fields

```rust
seek_buffer_tracker: TimeTracker,  // mirrors rebuffer_tracker for seek-induced stalls
seek_buffer_count: u32,
seek_count: u32,
```

---

## Key Flows

### SeekStart (Playing → Seeking or Paused → Seeking)

Increment `seek_count` by 1. No tracker changes.

### Seek that lands with buffer ready (Seeking → Playing)

No change to rebuffer or seek-buffer state.

### Seek that lands with empty buffer (Seeking → Rebuffering) — **behavioral fix**

**Current (incorrect):**
- `rebuffer_tracker.start(now_ms)`
- `rebuffer_count += 1`

**Corrected:**
- `seek_buffer_tracker.start(now_ms)`
- `seek_buffer_count += 1`
- `rebuffer_tracker` and `rebuffer_count` are not touched

### Mid-playback stall (Playing → Rebuffering)

No change:
- `rebuffer_tracker.start(now_ms)`
- `rebuffer_count += 1`
- `seek_buffer_tracker` and `seek_buffer_count` are not touched

### Leaving Rebuffering (any transition out)

Stop **both** trackers. Only the one that is running will accumulate time; `TimeTracker::stop` is a no-op when not running, so this is safe regardless of which transition entered Rebuffering.

Affected transitions:
- `Rebuffering → Playing` (recovery)
- `Rebuffering → Paused` (user pauses during stall)
- `Rebuffering → Seeking` (user seeks during stall)

### `begin_session`

Reset all three new fields to zero alongside the existing counters and trackers.

### `snapshot_metrics`

Include all three new fields in the snapshot (they read from `seek_buffer_tracker.current(now_ms)`, `seek_buffer_count`, `seek_count`).

---

## API Design

No public API changes. The new fields appear automatically in the `metrics` object on every beacon (except the session-open `play` beacon). No new methods, events, or configuration is required.

---

## Testing Approach

All tests in `crates/plinth-core/src/session.rs`.

### New tests

| Test | What it verifies |
|---|---|
| `seek_count_increments_on_every_seek_start` | `seek_count` is 1 after one seek, 2 after two seeks |
| `seek_count_increments_from_paused` | `seek_count` increments on SeekStart from Paused state |
| `seek_buffer_count_increments_on_seek_into_empty_buffer` | `Seeking → Rebuffering` increments `seek_buffer_count`, not `rebuffer_count` |
| `seek_buffer_ms_accumulates_seek_induced_stall` | `seek_buffer_ms` grows during Seeking → Rebuffering → Playing |
| `rebuffer_count_not_incremented_by_seek_into_empty_buffer` | `rebuffer_count` stays 0 after seek-induced rebuffer |
| `rebuffer_ms_not_accumulated_by_seek_into_empty_buffer` | `rebuffer_ms` stays 0 after seek-induced rebuffer |
| `seek_buffer_not_affected_by_mid_playback_stall` | Playing → Rebuffering leaves `seek_buffer_ms` and `seek_buffer_count` at 0 |
| `metrics_reset_on_new_session` | All three new fields are 0 after `begin_session` |
| `seek_buffer_ms_stops_on_pause_during_seek_stall` | `Rebuffering → Paused` stops `seek_buffer_tracker` |
| `seek_buffer_ms_stops_on_seek_during_seek_stall` | `Rebuffering → Seeking` stops `seek_buffer_tracker` |

### Existing tests to update

The existing `beacon_batch_roundtrips_through_json` test in `beacon.rs` constructs a `Metrics` literal — add the three new fields with zero values to keep it compiling. Same for any other test that constructs `Metrics` directly.

---

## Associated Documents to Update

- `docs/reference/beacon-payload.md` — add `seek_buffer_ms`, `seek_buffer_count`, `seek_count` to the metrics table
- `docs/reference/beacon-payload.schema.json` — add the three fields as `integer` (required) under `metrics.properties`
- `docs/reference/beacon-payload.samples.json` — add the new fields to all metrics objects in the sample payloads

---

## Out of Scope

- Server-side use of these metrics (dashboards, alerting).
- Seek buffer duration as a separate beacon event (seek_end already marks the boundary; the duration is derivable from `seek_buffer_ms` delta between adjacent beacons).
- Initial startup buffering (already captured in `vst_ms`).
