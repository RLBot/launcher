use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub digest: String,
    pub browser_download_url: String,
}

// Example: https://api.github.com/repos/swz-git/guilauncher/releases/latest
#[derive(Debug, Deserialize)]
pub struct Release {
    pub name: String,
    pub assets: Vec<Asset>,
}
