// interceptor.rs
// Boilerplate for native OS filesystem filtering

use crate::entropy::calculate_shannon_entropy;
use sysinfo::System;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct FilesystemInterceptor {
    sys: Arc<Mutex<System>>,
}

impl FilesystemInterceptor {
    pub fn new(sys: Arc<Mutex<System>>) -> Self {
        Self { sys }
    }

    /// Mock function mimicking a kernel IRP_MJ_WRITE callback (Windows) 
    /// or ES_EVENT_TYPE_AUTH_WRITE (macOS).
    pub async fn on_write_callback(&self, pid: u32, filepath: &str, buffer: &[u8]) -> bool {
        let entropy = calculate_shannon_entropy(buffer);
        
        if entropy > 7.5 {
            println!("[!] ALERT: High Entropy Spike Detected ({}). File: {}", entropy, filepath);
            self.suspend_process(pid).await;
            return false; // Block the write
        }
        
        true // Allow the write
    }

    async fn suspend_process(&self, pid: u32) {
        let mut sys = self.sys.lock().await;
        sys.refresh_processes();
        
        if let Some(process) = sys.process(sysinfo::Pid::from_u32(pid)) {
            println!("[*] Suspending Malicious Process: {} (PID: {})", process.name(), pid);
            process.kill(); // In native API: SuspendThread / task_suspend
            println!("[✓] Neural Freeze Complete.");
        }
    }
}
