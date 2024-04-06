use super::err::SystemError;
use crate::Result;
use std::path::PathBuf;

pub fn cwd() -> Result<PathBuf> {
	Ok(std::env::current_dir().map_err(|_| SystemError::EnvVar("CWD".to_string()))?)
}

pub fn home() -> Result<String> {
	Ok(std::env::var("HOME").map_err(|_| SystemError::EnvVar("HOME".to_string()))?)
}

pub fn xdg_home() -> Result<String> {
	Ok(std::env::var("XDG_CONFIG_HOME").map_err(|_| SystemError::EnvVar("XDG_CONFIG_HOME".to_string()))?)
}

pub fn editor() -> Result<String> {
	Ok(std::env::var("EDITOR").map_err(|_| SystemError::EnvVar("EDITOR".to_string()))?)
}
