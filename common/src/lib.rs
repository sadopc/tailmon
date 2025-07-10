use serde::{Deserialize, Serialize};

/// System information structure that will be sent from agent to server
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    /// Device identifier (e.g., machine name or unique ID)
    pub device_id: String,
    
    /// Operating system information (e.g., "Ubuntu 22.04")
    pub os_info: String,
    
    /// CPU usage as percentage
    pub cpu_usage: f32,
    
    /// Used RAM in MB
    pub ram_used_mb: u64,
    
    /// Total RAM in MB
    pub ram_total_mb: u64,
    
    /// Timestamp when data was sent (ISO 8601 format)
    pub last_seen: String,
} 