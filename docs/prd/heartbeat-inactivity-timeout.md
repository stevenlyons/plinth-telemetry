# Feature PRD: Heartbeat Inactivity Timeout

## Overview

Heartbeat beacons are sent on a fixed interval to confirm an active playback session is still alive. Currently, heartbeats fire indefinitely regardless of playback state — including when the player is paused, stalled, or idle. A session where nothing is happening is not meaningfully "active," and continuous beacons waste bandwidth, inflate server ingestion costs, and pollute analytics with stale keepalives.

This feature adds an inactivity timeout: if no playback-advancing state change has occurred for 60 seconds, heartbeat emission is suspended until the session becomes active again.

---

## Goals

- Stop emitting heartbeats when a session has been inactive for 60 continuous seconds.
- Resume heartbeats immediately when playback resumes.
- Avoid false positives: do not suppress heartbeats during legitimate active states (buffering, seeking).

---

## Personas

- **Developers**: Want SDK behavior that does not send spurious data on behalf of idle sessions.
- **Product Managers**: Want heartbeat counts to reflect genuine viewer engagement, not paused or abandoned sessions.
- **Infrastructure / Data Engineering**: Want to reduce beacon volume from sessions that are no longer actively consuming content.

---

## Inactivity Definition

A session is **inactive** when it has been in any of the following states for 60 continuous seconds without a state transition:

- `Paused`
- `Ended`
- `Error`

A session is **active** when it is in any of the following states, regardless of how long it has been there:

- `Buffering`
- `Playing`
- `Seeking`
- `Rebuffering`

The `Loading`, `Ready`, `PlayAttempt`, and `Idle` states are pre-playback and do not generate heartbeats, so the timeout does not apply to them.

---

## Behavioral Rules

### Suspension

1. When the session enters `Paused`, `Ended`, or `Error`, start a 60-second inactivity timer.
2. If the timer elapses without a state transition, suspend heartbeat emission. No heartbeat is emitted at the moment of suspension.
3. While suspended, `tick()` calls are no-ops with respect to heartbeat output.

### Resumption

4. When any player event causes a state transition out of an inactive state (e.g., the viewer presses play while paused), resume heartbeat emission immediately.
5. Resumption resets the heartbeat interval — a heartbeat will next fire after one full `heartbeat_interval_ms` has elapsed from the moment of resumption.
6. If the session is already in an active state when the inactivity timer would have been read, the timer is cancelled and no suspension occurs.

### State transitions within inactive states

7. Transitioning from one inactive state to another (e.g., `Paused → Ended`) resets the inactivity timer to 60 seconds from the time of the new transition.

### Session destroy

8. `destroy()` is not affected by inactivity state — it always emits its final beacon regardless of whether heartbeats are suspended.

---

## Configuration

- The inactivity timeout duration is **not configurable** in this release. It is fixed at 60 seconds.
- The feature is **always on** — there is no opt-out flag.

---

## Out of Scope

- Configurable timeout duration (future consideration).
- Automatic session termination after inactivity (this feature only suppresses heartbeats; it does not close the session).
- Network-level connection teardown or resource cleanup beyond heartbeat suppression.
