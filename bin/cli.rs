use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short='p', default_value = "14141")]
	pub port: u16,

	#[arg(long, short='h', default_value = "0.0.0.0")]
	pub host: String,

	#[arg(long, short='r', default_value = "https://api.mainnet-beta.solana.com")]
	pub rpc_endpoint: String,

	#[arg(long, short='j', default_value = "https://ny.mainnet.block-engine.jito.wtf")]
	pub jito_endpoint: String,
}
