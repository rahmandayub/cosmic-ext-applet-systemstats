//! Color utilities for status-based theming
//!
//! Provides status detection (Low/Medium/High) based on configurable thresholds
//! and returns appropriate colors for each status level.
//!
//! Colors are fixed to ensure visibility in both light and dark themes:
//! - Low (Green): #4CAF50
//! - Medium (Yellow/Amber): #FFC107
//! - High (Red): #F44336

use cosmic::iced::Color;

/// Convert a hex color code to Color
///
/// # Arguments
/// * `hex` - Hex color code in 0xRRGGBB format
///
/// # Example
/// ```
/// let green = hex_color(0x4CAF50);
/// ```
fn hex_color(hex: u32) -> Color {
    Color::from_rgb(
        ((hex >> 16) & 0xFF) as f32 / 255.0,
        ((hex >> 8) & 0xFF) as f32 / 255.0,
        (hex & 0xFF) as f32 / 255.0,
    )
}

/// Status levels for color-coding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Low usage/normal state - displayed in green
    Low,
    /// Medium usage/warning state - displayed in yellow/amber
    Medium,
    /// High usage/critical state - displayed in red
    High,
}

impl Status {
    /// Determine status based on value and thresholds
    ///
    /// # Arguments
    /// * `value` - The current value to evaluate
    /// * `low_max` - Maximum value for Low status (below this = Low)
    /// * `high_min` - Minimum value for High status (above this = High)
    ///
    /// Values between low_max and high_min are Medium status
    pub fn from_value(value: f32, low_max: f32, high_min: f32) -> Self {
        if value < low_max {
            Status::Low
        } else if value >= high_min {
            Status::High
        } else {
            Status::Medium
        }
    }

    /// Get the appropriate color for this status
    /// Returns fixed colors that work well in both light and dark themes
    pub fn to_color(self) -> Color {
        match self {
            Status::Low => hex_color(0x4CAF50),      // Green
            Status::Medium => hex_color(0xFFC107),   // Amber
            Status::High => hex_color(0xF44336),     // Red
        }
    }

    /// Get network download color (green)
    pub fn network_download_color() -> Color {
        hex_color(0x4CAF50)  // Green
    }

    /// Get network upload color (amber)
    pub fn network_upload_color() -> Color {
        hex_color(0xFFC107)   // Amber
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_from_value_cpu() {
        // CPU thresholds: low_max=40, high_min=75
        assert_eq!(Status::from_value(30.0, 40.0, 75.0), Status::Low);
        assert_eq!(Status::from_value(40.0, 40.0, 75.0), Status::Medium);
        assert_eq!(Status::from_value(60.0, 40.0, 75.0), Status::Medium);
        assert_eq!(Status::from_value(75.0, 40.0, 75.0), Status::High);
        assert_eq!(Status::from_value(90.0, 40.0, 75.0), Status::High);
    }

    #[test]
    fn test_status_from_value_temperature() {
        // Temperature thresholds: low_max=60, high_min=80
        assert_eq!(Status::from_value(50.0, 60.0, 80.0), Status::Low);
        assert_eq!(Status::from_value(70.0, 60.0, 80.0), Status::Medium);
        assert_eq!(Status::from_value(85.0, 60.0, 80.0), Status::High);
    }
}
