astrolib::mu!(cli, server, types);

use clap::Parser;
use jsonrpsee::server::{middleware::rpc::RpcServiceT, RpcServiceBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Cli::parse();

    if args.rpc_endpoint == "https://api.mainnet-beta.solana.com" {
        log::info!("Using default RPC endpoint: {}", args.rpc_endpoint);
    }

    if args.jito_endpoint == "https://ny.mainnet.block-engine.jito.wtf" {
        log::info!("Using default Jito endpoint: {}", args.jito_endpoint);
    }

    let server = Server::builder()
        .set_rpc_middleware(RpcServiceBuilder::new().layer_fn(Logger))
        .build(format!("{}:{}", args.host, args.port))
        .await?;

    log::info!("Server is running at: http://{}", server.local_addr()?);

    server
        .start(
            RpcServerImpl {
                rpc: args.rpc_endpoint,
                block_engine: args.jito_endpoint,
            }
            .into_rpc(),
        )
        .stopped()
        .await;

    Ok(())
}

#[derive(Clone)]
pub struct Logger<S>(S);

impl<'a, S> RpcServiceT<'a> for Logger<S>
where
    S: RpcServiceT<'a> + Send + Sync,
{
    type Future = S::Future;

    fn call(&self, req: jsonrpsee::types::Request<'a>) -> Self::Future {
        log::debug!("Method called: `{}`", req.method);
        self.0.call(req)
    }
}
