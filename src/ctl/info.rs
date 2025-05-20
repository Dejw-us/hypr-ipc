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
