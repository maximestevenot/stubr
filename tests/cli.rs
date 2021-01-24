use surf::{get, post};

use crate::{cli::StubrCli, utils::*};

mod utils;

#[async_std::test]
async fn should_serve_stubs_under_dir() {
    let stubr = StubrCli::new(&["tests/stubs/cli"]);
    get(&stubr.addr).await.unwrap().assert_ok();
    post(&stubr.addr).await.unwrap().assert_not_found();
}

#[async_std::test]
async fn should_serve_stubs_under_root_dir() {
    let stubr = StubrCli::new(&["--root-dir", "tests/stubs/cli"]);
    post(&stubr.addr).await.unwrap().assert_ok();
    get(&stubr.addr).await.unwrap().assert_not_found();
}