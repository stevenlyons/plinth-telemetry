# Notes: Rebuffer and Seek Buffer Tracking

## Key Decisions

- **Seek-induced rebuffering is a separate metric, not excluded.** The PRD originally said post-seek buffering "does not count" as rebuffering. Correct — but it's tracked separately as `seek_buffer_ms` / `seek_buffer_count`, not dropped.
- **`seek_count` counts all SeekStart events**, not just those that cause buffering. This lets callers compute `seek_buffer_count / seek_count` as a "seek stall rate".
- **`seek_buffer_count` != `seek_count`** — a seek to a buffered position doesn't increment `seek_buffer_count`.

## Behavioral Fix

Current session.rs `Seeking → Rebuffering` arm (lines ~403–413) incorrectly calls `rebuffer_tracker.start()` and increments `rebuffer_count`. Must change to `seek_buffer_tracker.start()` and `seek_buffer_count += 1`.

## Implementation Notes

- `Metrics` uses `#[derive(miniserde::Serialize, miniserde::Deserialize)]` — new fields are free.
- `Beacon` serializer in beacon.rs is hand-rolled (state machine) — no change needed there since Metrics serialization is derived.
- Leaving Rebuffering: stop both trackers unconditionally; `TimeTracker::stop` is a no-op when idle.
- `begin_session` must reset: `seek_buffer_tracker.reset()`, `seek_buffer_count = 0`, `seek_count = 0`.
