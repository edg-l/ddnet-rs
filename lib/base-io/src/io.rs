use std::sync::Arc;

use base_io_traits::{fs_traits::FileSystemInterface, http_traits::HttpClientInterface};
use hiarc::Hiarc;

use crate::runtime::IoRuntime;

#[derive(Debug, Hiarc, Clone)]
pub struct IoFileSys {
    #[hiarc_skip_unsafe]
    pub fs: Arc<dyn FileSystemInterface>,
    #[doc(alias = "runtime")]
    #[doc(alias = "batcher")]
    #[doc(alias = "tasks")]
    #[doc(alias = "spawner")]
    pub rt: IoRuntime,
}

impl From<Io> for IoFileSys {
    fn from(value: Io) -> Self {
        Self {
            fs: value.fs,
            rt: value.rt,
        }
    }
}

impl From<&Io> for IoFileSys {
    fn from(value: &Io) -> Self {
        Self {
            fs: value.fs.clone(),
            rt: value.rt.clone(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub type AsyncRuntime<'a> = tokio::runtime::Runtime;
#[cfg(target_arch = "wasm32")]
pub type AsyncRuntime<'a> = async_executor::LocalExecutor<'a>;

pub fn create_runtime() -> AsyncRuntime<'static> {
    // tokio runtime for client side tasks
    #[cfg(not(target_arch = "wasm32"))]
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4) // should be at least 4
        .enable_time()
        .enable_io()
        .build()
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    let rt = async_executor::LocalExecutor::new();

    rt
}

impl IoFileSys {
    pub fn new(fs_builder: impl FnOnce(&AsyncRuntime) -> Arc<dyn FileSystemInterface>) -> Self {
        let async_rt = create_runtime();

        Self {
            fs: fs_builder(&async_rt),
            rt: IoRuntime::new(async_rt),
        }
    }
}

#[derive(Debug, Hiarc, Clone)]
pub struct Io {
    #[hiarc_skip_unsafe]
    pub fs: Arc<dyn FileSystemInterface>,
    #[hiarc_skip_unsafe]
    pub http: Arc<dyn HttpClientInterface>,
    #[doc(alias = "runtime")]
    #[doc(alias = "batcher")]
    #[doc(alias = "tasks")]
    #[doc(alias = "spawner")]
    pub rt: IoRuntime,
}

impl Io {
    pub fn new(
        fs_builder: impl FnOnce(&AsyncRuntime) -> Arc<dyn FileSystemInterface>,
        http: Arc<dyn HttpClientInterface>,
    ) -> Self {
        let io_fs = IoFileSys::new(fs_builder);

        Self {
            fs: io_fs.fs,
            http,
            rt: io_fs.rt,
        }
    }

    pub fn from(io: IoFileSys, http: Arc<dyn HttpClientInterface>) -> Self {
        Self {
            fs: io.fs,
            http,
            rt: io.rt,
        }
    }
}
