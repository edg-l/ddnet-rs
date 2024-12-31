pub mod index_dir;

use std::net::SocketAddr;

use anyhow::anyhow;
use assets_base::AssetsIndex;
use axum::Router;
use base::hash::{fmt_hash, generate_hash_for};
use clap::{command, Parser};
use index_dir::IndexDir;
use tower_http::trace::TraceLayer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The port this server should listen on
    #[arg(short, long)]
    port: Option<u16>,
    /// Don't use any cached entry, basically forcing them to recreate them.
    #[arg(short, long)]
    no_cache: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let port = args.port.unwrap_or(3002);
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    let app = skins(args.no_cache)
        .await?
        .merge(entities(args.no_cache).await?)
        .merge(ctfs(args.no_cache).await?)
        .merge(emoticons(args.no_cache).await?)
        .merge(flags(args.no_cache).await?)
        .merge(freezes(args.no_cache).await?)
        .merge(games(args.no_cache).await?)
        .merge(hooks(args.no_cache).await?)
        .merge(huds(args.no_cache).await?)
        .merge(ninjas(args.no_cache).await?)
        .merge(particles(args.no_cache).await?)
        .merge(weapons(args.no_cache).await?);
    axum::serve(listener, app.layer(TraceLayer::new_for_http())).await?;

    Ok(())
}

async fn assets_generic(base_path: &str, ignore_cached: bool) -> anyhow::Result<Router> {
    // make sure there is an index file for this path
    let index_path = format!("{base_path}/index.json");
    if !tokio::fs::try_exists(&index_path).await.unwrap_or_default() || ignore_cached {
        let index = prepare_index_generic(base_path).await?;

        tokio::fs::write(index_path, serde_json::to_vec(&index)?).await?;
    }

    Ok(Router::new().nest_service(&format!("/{base_path}"), IndexDir::new(base_path).await?))
}

async fn skins(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("skins", ignore_cached).await
}

async fn entities(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("entities", ignore_cached).await
}

async fn ctfs(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("ctfs", ignore_cached).await
}

async fn emoticons(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("emoticons", ignore_cached).await
}

async fn flags(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("flags", ignore_cached).await
}

async fn freezes(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("freezes", ignore_cached).await
}

async fn games(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("games", ignore_cached).await
}

async fn hooks(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("hooks", ignore_cached).await
}

async fn huds(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("huds", ignore_cached).await
}

async fn ninjas(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("ninjas", ignore_cached).await
}

async fn particles(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("particles", ignore_cached).await
}

async fn weapons(ignore_cached: bool) -> anyhow::Result<Router> {
    assets_generic("weapons", ignore_cached).await
}

async fn prepare_index_generic(base_path: &str) -> anyhow::Result<AssetsIndex> {
    let mut res: AssetsIndex = Default::default();

    let mut files = tokio::fs::read_dir(base_path)
        .await
        .map_err(|err| anyhow!("can't dir find {base_path:?}: {err}"))?;

    while let Some(file) = files.next_entry().await? {
        anyhow::ensure!(
            file.metadata().await?.is_file(),
            "only files are allowed as assets files currently."
        );
        let path = file.path();

        // ignore all json files for now
        if path.extension().is_some_and(|ext| ext.eq("json")) {
            continue;
        }

        let file_name = path
            .file_stem()
            .ok_or_else(|| anyhow!("Only files with proper names are allowed"))?
            .to_string_lossy()
            .to_string();
        let file_ext = path
            .extension()
            .ok_or_else(|| anyhow!("Files need proper file endings"))?
            .to_string_lossy()
            .to_string();

        let file = tokio::fs::read(&path)
            .await
            .map_err(|err| anyhow!("can't find {path:?}: {err}"))?;

        let hash = generate_hash_for(&file);

        anyhow::ensure!(
            !file_name.ends_with(&format!("_{}", fmt_hash(&hash))),
            "Only files without their hashes are allowed."
        );

        res.insert(
            file_name,
            assets_base::AssetIndexEntry {
                ty: file_ext,
                hash,
                size: file.len() as u64,
            },
        );
    }

    Ok(res)
}
