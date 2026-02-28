miniserde::make_place!(Place);

/// 11-state player state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl PlayerState {
    fn as_str(&self) -> &'static str {
        match self {
            PlayerState::Idle => "idle",
            PlayerState::Loading => "loading",
            PlayerState::Ready => "ready",
            PlayerState::PlayAttempt => "play_attempt",
            PlayerState::Buffering => "buffering",
            PlayerState::Playing => "playing",
            PlayerState::Paused => "paused",
            PlayerState::Seeking => "seeking",
            PlayerState::Rebuffering => "rebuffering",
            PlayerState::Ended => "ended",
            PlayerState::Error => "error",
        }
    }
}

impl miniserde::ser::Serialize for PlayerState {
    fn begin(&self) -> miniserde::ser::Fragment {
        miniserde::ser::Fragment::Str(std::borrow::Cow::Borrowed(self.as_str()))
    }
}

impl miniserde::de::Deserialize for PlayerState {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        impl miniserde::de::Visitor for Place<PlayerState> {
            fn string(&mut self, s: &str) -> miniserde::Result<()> {
                self.out = Some(match s {
                    "idle" => PlayerState::Idle,
                    "loading" => PlayerState::Loading,
                    "ready" => PlayerState::Ready,
                    "play_attempt" => PlayerState::PlayAttempt,
                    "buffering" => PlayerState::Buffering,
                    "playing" => PlayerState::Playing,
                    "paused" => PlayerState::Paused,
                    "seeking" => PlayerState::Seeking,
                    "rebuffering" => PlayerState::Rebuffering,
                    "ended" => PlayerState::Ended,
                    "error" => PlayerState::Error,
                    _ => return Err(miniserde::Error),
                });
                Ok(())
            }
        }
        Place::new(out)
    }
}
