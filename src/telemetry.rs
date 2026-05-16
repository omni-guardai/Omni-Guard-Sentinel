use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// 💡 In sysinfo v0.30+, extension traits like CpuExt are completely removed.
// We only import the native structs we need.
use sysinfo::System;

#[derive(Serialize, Deserialize, Clone)]
pub struct SnapshotPayload {
    pub agent_id: String,
    pub user_id: String,
    pub status: String,
    pub cpu_usage: f64,
    pub ram_usage: f64,
    pub threat_level: String,
    pub last_entropy_spike: Option<f64>,
    pub protected_blocks: u64,
    pub updated_at: u64,
}

pub fn collect_telemetry() -> SnapshotPayload {
    let mut sys = System::new_all();
    sys.refresh_all();

    // 💡 Read real metrics directly using standard v0.30 architecture methods
    let cpu_usage = if let Some(global_cpu) = sys.cpus().first() {
        global_cpu.cpu_usage() as f64
    } else {
        0.0
    };

    let total_memory = sys.total_memory() as f64;
    let ram_usage = if total_memory > 0.0 {
        (sys.used_memory() as f64 / total_memory) * 100.0
    } else {
        0.0
    };

    SnapshotPayload {
        agent_id: "agent_01".to_string(),
        user_id: "user_4ever".to_string(),
        status: "active".to_string(),
        cpu_usage,
        ram_usage,
        threat_level: "low".to_string(),
        last_entropy_spike: None,
        protected_blocks: 42,
        updated_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    }
}
