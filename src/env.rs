use std::env::{self, VarError};

pub fn his() -> Result<String, VarError> {
  env::var("HYPRLAND_INSTANCE_SIGNATURE")
}

pub fn xdg_runtime_dir() -> Result<String, VarError> {
  env::var("XDG_RUNTIME_DIR")
}
