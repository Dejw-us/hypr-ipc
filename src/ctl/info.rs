use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Version {
  pub branch: String,
  pub commit: String,
  pub version: String,
  pub dirty: bool,
  pub commit_message: String,
  pub commit_date: String,
  pub tag: String,
  pub commits: String,
  #[serde(rename = "buildAquamarine")]
  pub build_aquamarine: String,
  #[serde(rename = "buildHyprlang")]
  pub build_hyprlang: String,
  #[serde(rename = "buildHyprutils")]
  pub build_hyprutils: String,
  #[serde(rename = "buildHyprcursor")]
  pub build_hyprcursor: String,
  #[serde(rename = "buildHyprgraphics")]
  pub build_hyprgraphics: String,
  pub flags: Vec<String>,
}

pub type Monitors = Vec<Monitor>;

#[derive(Deserialize, Debug)]
pub struct Monitor {
  pub id: i16,
  pub name: String,
  pub description: String,
  pub make: String,
  pub model: String,
  pub serial: String,
  pub width: u32,
  pub height: u32,
  #[serde(rename = "refreshRate")]
  pub refresh_rate: f32,
  pub x: i16,
  pub y: i16,
  #[serde(rename = "activeWorkspace")]
  pub active_workspace: WorkspaceInfo,
  #[serde(rename = "specialWorkspace")]
  pub special_workspace: WorkspaceInfo,
  pub reserved: Vec<u16>,
  pub scale: f32,
  pub transform: i16,
  pub focused: bool,
  #[serde(rename = "dpmsStatus")]
  pub dpms_status: bool,
  pub vrr: bool,
  pub solitary: String,
  #[serde(rename = "activelyTearing")]
  pub actively_tearing: bool,
  #[serde(rename = "directScanoutTo")]
  pub direct_scanout_to: String,
  pub disabled: bool,
  #[serde(rename = "currentFormat")]
  pub current_format: String,
  #[serde(rename = "mirrorOf")]
  pub mirror_of: String,
  #[serde(rename = "availableModes")]
  pub available_modes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct WorkspaceInfo {
  pub id: i16,
  pub name: String,
}

pub type Workspaces = Vec<Workspace>;

#[derive(Deserialize, Debug)]
pub struct Workspace {
  pub id: i16,
  pub name: String,
  #[serde(rename = "monitorID")]
  pub monitor_id: i16,
  pub windows: i16,
  #[serde(rename = "hasfullscreen")]
  pub has_fullscreen: bool,
  #[serde(rename = "lastwindow")]
  pub last_window: String,
  #[serde(rename = "lastwindowtitle")]
  pub last_window_title: String,
  #[serde(rename = "ispersistent")]
  pub is_persistent: bool,
}
