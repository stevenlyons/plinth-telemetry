# Notes: Heartbeat Inactivity Timeout

## Context

Heartbeats fire forever when paused. Goal: suppress after 60s of inactivity.

## Key decision: where does the logic live?

All in `plinth-core/session.rs`. Platform layers are unchanged — they keep calling `tick()` and receive empty vec when suppressed.

## Scoping clarification

`Ended` and `Error` already suppress heartbeats via the existing active-state guard in `tick()`. The only state this feature actually changes behavior for is `Paused`. The TDD tracks all three for consistency.

## Timer reset on inactive→inactive transition

PRD rule: Paused → Ended resets the 60s timer. Implemented by always writing `inactive_since_ms = Some(now_ms)` on entry to any inactive state, not just when it's currently None.

## No platform changes

Confirmed from codebase: all platform timers (JS setInterval, Swift DispatchSourceTimer, Android coroutine delay) just call `tick()` and act on the returned beacons. Empty vec = no-op. No changes needed anywhere outside plinth-core.

## destroy() unaffected

destroy() bypasses tick() entirely — it has its own beacon path. Suppression does not apply.
