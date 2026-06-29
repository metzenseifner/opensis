use std::collections::HashMap;

pub struct RecordingController {
    cache: SmpCache,                               // from the companion note
    states: dashmap::DashMap<DeviceId, DeviceRec>, // per-device FSM + register
}

struct DeviceRec {
    state: RecordingState,
    metadata: HashMap<MetadataField, String>, // the register M
}

impl Default for DeviceRec {
    fn default() -> Self {
        Self {
            state: RecordingState::Idle,
            metadata: HashMap::new(),
        }
    }
}
