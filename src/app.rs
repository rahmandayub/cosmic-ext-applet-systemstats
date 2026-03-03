use cosmic::app::{Core, Task};
use cosmic::iced::{Alignment, Color, Limits, Subscription};
use cosmic::iced::time;
use cosmic::iced_widget::{container, Row};
use cosmic::widget::{autosize, text};
use cosmic::Element;
use std::time::Duration;

use crate::color::Status;
use crate::config::Config;
use crate::formatting::*;
use crate::monitors::MonitorStats;

/// Create colored text using container with text_color style
fn colored_text<'a>(content: String, color: Color) -> Element<'a, Message> {
    container(text(content))
        .style(move |_| container::Style {
            text_color: Some(color),
            ..Default::default()
        })
        .into()
}

const ID: &str = "com.github.rylan-x.systemstats";

pub struct SystemStats {
    core: Core,
    monitors: MonitorStats,
    config: Config,
}

/// Messages the applet can receive
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

impl cosmic::Application for SystemStats {
    type Executor = cosmic::executor::Default;
    type Flags = Config;
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, config: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = SystemStats {
            core,
            monitors: MonitorStats::new(&config),
            config,
        };
        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::Tick => {
                self.monitors.update(&self.config);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let mut row_children: Vec<Element<'_, Message>> = Vec::new();
        let mut first = true;

        // Helper to add separator before non-first items
        let mut add_separator = |children: &mut Vec<Element<'_, Message>>| {
            if !first {
                children.push(text(" | ").into());
            }
            first = false;
        };

        // CPU Usage
        if self.config.monitors.cpu_usage {
            let cpu_usage = self.monitors.cpu.usage();
            let (formatted, status) = format_percentage_with_status(
                cpu_usage,
                self.config.thresholds.cpu.low_max,
                self.config.thresholds.cpu.high_min
            );
            add_separator(&mut row_children);
            row_children.push(colored_text(format!("CPU: {}", formatted), status.to_color()));
        }

        // CPU Temperature
        if self.config.monitors.cpu_temperature {
            if let Some(temp) = self.monitors.temperature.cpu_celsius() {
                let (formatted, status) = format_celsius_with_status(
                    temp,
                    self.config.thresholds.temperature.low_max,
                    self.config.thresholds.temperature.high_min
                );
                add_separator(&mut row_children);
                row_children.push(colored_text(formatted, status.to_color()));
            }
        }

        // GPU Temperature
        if self.config.monitors.gpu_temperature {
            if let Some(temp) = self.monitors.temperature.gpu_celsius() {
                let (formatted, status) = format_celsius_with_status(
                    temp,
                    self.config.thresholds.temperature.low_max,
                    self.config.thresholds.temperature.high_min
                );
                add_separator(&mut row_children);
                row_children.push(colored_text(format!("GPU: {}", formatted), status.to_color()));
            }
        }

        // Memory
        if self.config.monitors.memory {
            let used_gb = self.monitors.memory.used_gb();
            let total_gb = self.monitors.memory.total_gb();
            let (formatted, status) = format_memory_gb_with_status(
                used_gb,
                total_gb,
                self.config.thresholds.memory.low_max,
                self.config.thresholds.memory.high_min
            );
            add_separator(&mut row_children);
            row_children.push(colored_text(format!("RAM: {}/{}", formatted, format_memory_gb(total_gb)), status.to_color()));
        }

        // Network
        if self.config.monitors.network {
            let download_speed = format_network_speed(self.monitors.network.download_bps());
            let upload_speed = format_network_speed(self.monitors.network.upload_bps());
            add_separator(&mut row_children);
            // Download in green
            row_children.push(colored_text(format!("↓{}", download_speed), Status::network_download_color()));
            // Upload in amber
            row_children.push(colored_text(format!(" ↑{}", upload_speed), Status::network_upload_color()));
        }

        // If no elements, show empty
        if row_children.is_empty() {
            row_children.push(text("").into());
        }

        let content = Row::from_vec(row_children)
            .padding([0, 8])
            .align_y(Alignment::Center)
            .spacing(0);

        let limits = Limits::NONE
            .max_width(600.0)
            .min_height(1.0)
            .max_height(128.0);

        autosize::autosize(content, cosmic::widget::Id::unique())
            .limits(limits)
            .into()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(Duration::from_millis(self.config.refresh_interval_ms)).map(|_| Message::Tick)
    }
}
