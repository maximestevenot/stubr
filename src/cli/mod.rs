use std::{env::current_dir, ffi::OsStr, fs::DirEntry, path::PathBuf};

use clap::{AppSettings, Clap, ValueHint};

use commands::Commands;
use stubr::{Config, Stubr};

mod completion;
mod commands;

/// A Rust implementation of Wiremock
#[derive(Clap, Debug, Default)]
#[clap(
name = "stubr",
version = "0.0.1",
setting = AppSettings::GlobalVersion,
)]
pub struct Cli {
    /// stub files directory
    ///
    /// Wiremock stub files are json files.
    /// Defaults to current directory when not present
    #[clap(parse(from_os_str), value_hint = ValueHint::DirPath)]
    dir: Option<PathBuf>,
    /// equivalent of 'root-dir' option in Wiremock cli
    ///
    /// Expects a 'mappings' folder under this directory which contains stub files
    #[clap(long = "root-dir", parse(from_os_str), value_hint = ValueHint::DirPath)]
    root_dir: Option<PathBuf>,
    /// port number the server is listening on
    ///
    /// When absent, defaults to a random one
    #[clap(short = 'p', long = "port")]
    port: Option<u16>,
    #[clap(subcommand)]
    cmd: Option<Commands>,
}

impl Cli {
    const MAPPINGS_FOLDER: &'static str = "mappings";

    // Runs stubr forever until process exits
    pub async fn run(&self) -> anyhow::Result<()> {
        if let Some(cmd) = self.cmd.as_ref() {
            cmd.exec()
        } else {
            Stubr::run(self.stubs_dir(), self.into()).await
        }
    }

    fn stubs_dir(&self) -> PathBuf {
        self.root_dir()
            .or_else(|| self.dir())
            .expect("Could not find stub directory")
    }

    fn dir(&self) -> Option<PathBuf> {
        current_dir().ok()
            .and_then(|current| {
                self.dir.as_ref()
                    .map(|d| current.join(d))
                    .or(Some(current))
            })
    }

    fn root_dir(&self) -> Option<PathBuf> {
        current_dir().ok()
            .and_then(|current| {
                self.root_dir.as_ref()
                    .filter(|&it| Self::does_contains_mappings_folder(it))
                    .map(|it| it.join(Self::MAPPINGS_FOLDER))
                    .map(|it| current.join(it))
            })
    }

    fn does_contains_mappings_folder(input: &PathBuf) -> bool {
        input.read_dir().ok().as_mut()
            .map(|all| all.any(|child| child.map(Self::is_mappings_folder).unwrap_or_default()))
            .unwrap_or_default()
    }

    fn is_mappings_folder(folder: DirEntry) -> bool {
        let path = folder.path();
        path.is_dir() && path.file_name() == Some(OsStr::new(Self::MAPPINGS_FOLDER))
    }
}

impl From<&Cli> for Config {
    fn from(cli: &Cli) -> Self {
        Self { port: cli.port }
    }
}

#[cfg(test)]
mod cli_test {
    use std::{
        env::current_dir,
        path::PathBuf,
    };

    use crate::cli::Cli;

    #[test]
    fn stubs_dir_should_append_dir_to_current_dir() {
        let dir = PathBuf::from("tests/stubs/cli");
        let cli = Cli { dir: Some(dir.clone()), ..Default::default() };
        assert_eq!(cli.stubs_dir(), current_dir().unwrap().join(dir))
    }

    #[test]
    fn stubs_dir_should_default_to_current_dir() {
        let cli = Cli { dir: None, ..Default::default() };
        assert_eq!(cli.stubs_dir(), current_dir().unwrap())
    }

    #[test]
    fn root_dir_should_default_to_none_when_not_provided() {
        let cli = Cli { root_dir: None, ..Default::default() };
        assert!(cli.root_dir().is_none())
    }

    #[test]
    fn root_dir_should_be_appended_to_current_dir() {
        let root_dir = PathBuf::from("tests/stubs/cli");
        let cli = Cli { root_dir: Some(root_dir.clone()), ..Default::default() };
        assert_eq!(cli.root_dir().unwrap(), current_dir().unwrap().join(root_dir.join("mappings")))
    }

    #[test]
    fn root_dir_should_have_precedence_over_dir() {
        let dir = PathBuf::from("tests/stubs");
        let root_dir = PathBuf::from("tests/stubs/cli");
        let cli = Cli { dir: Some(dir), root_dir: Some(root_dir.clone()), ..Default::default() };
        assert_eq!(cli.stubs_dir(), current_dir().unwrap().join(root_dir.join("mappings")))
    }
}