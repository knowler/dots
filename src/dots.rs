use serde_derive::Deserialize;
use std::io::Result;
use std::path::PathBuf;
use yansi::Paint;
use git2::Repository;

#[derive(Deserialize, Debug)]
pub struct Dots {
    path: PathBuf,
    repo: String,
}

impl Dots {
    pub fn from_config(path: &PathBuf) -> Result<Self> {
        let raw = std::fs::read_to_string(&path)?;

        let config: Self = toml::from_str(raw.as_str())?;

        Ok(config)
    }

    pub fn init(&self) {
        let repo = match Repository::clone(&self.repo.as_str(), &self.path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    }

    pub fn install(&self) {
        println!("{}", Paint::magenta("Installing dotfiles...").italic());
    }

    pub fn update(&self) {
        println!("{}", Paint::magenta("Updating dotfiles...").italic());
    }
}

