
pub use self::{commands::*, primitives::*, utils::*};

pub(crate) mod commands;

use scsys::prelude::BoxResult;

fn main() -> BoxResult {
    println!("Welcome to xtask");

    let _bundler = Bundler::new();
    Ok(())

}

use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct Bundler {
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub debug: bool,
    #[arg(action = clap::ArgAction::SetTrue, long)]
    pub desktop: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub release: bool,
    #[arg(action = clap::ArgAction::SetTrue, long, short)]
    pub update: bool,
}

impl Bundler {
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for Bundler {
    fn default() -> Self {
        Self::new()
    }
}


pub fn program(program: &str) -> scsys::BoxResult {
    
    let args = wild::args().collect::<Vec<_>>();
    std::process::Command::new(program).current_dir(project_root()).args(args.clone().as_slice()).status()?;
    println!("The args are: {:?}", args);
    Ok(())
}

pub(crate) mod primitives {
    pub type Bundle<T = &'static str> = std::collections::HashMap<T, Vec<Vec<T>>>;
}

pub(crate) mod utils {
    use scsys::prelude::BoxResult;
    use std::path::{Path, PathBuf};
    use std::{collections::HashMap, fs, io, process::Command};

    ///
    pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }

    ///
    pub fn dist_dir() -> PathBuf {
        project_root().join(".artifacts/dist")
    }

    ///
    pub fn execute_bundle(bundle: super::Bundle) -> BoxResult {
        for k in bundle.keys() {
            // Step 1: Rustup
            for i in 0..bundle[k].len() {
                let mut cmd = Command::new(k);
                cmd.current_dir(project_root());
                cmd.args(bundle[k][i].clone().as_slice()).status()?;
            }
        }
        Ok(())
    }

    /// Fetch the project root unless specified otherwise with a CARGO_MANIFEST_DIR env variable
    pub fn project_root() -> PathBuf {
        Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(1)
            .unwrap()
            .to_path_buf()
    }

}