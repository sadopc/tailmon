// Tailmon Dashboard JavaScript

// Fetch metrics from server
async function fetchMetrics() {
    try {
        const response = await fetch('/api/all_metrics');
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const devices = await response.json();
        return devices;
    } catch (error) {
        console.error('Error fetching metrics:', error);
        return [];
    }
}

// Render dashboard with device data
function renderDashboard(devices) {
    const container = document.getElementById('dashboard-container');
    
    if (devices.length === 0) {
        container.innerHTML = `
            <div class="device-card" style="grid-column: 1 / -1; text-align: center;">
                <h3>No devices connected</h3>
                <p>Waiting for agents to connect...</p>
            </div>
        `;
        return;
    }
    
    container.innerHTML = devices.map(device => {
        // Determine status class based on CPU and RAM usage
        let statusClass = '';
        if (device.cpu_usage > 80 || (device.ram_used_mb / device.ram_total_mb) > 0.9) {
            statusClass = 'status-critical';
        } else if (device.cpu_usage > 60 || (device.ram_used_mb / device.ram_total_mb) > 0.7) {
            statusClass = 'status-warning';
        }
        
        // Calculate RAM usage percentage
        const ramUsagePercent = ((device.ram_used_mb / device.ram_total_mb) * 100).toFixed(1);
        
        // Format last seen time
        const lastSeen = new Date(device.last_seen);
        const timeAgo = getTimeAgo(lastSeen);
        
        return `
            <div class="device-card ${statusClass}">
                <div class="device-header">
                    <div class="device-name">${escapeHtml(device.device_id)}</div>
                    <div class="device-os">${escapeHtml(device.os_info)}</div>
                </div>
                
                <div class="metrics-grid">
                    <div class="metric-item">
                        <div class="metric-label">CPU Usage</div>
                        <div class="metric-value cpu-usage">${device.cpu_usage.toFixed(1)}<span class="metric-unit">%</span></div>
                    </div>
                    
                    <div class="metric-item">
                        <div class="metric-label">RAM Usage</div>
                        <div class="metric-value ram-usage">${ramUsagePercent}<span class="metric-unit">%</span></div>
                    </div>
                    
                    <div class="metric-item">
                        <div class="metric-label">RAM Used</div>
                        <div class="metric-value">${device.ram_used_mb}<span class="metric-unit">MB</span></div>
                    </div>
                    
                    <div class="metric-item">
                        <div class="metric-label">RAM Total</div>
                        <div class="metric-value">${device.ram_total_mb}<span class="metric-unit">MB</span></div>
                    </div>
                </div>
                
                <div class="last-seen">
                    Last seen: ${timeAgo}
                </div>
            </div>
        `;
    }).join('');
}

// Helper function to escape HTML
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

// Helper function to get time ago
function getTimeAgo(date) {
    const now = new Date();
    const diffInSeconds = Math.floor((now - date) / 1000);
    
    if (diffInSeconds < 60) {
        return `${diffInSeconds} seconds ago`;
    } else if (diffInSeconds < 3600) {
        const minutes = Math.floor(diffInSeconds / 60);
        return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
    } else if (diffInSeconds < 86400) {
        const hours = Math.floor(diffInSeconds / 3600);
        return `${hours} hour${hours > 1 ? 's' : ''} ago`;
    } else {
        const days = Math.floor(diffInSeconds / 86400);
        return `${days} day${days > 1 ? 's' : ''} ago`;
    }
}

// Update dashboard every 3 seconds
async function updateDashboard() {
    const devices = await fetchMetrics();
    renderDashboard(devices);
}

// Initialize dashboard
document.addEventListener('DOMContentLoaded', () => {
    console.log('Tailmon Dashboard loaded');
    
    // Initial load
    updateDashboard();
    
    // Update every 3 seconds
    setInterval(updateDashboard, 3000);
}); 