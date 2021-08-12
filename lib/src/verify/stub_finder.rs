use std::{
    env::current_dir,
    ffi::OsString,
    fs::OpenOptions,
    path::PathBuf,
};

use super::super::model::JsonStub;

pub(crate) struct ProducerStubFinder;

impl ProducerStubFinder {
    pub(crate) fn find_stubs() -> Vec<(JsonStub, OsString)> {
        Self::stub_dir()
            .and_then(|it| it.read_dir().ok())
            .map(|dir| dir.map(|it| it.unwrap().path()).collect())
            .map(|it| Self::map_json_stub(it))
            .unwrap_or_default()
    }

    fn map_json_stub(files: Vec<PathBuf>) -> Vec<(JsonStub, OsString)> {
        files.iter()
            .filter_map(|path| OpenOptions::new().read(true).open(path).ok().zip(path.file_name()))
            .filter_map(|(file, name)| serde_json::from_reader(file).ok().zip(Some(name.to_os_string())))
            .collect()
    }

    fn stub_dir() -> Option<PathBuf> {
        current_dir().map(|it| it.join("stubs")).ok()
    }
}