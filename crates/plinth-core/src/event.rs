use serde::{Deserialize, Serialize};

use crate::beacon::QualityLevel;

/// All player inputs that the session state machine can act on.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlayerEvent {
    /// Source URL assigned: Idle → Loading.
    Load { src: String },
    /// Metadata / manifest loaded; player can play: Loading → Ready.
    CanPlay,
    /// play() called by user: Ready/Paused/Ended → PlayAttempt.
    Play,
    /// Buffer empty / stall: PlayAttempt → Buffering, Playing → Rebuffering.
    Waiting,
    /// First decoded frame rendered (initial play only): PlayAttempt/Buffering → Playing.
    FirstFrame,
    /// Buffer sufficiently full to resume: Buffering/Rebuffering → Playing.
    CanPlayThrough,
    /// User or system paused: Playing/Rebuffering → Paused.
    Pause,
    /// Seek initiated. `from_ms` is the playhead position before the seek.
    SeekStart { from_ms: u64 },
    /// Seek completed. `to_ms` is the final playhead position.
    /// `buffer_ready` determines whether seek_end resolves to Playing or Rebuffering
    /// when `pre_seek_state` was Playing.
    SeekEnd { to_ms: u64, buffer_ready: bool },
    /// Video reached natural end: Playing → Ended.
    Ended,
    /// Player error occurred.
    Error {
        code: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
        fatal: bool,
    },
    /// SDK destroyed; graceful teardown from Ready/Paused/Ended/Error.
    Destroy,
    /// Rendition quality changed while Playing.
    QualityChange { quality: QualityLevel },
}
