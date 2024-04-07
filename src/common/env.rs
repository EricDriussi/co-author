use super::err::SystemError;
use crate::Result;
use std::path::PathBuf;

pub fn cwd() -> Result<PathBuf> {
	Ok(std::env::current_dir().map_err(|_| SystemError::EnvVar("CWD".to_string()))?)
}

pub fn home() -> Result<String> {
	let home = "HOME";
	Ok(std::env::var(home).map_err(|_| SystemError::EnvVar(home.to_string()))?)
}

pub fn xdg_home() -> Result<String> {
	let xdg_home = "XDG_CONFIG_HOME";
	Ok(std::env::var(xdg_home).map_err(|_| SystemError::EnvVar(xdg_home.to_string()))?)
}

pub fn editor() -> Result<String> {
	let editor = "EDITOR";
	Ok(std::env::var(editor).map_err(|_| SystemError::EnvVar(editor.to_string()))?)
}
