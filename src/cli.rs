use crate::*;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short='p', default_value = "14141")]
	pub port: u16,

	#[arg(long, short='h', default_value = "0.0.0.0")]
	pub host: String,

	#[arg(long, short='k')]
	pub key: Option<String>,
}
