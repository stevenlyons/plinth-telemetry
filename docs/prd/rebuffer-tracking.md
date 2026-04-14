# Feature PRD: Rebuffer and Seek Buffer Tracking

## Overview

Buffering interruptions are among the most damaging viewer experience events in video streaming. plinth-telemetry tracks two distinct kinds of mid-session buffering that developers need to measure separately:

**Rebuffering** — the player runs out of content during active playback. This is an unexpected stall the viewer did not initiate and is the primary QoE signal for CDN and encoding health.

**Seek buffering** — the viewer seeks to a position that is not yet downloaded and the player must fetch content before resuming. This is an expected consequence of the seek action, not a network failure, and warrants its own measurement because it directly reflects seek responsiveness.

Initial startup buffering (the time between pressing play and the first frame) is Video Start Time and is tracked separately. It is not included in either metric defined here.

## Goals

- Measure the total time and count of mid-playback rebuffer events so teams can quantify unexpected stalls.
- Measure the total time and count of seek-induced buffering events so teams can quantify seek responsiveness.
- Keep the two metrics separate so dashboards can distinguish infrastructure problems (rebuffering) from seek latency (seek buffering).
- Surface both metrics on every beacon as cumulative snapshots.

## Personas

- **Developers**: Identify content segments, CDN regions, or time periods with high rebuffer rates; separately track whether seek performance meets expectations.
- **Product Managers**: Track rebuffer rate and seek buffer rate as top-level QoE KPIs, set SLOs, and measure the impact of infrastructure or player changes.
- **Viewers**: Benefit from improvements driven by accurate data without direct interaction.

## Rebuffering

A rebuffer event begins when the player stalls **after the first frame has been rendered** and the buffer runs dry. In state machine terms this is the `Playing → Rebuffering` transition, triggered by a `stall` event.

A rebuffer event ends when the player recovers and resumes playback, the viewer pauses, or the viewer seeks away.

Initial startup buffering (`PlayAttempt / Buffering` states, before the first frame) is **not** rebuffering. Seek-induced buffering is **not** rebuffering.

### Rebuffer metrics

| Field | Type | Description |
|---|---|---|
| `rebuffer_ms` | integer | Cumulative milliseconds spent rebuffering due to mid-playback stalls (`Playing → Rebuffering` only). |
| `rebuffer_count` | integer | Number of discrete mid-playback rebuffer events. |

## Seek Buffering

A seek buffer event begins when a seek completes at a position that is not yet downloaded, causing the player to wait for content before it can resume playing. In state machine terms this is the `Seeking → Rebuffering` transition (seekEnd with an empty buffer).

A seek buffer event ends when the player has downloaded enough content to resume playback.

### Seek buffer metrics

| Field | Type | Description |
|---|---|---|
| `seek_buffer_ms` | integer | Cumulative milliseconds spent buffering after seeks (`Seeking → Rebuffering` only). |
| `seek_count` | integer | Number of seek events initiated during the session (`SeekStart` events). |
| `seek_buffer_count` | integer | Number of seeks that resulted in buffering (`Seeking → Rebuffering` transitions). |

## Behavioral Rules

### Rebuffering
1. `rebuffer_ms` starts accumulating on the `Playing → Rebuffering` (`stall`) transition.
2. `rebuffer_ms` stops accumulating when leaving `Rebuffering` for any reason (recovery, pause, or seek away).
3. `rebuffer_count` increments by one each time rule 1 triggers.
4. The `Seeking → Rebuffering` transition does **not** affect `rebuffer_ms` or `rebuffer_count`.

### Seek buffering
5. `seek_count` increments by one on every `SeekStart` event, regardless of whether the seek results in buffering.
6. `seek_buffer_ms` starts accumulating on the `Seeking → Rebuffering` transition (seekEnd with empty buffer).
7. `seek_buffer_ms` stops accumulating when leaving `Rebuffering` for any reason (recovery, pause, or seek away).
8. `seek_buffer_count` increments by one each time rule 6 triggers.
9. The `Playing → Rebuffering` transition does **not** affect `seek_buffer_ms`, `seek_count`, or `seek_buffer_count`.

### All metrics
10. All six fields reset to zero at the start of each new play session (`play` beacon, seq=0).
11. All six fields are included in the cumulative `metrics` snapshot on every beacon except the session-open `play` beacon.

## Out of Scope

- Initial startup buffering time (counted in VST, not here).
- Detection of ABR quality drops without a full stall (tracked as `quality_change` events).
- Server-side rebuffer detection or validation against playhead data (future work).
