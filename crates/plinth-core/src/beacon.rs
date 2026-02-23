use serde::{Deserialize, Serialize};

use crate::metrics::Metrics;
use crate::state::PlayerState;

/// Beacon event type — drives which additional fields are present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BeaconEvent {
    SessionOpen,
    FirstFrame,
    Pause,
    Play,
    SeekStart,
    SeekEnd,
    RebufferStart,
    RebufferEnd,
    QualityChange,
    Error,
    Heartbeat,
    SessionEnd,
}

/// A single beacon. Flat struct with `skip_serializing_if` on optional fields
/// so the JSON matches the schema's conditional-field rules without a wrapper enum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beacon {
    pub seq: u32,
    pub play_id: String,
    pub ts: u64,
    pub event: BeaconEvent,

    // Present on all beacons except session_open.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<PlayerState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,

    // session_open fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<VideoMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk: Option<SdkMetadata>,

    // heartbeat field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playhead_ms: Option<u64>,

    // seek fields (present on seek_start and seek_end).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seek_from_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seek_to_ms: Option<u64>,

    // quality_change field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<QualityLevel>,

    // error field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<PlayerError>,
}

/// HTTP POST body — wraps a batch of beacons from a single play session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconBatch {
    pub beacons: Vec<Beacon>,
}

impl BeaconBatch {
    pub fn new(beacons: Vec<Beacon>) -> Self {
        BeaconBatch { beacons }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// ── Metadata types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMetadata {
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkMetadata {
    pub api_version: u32,
    pub core: SdkComponent,
    pub framework: SdkComponent,
    pub player: SdkComponent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkComponent {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityLevel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate_bps: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerError {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub fatal: bool,
}
