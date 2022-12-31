/*
    Appellation: workspace <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workspace {
    pub artifacts: PathBuf,
    pub root: PathBuf,
    
}

impl Workspace {
    pub fn new(artifacts: Option<PathBuf>) -> Self {
        let root = crate::project_root();
        let artifacts = artifacts.unwrap_or_else(|| root.clone().join(".artifacts"));
        Self { artifacts, root }
    }
    ///
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
    ///
    pub fn update_root(&mut self, path: PathBuf) -> &Self {
        self.root = path;
        self
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new(None)
    }
}
