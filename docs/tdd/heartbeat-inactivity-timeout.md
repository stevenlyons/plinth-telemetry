# TDD: Heartbeat Inactivity Timeout

## Overview

Heartbeats currently fire indefinitely in the `Paused` state (the only inactive state where heartbeats are currently emitted; `Ended` and `Error` already suppress them via the existing active-state guard). This change adds an inactivity timer inside `plinth-core`: when the session has been in `Paused` continuously for 60 seconds, heartbeat emission is suppressed. Heartbeats resume immediately when any player event moves the session back to an active state. No platform-layer changes are required — the platforms keep calling `tick()` on their existing schedules and receive empty responses when suppressed.

---

## Architecture

All changes are confined to **Layer 1** (`plinth-core`). Platform wrappers (`plinth-js`, `plinth-apple`, `plinth-android`) and player integrations (Layer 3) are unchanged.

| Component | Change |
|---|---|
| `crates/plinth-core/src/session.rs` | Add `inactive_since_ms` field; update `process_event` and `tick` |

No changes to config, FFI, Wasm bindings, beacon schema, or any platform package.

---

## Data Models

### `Session` struct — new field

```rust
pub struct Session {
    // ... existing fields ...

    /// Timestamp (ms) when the session entered an inactive state (Paused, Ended, Error).
    /// None when the session is active or not yet started.
    inactive_since_ms: Option<u64>,
}
```

**Inactive states** (heartbeat suppressed after timeout): `Paused`, `Ended`, `Error`

**Active states** (heartbeat never suppressed): `Playing`, `Buffering`, `Seeking`, `Rebuffering`

Pre-playback states (`Idle`, `Loading`, `Ready`, `PlayAttempt`) do not participate because they never produce heartbeats.

### Inactivity timeout constant

```rust
const HEARTBEAT_INACTIVITY_TIMEOUT_MS: u64 = 60_000;
```

Defined as a module-level constant in `session.rs`. Not configurable in this release.

---

## Key Flows

### Entering an inactive state (Paused)

1. `process_event` drives a state transition to `Paused`.
2. Before returning, `inactive_since_ms` is set to `now_ms` if not already set.
3. Heartbeat interval continues normally. Each `tick(now_ms)` call computes elapsed = `now_ms − inactive_since_ms`. While elapsed < 60 000ms, heartbeats fire as usual.
4. Once elapsed ≥ 60 000ms, `tick` returns `vec![]` — no beacon, no update to `last_heartbeat_ms`.

### Resuming from Paused (e.g., play pressed)

1. `process_event` drives a state transition to `Playing` (or `Buffering`).
2. `inactive_since_ms` is cleared (`= None`).
3. `last_heartbeat_ms` is updated to `now_ms` by the beacon emitted for the transition (existing behavior: any beacon resets the heartbeat countdown).
4. Next `tick` after `heartbeat_interval_ms` fires normally.

### Transitioning between inactive states (Paused → Ended)

1. `process_event` transitions from `Paused` to `Ended`.
2. `inactive_since_ms` is **reset** to the new `now_ms` (the timer restarts from the moment of the new transition).
3. `Ended` never fires heartbeats under the existing active-state guard, so suppression from the new timer is moot — but the reset ensures correctness if state definitions change.

### Transition into Error

Same as Ended: `inactive_since_ms` set/reset to `now_ms`. Already suppressed by existing guard, but tracked for consistency.

### destroy()

`destroy()` is unaffected. It emits a final beacon unconditionally regardless of `inactive_since_ms`.

---

## Implementation — `session.rs`

### `Session::new`

Initialize `inactive_since_ms: None`.

### `process_event` — update `inactive_since_ms` after state transition

After each state transition, add:

```rust
fn update_inactivity(&mut self, now_ms: u64) {
    match self.state {
        PlayerState::Paused | PlayerState::Ended | PlayerState::Error => {
            if self.inactive_since_ms.is_none() {
                self.inactive_since_ms = Some(now_ms);
            }
        }
        _ => {
            self.inactive_since_ms = None;
        }
    }
}
```

Call `self.update_inactivity(now_ms)` at the end of `process_event`, after the state field is updated and beacons are collected.

**Important:** When transitioning from one inactive state to another (e.g., `Paused → Ended`), the existing guard sets `inactive_since_ms` only when `None`. To reset the timer on a transition between inactive states, use:

```rust
PlayerState::Paused | PlayerState::Ended | PlayerState::Error => {
    // Reset on every transition into an inactive state, not just the first
    self.inactive_since_ms = Some(now_ms);
}
```

This is the correct behavior per the PRD.

### `Session::tick` — add inactivity suppression

After the existing active-state guard, add:

```rust
if let Some(inactive_since) = self.inactive_since_ms {
    if now_ms.saturating_sub(inactive_since) >= HEARTBEAT_INACTIVITY_TIMEOUT_MS {
        return vec![];
    }
}
```

Full updated `tick` (showing insertion point):

```rust
pub fn tick(&mut self, now_ms: u64) -> Vec<Beacon> {
    let last = match self.last_heartbeat_ms {
        Some(t) => t,
        None => return vec![],
    };

    if now_ms.saturating_sub(last) < self.config.heartbeat_interval_ms {
        return vec![];
    }

    let active = matches!(
        self.state,
        PlayerState::PlayAttempt
            | PlayerState::Buffering
            | PlayerState::Playing
            | PlayerState::Paused
            | PlayerState::Seeking
            | PlayerState::Rebuffering
    );

    if !active {
        return vec![];
    }

    // Suppress heartbeat if the session has been inactive for too long
    if let Some(inactive_since) = self.inactive_since_ms {
        if now_ms.saturating_sub(inactive_since) >= HEARTBEAT_INACTIVITY_TIMEOUT_MS {
            return vec![];
        }
    }

    self.last_heartbeat_ms = Some(now_ms);
    let m = self.snapshot_metrics(now_ms);
    let playhead = self.playhead_ms;
    let mut b = self.make_beacon(BeaconEvent::Heartbeat, Some(self.state), Some(m), now_ms);
    b.playhead_ms = Some(playhead);
    vec![b]
}
```

---

## API Design

No public API changes. `tick()` signature is unchanged at all layers. The suppression is transparent — callers receive an empty beacon list, exactly as they do today when the interval hasn't elapsed.

---

## Testing Approach

All new tests go in `crates/plinth-core/tests/` (or alongside existing session tests).

### Unit tests — inactivity suppression

| Test | Scenario | Assertion |
|---|---|---|
| `heartbeat_suppressed_after_60s_paused` | Pause session; advance time 59 999ms; tick → heartbeat emitted. Advance 1ms more; tick → no heartbeat. | Pass/fail on beacon count |
| `heartbeat_resumes_after_resume_from_pause` | Pause; advance 65s (suppress); emit play event; advance heartbeat_interval_ms; tick → heartbeat emitted | Beacon present |
| `inactivity_timer_resets_on_transition_between_inactive_states` | Pause at t=0; advance 50s; end session (Ended) at t=50s; advance 15s; tick → no beacon (only 15s since Ended, well under 60s) | Beacon absent (only 15s elapsed since Ended, but Ended is also not in active set — verify timer reset, not beacon) |
| `inactivity_timer_cleared_on_active_state` | Pause at t=0; advance 30s; resume play; advance 30s more; tick → heartbeat (timer reset on resume, 30s < 60s) | Beacon present |
| `heartbeat_fires_normally_during_first_59s_of_pause` | Pause; advance 10s; tick → heartbeat. Advance 10s; tick → heartbeat. (× 5) 6th tick at 60s → suppressed | 5 heartbeats, 6th suppressed |
| `destroy_always_emits_regardless_of_suppression` | Pause; advance 65s (suppress); call destroy → beacon returned | Beacon present |

### Regression

Run the full existing test suite (`cargo test -p plinth-core`) to confirm no existing heartbeat behavior changed.

---

## Open Questions

None. All decisions resolved from PRD and codebase review.

---

## Out of Scope

- Configurable timeout duration.
- Automatic session termination or cleanup after inactivity.
- Platform-layer changes (all layers are unchanged).

---

## Associated Documents

No updates required to beacon schema, payload reference, state machine diagram, or quickstart guides. This is an internal SDK behavior change with no externally visible API or payload differences.
