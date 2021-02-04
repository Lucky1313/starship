use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CmdDurationConfig<'a> {
    pub min_time: i64,
    pub format: &'a str,
    pub style: &'a str,
    pub show_milliseconds: bool,
    pub disabled: bool,
    pub show_notifications: bool,
    pub min_time_to_notify: i64,
    pub notify_display_time: u64,
}

impl<'a> RootModuleConfig<'a> for CmdDurationConfig<'a> {
    fn new() -> Self {
        CmdDurationConfig {
            min_time: 2_000,
            format: "took [$duration]($style) ",
            show_milliseconds: false,
            style: "yellow bold",
            disabled: false,
            show_notifications: false,
            min_time_to_notify: 45_000,
            notify_display_time: 750,
        }
    }
}
