# Notes: Seek Tracking

## Key Decisions

- **Debounce window: 300ms.** Long enough to absorb rapid scrub events (browsers fire seeking/seeked at ~100–200ms intervals during drag). Not configurable in this version.
- **`_pendingSeekFrom` only set on first `seeking` of a scrub.** Subsequent `seeking` events during the same scrub (timer still pending) do NOT overwrite the origin. This preserves the true seek origin across a long scrub.
- **`isSeeking = false` set in debounce callback, not on `seeked`.** This is what keeps stall suppression active for the entire duration of the scrub.
- **`destroy()` must clear the timer.** Prevents a post-teardown callback from emitting seek events.

## What Changes Where

All changes are Layer 3 only (player integrations). No plinth-core or plinth-js changes.

- hlsjs, dashjs: `onWaiting` — add `if (!this.isSeeking)` guard before stall/waiting emit
- shaka: `onBuffering` stall path — add `if (!this.isSeeking)` guard (recovery path already guarded)
- All three: debounce `setTimeout`/`clearTimeout` in seek handler
- All three: clear timer in `destroy()`

## Test Infrastructure

Node 24 supports `mock.timers.enable(["setTimeout"])` + `mock.timers.tick(ms)`. Tests use `beforeEach/afterEach` to enable/reset. Existing seek tests (test 10 in each integration) must be updated to tick 300ms before asserting.
