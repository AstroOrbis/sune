use crate::*;
use reqwest::Client;


pub struct Kitsune {
	pub client: Client,
	pub sune_url: String,
}

impl Kitsune {
	pub fn new(url: String) -> Self {
		Self {
			client: Client::new(),
			sune_url: url,
		}
	}

	pub async fn post(&self, req: SuneRpcRequest) -> Result<String, anyhow::Error> {
		let res = self.client.post(&self.sune_url)
			.json(&req)
			.send()
			.await?;

		Ok(res.text().await?)
	}

	pub async fn send_rpc(&self, params: SendRpcParams) -> Result<String, anyhow::Error> {
		let req = SuneRpcRequest::from(params);
		self.post(req).await
	}

	pub async fn send_jito(&self, params: SendJitoParams) -> Result<String, anyhow::Error> {
		let req = SuneRpcRequest::from(params);
		self.post(req).await
	}
}

