use std::{
    collections::HashMap,
    path::Path,
    str::FromStr,
    task::{Context, Poll},
};

use assets_base::AssetsIndex;
use axum::http::{Request, Uri};
use base::hash::fmt_hash;
use tower_http::services::ServeDir;
use tower_service::Service;

#[derive(Debug, Clone)]
pub struct IndexDir {
    index: HashMap<String, String>,
    serve: ServeDir,
}

impl IndexDir {
    pub async fn new<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let index = tokio::fs::read(path.as_ref().join("index.json")).await?;
        let index: AssetsIndex = serde_json::from_slice(&index)?;

        let index = index
            .into_iter()
            .map(|(name, entry)| {
                let full_path = format!("{name}_{}.{}", fmt_hash(&entry.hash), entry.ty);
                (full_path, format!("{name}.{}", entry.ty))
            })
            .collect();

        Ok(Self {
            index,
            serve: ServeDir::new(path),
        })
    }
}

impl<ReqBody: 'static + Send> Service<Request<ReqBody>> for IndexDir {
    type Response = <ServeDir as Service<Request<ReqBody>>>::Response;
    type Error = <ServeDir as Service<Request<ReqBody>>>::Error;
    type Future = <ServeDir as Service<Request<ReqBody>>>::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        <ServeDir as Service<Request<ReqBody>>>::poll_ready(&mut self.serve, cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let uri = req.uri().clone();
        let path = uri.path();
        let file_path = urlencoding::decode(path)
            .map(|s| {
                let s: &str = &s;
                let s: &Path = s.as_ref();
                s.to_path_buf()
            })
            .ok();

        *req.uri_mut() = if let Some((name, mut parent)) = file_path
            .as_ref()
            .and_then(|file_path| {
                file_path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .zip(file_path.parent().map(|p| {
                        p.components()
                            .filter_map(|c| match c {
                                std::path::Component::Prefix(_)
                                | std::path::Component::RootDir
                                | std::path::Component::CurDir
                                | std::path::Component::ParentDir => None,
                                std::path::Component::Normal(path) => path.to_str(),
                            })
                            .map(|s| urlencoding::encode(s).to_string())
                            .collect::<Vec<_>>()
                            .join("/")
                    }))
            })
            .and_then(|(name, parent)| {
                self.index
                    .get(name)
                    .map(|name| urlencoding::encode(name))
                    .zip(Some(parent))
            }) {
            if !parent.is_empty() {
                parent.push('/');
            }
            Uri::from_str(&format!("http://localhost/{}{}", parent, name)).unwrap_or(uri)
        } else {
            uri
        };
        self.serve.call(req)
    }
}
