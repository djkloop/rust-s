use std::io::Error as IoError;
use std::path::Path;
use std::sync::Arc;

use async_std::{fs::OpenOptions, io};
use tempfile::TempDir;
use tide::{convert::json, Body, Request, Response, StatusCode};
use tide_fluent_routes::{
    root,
    routebuilder::{RouteBuilder, RouteBuilderExt},
    router::Router,
};

#[derive(Clone)]
pub struct TempDirState {
    tempdir: Arc<TempDir>,
}

impl TempDirState {
    fn try_new() -> Result<Self, IoError> {
        Ok(Self {
            tempdir: Arc::new(tempfile::tempdir()?),
        })
    }

    fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

#[async_std::main]
async fn main() -> Result<(), IoError> {
    tide::log::start();
    let mut server = tide::with_state(TempDirState::try_new()?);

    server.register(root().at("api/file/:file", |route| route.put(upload).get(download)));

    server.listen("127.0.0.1:8888").await?;

    Ok(())
}

pub async fn upload(req: Request<TempDirState>) -> tide::Result {
    let path = req.param("file")?;
    
    let fs_server_path = req.state().path().join(path);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(&fs_server_path)
        .await?;
    let bytes_written = io::copy(req, file).await?;

    Ok(json!({
        "bytes_written": bytes_written,
        "path": fs_server_path.canonicalize()?.to_str()
    })
    .into())
}

pub async fn download(req: Request<TempDirState>) -> tide::Result {
    // 获取文件
    let path = req.param("file")?;
    let fs_server_path = req.state().path().join(path);
    if let Ok(res) = Body::from_file(fs_server_path).await {
        Ok(res.into())
    } else {
        Ok(Response::new(StatusCode::NotFound))
    }
}
