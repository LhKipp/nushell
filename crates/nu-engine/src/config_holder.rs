use std::path::{Path, PathBuf};

use nu_data::config::{config_defaults, ConfigValueOrDefault, NuConfig};
use nu_errors::ShellError;
use nu_protocol::{ConfigPath, Value};

/// ConfigHolder holds information which configs have been loaded.
#[derive(Clone)]
pub struct ConfigHolder {
    pub global_config: Option<NuConfig>,
    pub local_configs: Vec<NuConfig>,
}

impl Default for ConfigHolder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigHolder {
    pub fn new() -> ConfigHolder {
        ConfigHolder {
            global_config: None,
            local_configs: vec![],
        }
    }

    pub fn add_local_cfg(&mut self, cfg: NuConfig) {
        self.local_configs.push(cfg);
    }

    pub fn set_global_cfg(&mut self, cfg: NuConfig) {
        self.global_config = Some(cfg);
    }

    pub fn remove_cfg(&mut self, cfg_path: &ConfigPath) {
        match cfg_path {
            ConfigPath::Global(_) => self.global_config = None,
            ConfigPath::Local(p) => self.remove_local_cfg(p),
        }
    }

    fn remove_local_cfg<P: AsRef<Path>>(&mut self, cfg_path: P) {
        // Remove the first loaded local config with specified cfg_path
        if let Some(index) = self
            .local_configs
            .iter()
            .rev()
            .position(|cfg| cfg.file_path == cfg_path.as_ref())
        {
            self.local_configs.remove(index);
        }
    }

    fn config_iter(&self) -> impl Iterator<Item = &NuConfig> + '_ {
        self.local_configs.iter().rev().chain(&self.global_config)
    }
}

macro_rules! config_value_or_default {
    ($self:ident, $name:ident) => {{
        for config in $self.config_iter() {
            match config.$name() {
                Ok(None) => {
                    //Config didn't contain key, keep searching for config having entry
                }
                Ok(Some(value)) => {
                    return Ok(value);
                }
                Err(e) => {
                    //Config contained key, but value has errornous type
                    //Notify user about it
                    return Err((config_defaults::$name(), e));
                }
            }
        }
        //No config contained the key, return default value
        Ok(config_defaults::$name())
    }};
}

impl ConfigValueOrDefault for ConfigHolder {
    fn history_path_or_default(&self) -> Result<Option<PathBuf>, (Option<PathBuf>, ShellError)> {
        for config in self.config_iter() {
            match config.history_path() {
                Ok(None) => {
                    //Config didn't contain key, keep searching for config having entry
                }
                Ok(Some(path)) => return Ok(Some(path)),
                Err(e) => {
                    //Config contained key, but value has errornous type
                    //Notify user about it
                    return match config_defaults::history_path() {
                        //Return value from defaults with error from config
                        Ok(v) => Err((v, e)),
                        Err((v, _)) => Err((v, e)),
                    };
                }
            }
        }
        //No config contained the key, return default value
        config_defaults::history_path()
    }

    fn skip_welcome_message_or_default(&self) -> Result<bool, (bool, ShellError)> {
        config_value_or_default!(self, skip_welcome_message)
    }

    fn prompt_or_default(&self) -> Result<Value, (Value, ShellError)> {
        config_value_or_default!(self, prompt)
    }
}
