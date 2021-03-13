use std::path::PathBuf;

use nu_errors::ShellError;
use nu_protocol::Value;

use super::user_data_dir;

///Trait providing Ok(value), where value might be a default value or user provided
///or Err((default_value, err)) (err might come from wrong config value format...)
//The rational here is to always provide a value and not throw away the error
pub trait ConfigValueOrDefault {
    fn history_path_or_default(&self) -> Result<Option<PathBuf>, (Option<PathBuf>, ShellError)>;
    fn skip_welcome_message_or_default(&self) -> Result<bool, (bool, ShellError)>;
    fn prompt_or_default(&self) -> Result<Value, (Value, ShellError)>;
}

//History path is a little special because the default value can throw an error by itself
pub fn history_path() -> Result<Option<PathBuf>, (Option<PathBuf>, ShellError)> {
    let mut dir = user_data_dir().map_err(|e| (None, e))?;
    dir.push("history.txt");
    Ok(Some(dir))
}

pub fn skip_welcome_message() -> bool {
    false
}

pub fn prompt() -> Value {
    Value::nothing()
}
