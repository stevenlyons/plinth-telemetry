use serde::{Deserialize, Serialize};

/// 11-state player state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerState {
    /// No video source loaded; initial state after init or reset.
    Idle,
    /// Source assigned; fetching manifest / metadata.
    Loading,
    /// Metadata loaded; player paused at start position.
    Ready,
    /// play() called; awaiting first decoded frame. VST + WatchedTime begin here.
    PlayAttempt,
    /// play() called but buffer is empty; downloading initial segment.
    Buffering,
    /// Frames actively being decoded and rendered. PlayedTime accumulates.
    Playing,
    /// Playback suspended by user or system; position held.
    Paused,
    /// Seek requested; position changing, frames not rendered.
    Seeking,
    /// Was Playing; buffer exhausted mid-playback; stalled. RebufferTime accumulates.
    Rebuffering,
    /// Playback reached end of content naturally.
    Ended,
    /// Fatal, unrecoverable player error.
    Error,
}
