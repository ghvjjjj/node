use config::Config;
use config::ConfigError;
use config::File;
use serde::Deserialize;

use crate::clash::DelayTestConfig;
use crate::speedtest::SpeedTestConfig;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Settings {
    pub fast_mode: bool,
    pub subs: Vec<String>,
    pub rename_node: bool,
    pub rename_pattern: String,
    pub need_add_pool: bool,
    pub test_group_size: usize,
    pub pools: Vec<String>,
    pub connect_test: DelayTestConfig,
    pub speed_test: SpeedTestConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("conf/config.toml"))
            .build()?;
        settings.try_deserialize::<Settings>()
    }
}
