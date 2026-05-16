pub mod telemetry;
pub mod interceptor;
pub mod entropy;
pub mod watchdog;
pub mod rollback;
pub mod atomic_rollback;
pub mod self_protection;

pub fn check_status() -> &'static str {
    "Omni-Guard Core Engine Active"
}
