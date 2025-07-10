use common::SystemInfo;
use sysinfo::{System, SystemExt, CpuExt};
use chrono::Utc;
use std::env;
use tracing::{info, warn, error};

// Default server URL - can be overridden by TAILMON_SERVER_URL environment variable
const DEFAULT_SERVER_URL: &str = "http://127.0.0.1:3000/api/metrics";

/// Get server URL from environment variable or use default
fn get_server_url() -> String {
    env::var("TAILMON_SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string())
}

/// Collects system information using sysinfo library
async fn get_system_info() -> SystemInfo {
    // Create a new System instance
    let mut system = System::new_all();
    
    // Refresh all system information
    system.refresh_all();
    
    // Wait a bit for CPU usage calculation (sysinfo needs time to calculate)
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    system.refresh_cpu();
    
    // Get device ID (hostname)
    let device_id = system.host_name().unwrap_or_else(|| "unknown".to_string());
    
    // Get OS information with platform-specific details
    let os_name = system.name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = system.os_version().unwrap_or_else(|| "Unknown".to_string());
    
    // Add platform-specific details
    let platform_specific_details = get_platform_specific_details(&system);
    let os_info = if platform_specific_details.is_empty() {
        format!("{} {}", os_name, os_version)
    } else {
        format!("{} {} ({})", os_name, os_version, platform_specific_details)
    };
    
    // Get CPU usage
    let cpu_usage = system.global_cpu_info().cpu_usage();
    
    // Get RAM information (convert from bytes to MB)
    let ram_used_mb = system.used_memory() / 1024 / 1024;
    let ram_total_mb = system.total_memory() / 1024 / 1024;
    
    // Get current timestamp in ISO 8601 format
    let last_seen = Utc::now().to_rfc3339();
    
    SystemInfo {
        device_id,
        os_info,
        cpu_usage,
        ram_used_mb,
        ram_total_mb,
        last_seen,
    }
}

/// Get platform-specific system details
fn get_platform_specific_details(system: &System) -> String {
    #[cfg(target_os = "linux")]
    {
        let kernel_version = system.kernel_version().unwrap_or_default();
        if !kernel_version.is_empty() {
            return format!("Kernel: {}", kernel_version);
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows specific details could include build number, edition, etc.
        return "Windows".to_string();
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS specific details could include Darwin version, etc.
        return "macOS".to_string();
    }
    
    // Default case for other platforms
    String::new()
}

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter("agent=info")
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
    
    info!("Agent starting...");
    let server_url = get_server_url();
    info!("Will send data to server at: {}", server_url);
    
    // Create HTTP client with timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client");
    
    let mut consecutive_failures = 0;
    const MAX_CONSECUTIVE_FAILURES: u32 = 5;
    
    // Infinite loop to continuously send data
    loop {
        // Collect system information
        let system_info = match get_system_info().await {
            info => {
                info!("Collected system info for device: {}", info.device_id);
                info
            }
        };
        
        // Send data to server
        match client.post(&server_url)
            .json(&system_info)
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("✅ Successfully sent data to server");
                        consecutive_failures = 0; // Reset failure counter on success
                    } else {
                        warn!("❌ Server returned error status: {}", response.status());
                        consecutive_failures += 1;
                    }
                }
                Err(e) => {
                    consecutive_failures += 1;
                    error!("❌ Failed to send data to server: {}", e);
                    
                    // If we have too many consecutive failures, wait longer
                    if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                        warn!("⚠️  Too many consecutive failures ({}), waiting 30 seconds before retry...", consecutive_failures);
                        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                        consecutive_failures = 0; // Reset counter after long wait
                    }
                }
            }
        
        // Wait before next iteration (shorter wait if we had failures)
        let wait_time = if consecutive_failures > 0 {
            std::cmp::min(5 + consecutive_failures * 2, 15) // Progressive backoff, max 15 seconds
        } else {
            5
        };
        
        info!("Waiting {} seconds before next update...", wait_time);
        tokio::time::sleep(tokio::time::Duration::from_secs(wait_time as u64)).await;
    }
} 