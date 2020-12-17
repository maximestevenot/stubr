use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::Duration;

use async_trait::async_trait;
use tempfile::tempdir;

use stubr::server::StubServer;

pub struct Wiremock {
    pub process: Child,
    pub port: String,
}

static PORT: AtomicUsize = AtomicUsize::new(50_000);

fn port() -> String {
    PORT.fetch_add(1, Ordering::SeqCst).to_string()
}

#[async_trait]
impl StubServer for Wiremock {
    async fn register_stubs(&self, _stub_folder: PathBuf) -> anyhow::Result<()> { unimplemented!() }

    fn uri(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}

impl Wiremock {
    pub fn start(stub_file: PathBuf) -> Self {
        let port = port();
        let stub = Self::tmp_stub(stub_file);
        let process = Command::new("/usr/local/bin/wiremock")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .args(&[
                "--port", port.as_str(),
                "--disable-banner",
                "--root-dir", stub.to_str().unwrap()
            ])
            .spawn().unwrap();
        // give some time to java process & server to spin up
        sleep(Duration::from_secs(5));
        Self { process, port }
    }

    fn tmp_stub(stub_file: PathBuf) -> PathBuf {
        let root = tempdir().unwrap().into_path();
        let mappings = root.join("mappings");
        fs::create_dir(&mappings).unwrap();
        let stub = mappings.join("stub.json");
        File::create(&stub).unwrap();
        fs::copy(stub_file, &stub).unwrap();
        root
    }
}

impl Drop for Wiremock {
    fn drop(&mut self) {
        self.process.kill().unwrap();
    }
}