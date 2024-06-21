pub use std::{collections::HashSet, hash::Hash};
pub use serde::{Serialize, Deserialize};
pub use itertools::Itertools;

pub use cached::proc_macro::io_cached;

pub use clap::{Parser, Args, Subcommand, ValueEnum};

mod cli;
use cli::*;

pub use jsonrpsee::{
	core::async_trait,
	proc_macros::rpc,
	server::Server,
	types::ErrorObjectOwned,
    http_client::HttpClientBuilder
};

#[rpc(server, client)]
pub trait Rpc {
	#[method(name = "get_trending")]
	async fn send_jito(
		&self,
	) -> Result<(), ErrorObjectOwned>;

    #[method(name = "update")]
    async fn send_rpc(
        &self,
    ) -> Result<(), ErrorObjectOwned>;
}


#[derive(Default)]
pub struct RpcServerImpl {}

#[async_trait]
impl RpcServer for RpcServerImpl {
    async fn send_jito(&self) -> Result<(), ErrorObjectOwned> {
        sune::send_jito();
        Ok(())
    }

    async fn send_rpc(&self) -> Result<(), ErrorObjectOwned> {
        sune::send_rpc();
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = Cli::parse();

	let server = Server::builder().build(format!("{}:{}", args.host, args.port)).await?;

	let addr = server.local_addr()?;

    let url = format!("http://{}", addr);
	println!("Server is running at: {}", url);

	let handle = server.start(RpcServerImpl::default().into_rpc());
    handle.stopped().await;
    Ok(())
}

