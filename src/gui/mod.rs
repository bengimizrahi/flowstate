use crate::app::*;
use crate::support;

mod constants;
use constants::*;
mod menu_bar;
mod ribbon;
mod tab_bar;
mod gantt_chart;
mod gantt_chart_resources;
mod gantt_chart_tasks;
mod inspection;
mod inspection_task;
mod inspection_resource;
mod config;
use config::GuiConfig;
mod utils;
mod gui;
pub use gui::Gui;

#[macro_export]
macro_rules! gui_log {
    ($gui:expr, $($arg:tt)*) => {
        $gui.log(format!($($arg)*))
    };
}

use imgui::*;
use imgui::sys::*;
use chrono::{Utc, DateTime, Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;