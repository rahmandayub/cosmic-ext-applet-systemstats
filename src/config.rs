use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Helper to parse hex color string (with or without # prefix) to u32
fn parse_hex_color(s: &str) -> u32 {
    let s = s.trim().trim_start_matches('#');
    u32::from_str_radix(s, 16).unwrap_or(0x00B4D8)
}

/// Label configuration with name and color
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LabelConfig {
    /// Display name for the label (e.g., "CPU:", "GPU:")
    #[serde(default)]
    pub name: String,
    /// Hex color code (e.g., "00B4D8" or "#00B4D8")
    #[serde(default)]
    pub color: String,
}

impl LabelConfig {
    /// Get the color as a u32 value
    pub fn color_hex(&self) -> u32 {
        parse_hex_color(&self.color)
    }
}

impl Default for LabelConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: String::new(),
        }
    }
}

/// Label configurations for all monitors
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Labels {
    #[serde(default)]
    pub cpu: LabelConfig,
    #[serde(default)]
    pub cpu_temp: LabelConfig,
    #[serde(default)]
    pub gpu_temp: LabelConfig,
    #[serde(default)]
    pub ram: LabelConfig,
    #[serde(default)]
    pub network_download: LabelConfig,
    #[serde(default)]
    pub network_upload: LabelConfig,
}

impl Default for Labels {
    fn default() -> Self {
        Self {
            cpu: LabelConfig {
                name: "CPU: ".to_string(),
                color: "00B4D8".to_string(),  // Blue
            },
            cpu_temp: LabelConfig {
                name: "TEMP: ".to_string(),
                color: "FFD700".to_string(),  // Yellow
            },
            gpu_temp: LabelConfig {
                name: "GPU: ".to_string(),
                color: "00D4AA".to_string(),  // Teal
            },
            ram: LabelConfig {
                name: "RAM: ".to_string(),
                color: "9B5DE5".to_string(),  // Purple
            },
            network_download: LabelConfig {
                name: "".to_string(),  // Not used, just for color
                color: "00B4D8".to_string(),  // Blue for download arrow
            },
            network_upload: LabelConfig {
                name: "".to_string(),  // Not used, just for color
                color: "FB8500".to_string(),  // Orange for upload arrow
            },
        }
    }
}

/// CPU usage thresholds (percentage)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CpuThresholds {
    #[serde(default = "default_cpu_low_max")]
    pub low_max: f32,
    #[serde(default = "default_cpu_high_min")]
    pub high_min: f32,
}

impl Default for CpuThresholds {
    fn default() -> Self {
        Self {
            low_max: default_cpu_low_max(),
            high_min: default_cpu_high_min(),
        }
    }
}

fn default_cpu_low_max() -> f32 { 40.0 }
fn default_cpu_high_min() -> f32 { 75.0 }

/// Memory usage thresholds (percentage)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryThresholds {
    #[serde(default = "default_memory_low_max")]
    pub low_max: f32,
    #[serde(default = "default_memory_high_min")]
    pub high_min: f32,
}

impl Default for MemoryThresholds {
    fn default() -> Self {
        Self {
            low_max: default_memory_low_max(),
            high_min: default_memory_high_min(),
        }
    }
}

fn default_memory_low_max() -> f32 { 50.0 }
fn default_memory_high_min() -> f32 { 80.0 }

/// Temperature thresholds (Celsius)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemperatureThresholds {
    #[serde(default = "default_temp_low_max")]
    pub low_max: f32,
    #[serde(default = "default_temp_high_min")]
    pub high_min: f32,
}

impl Default for TemperatureThresholds {
    fn default() -> Self {
        Self {
            low_max: default_temp_low_max(),
            high_min: default_temp_high_min(),
        }
    }
}

fn default_temp_low_max() -> f32 { 60.0 }
fn default_temp_high_min() -> f32 { 80.0 }

/// All threshold configurations
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Thresholds {
    #[serde(default)]
    pub cpu: CpuThresholds,
    #[serde(default)]
    pub memory: MemoryThresholds,
    #[serde(default)]
    pub temperature: TemperatureThresholds,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            cpu: CpuThresholds::default(),
            memory: MemoryThresholds::default(),
            temperature: TemperatureThresholds::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_refresh_interval")]
    pub refresh_interval_ms: u64,

    #[serde(default)]
    pub monitors: MonitorToggles,

    #[serde(default)]
    pub thresholds: Thresholds,

    #[serde(default)]
    pub labels: Labels,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitorToggles {
    #[serde(default = "default_true")]
    pub cpu_usage: bool,

    #[serde(default = "default_true")]
    pub cpu_temperature: bool,

    #[serde(default = "default_true")]
    pub gpu_temperature: bool,

    #[serde(default = "default_true")]
    pub memory: bool,

    #[serde(default = "default_true")]
    pub network: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            refresh_interval_ms: default_refresh_interval(),
            monitors: MonitorToggles::default(),
            thresholds: Thresholds::default(),
            labels: Labels::default(),
        }
    }
}

impl Default for MonitorToggles {
    fn default() -> Self {
        Self {
            cpu_usage: true,
            cpu_temperature: true,
            gpu_temperature: true,
            memory: true,
            network: true,
        }
    }
}

fn default_refresh_interval() -> u64 {
    1000 // Milliseconds
}

fn default_true() -> bool {
    true
}

impl Config {
    /// Load config from XDG config directory or create default if it doesn't exist
    pub fn load() -> Self {
        match Self::config_path() {
            Some(path) => {
                if path.exists() {
                    match fs::read_to_string(&path) {
                        Ok(contents) => match toml::from_str(&contents) {
                            Ok(config) => {
                                log::info!("Loaded config from {}", path.display());
                                config
                            }
                            Err(e) => {
                                log::warn!("Failed to parse config file: {}. Using defaults.", e);
                                Self::default()
                            }
                        },
                        Err(e) => {
                            log::warn!("Failed to read config file: {}. Using defaults.", e);
                            Self::default()
                        }
                    }
                } else {
                    // Create default config file
                    let default_config = Self::default();
                    if let Err(e) = Self::create_default_config(&path, &default_config) {
                        log::warn!("Failed to create default config: {}. Using in-memory defaults.", e);
                    } else {
                        log::info!("Created default config at {}", path.display());
                    }
                    default_config
                }
            }
            None => {
                log::warn!("Could not determine config directory. Using defaults.");
                Self::default()
            }
        }
    }

    /// Get the config file path following XDG Base Directory spec
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("systemstats");
            path.push("config.toml");
            path
        })
    }

    /// Create default config file
    fn create_default_config(path: &PathBuf, config: &Config) -> std::io::Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_content = format!(
r#"# SystemStats Configuration

# Refresh interval in milliseconds (default: 1000 = 1 second)
refresh_interval_ms = {}

[monitors]
# Toggle individual monitors on/off

cpu_usage = {}

cpu_temperature = {}

gpu_temperature = {}

memory = {}

network = {}

[thresholds.cpu]
# CPU usage thresholds (percentage)
# Values < low_max = Low (blue)
# Values between low_max and high_min = Medium (orange)
# Values >= high_min = High (red)
low_max = {}
high_min = {}

[thresholds.memory]
# Memory usage thresholds (percentage)
low_max = {}
high_min = {}

[thresholds.temperature]
# Temperature thresholds (Celsius)
low_max = {}
high_min = {}

[labels.cpu]
# CPU usage label name and color (hex without #)
name = "{}"
color = "{}"

[labels.cpu_temp]
# CPU temperature label name and color
name = "{}"
color = "{}"

[labels.gpu_temp]
# GPU temperature label name and color
name = "{}"
color = "{}"

[labels.ram]
# Memory label name and color
name = "{}"
color = "{}"

[labels.network_download]
# Network download arrow color (name is not used)
color = "{}"

[labels.network_upload]
# Network upload arrow color (name is not used)
color = "{}"
"#,
            config.refresh_interval_ms,
            config.monitors.cpu_usage,
            config.monitors.cpu_temperature,
            config.monitors.gpu_temperature,
            config.monitors.memory,
            config.monitors.network,
            config.thresholds.cpu.low_max,
            config.thresholds.cpu.high_min,
            config.thresholds.memory.low_max,
            config.thresholds.memory.high_min,
            config.thresholds.temperature.low_max,
            config.thresholds.temperature.high_min,
            config.labels.cpu.name,
            config.labels.cpu.color,
            config.labels.cpu_temp.name,
            config.labels.cpu_temp.color,
            config.labels.gpu_temp.name,
            config.labels.gpu_temp.color,
            config.labels.ram.name,
            config.labels.ram.color,
            config.labels.network_download.color,
            config.labels.network_upload.color
        );

        fs::write(path, config_content)
    }
}
