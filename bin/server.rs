pub use jsonrpsee::{
	core::async_trait,
	proc_macros::rpc,
	server::Server,
	types::ErrorObjectOwned,
};
use sune::{SendJitoParams, SendRpcParams};
use crate::types::*;

#[rpc(server, client)]
pub trait Rpc {
	#[method(name = "send_jito")]
	async fn send_jito(
		&self,
        params: SendJitoParams
	) -> Result<(), ErrorObjectOwned>;

    #[method(name = "send_rpc")]
    async fn send_rpc(
        &self,
        params: SendRpcParams
    ) -> Result<(), ErrorObjectOwned>;


    // Solana compatability

    #[method(name = "sendTransaction")]
    async fn send_transaction(
        &self,
        tx: String
    ) -> Result<String, ErrorObjectOwned>;

    #[method(name = "getVersion")]
    async fn get_version(&self) -> Result<GetVersionResponse, ErrorObjectOwned>;

    #[method(name = "getLatestBlockhash")]
    async fn get_latest_blockhash(&self) -> Result<SolanaCV<SlotContext, GetLatestBlockhashResponseValue>, ErrorObjectOwned>;

    #[method(name = "getBalance")]
    async fn get_balance(&self, address: String) -> Result<SolanaCV<SlotContext, u64>, ErrorObjectOwned>;

    #[method(name = "getFeeForMessage")]
    async fn get_fee_for_message(&self, message: String) -> Result<SolanaCV<SlotContext, Option<u64>>, ErrorObjectOwned>;

    #[method(name = "getSignatureStatuses")]
    // We use serde_json::Value because honestly I'm too lazy to properly serialize this type, and it works anyways lol
    // maybe look into returning serde_json::Value for everything? later though
    async fn get_signature_statuses(&self, signatures: Vec<String>) -> Result<serde_json::Value, ErrorObjectOwned>;

    #[method(name = "isBlockhashValid")]
    async fn is_blockhash_valid(&self, blockhash: String) -> Result<SolanaCV<SlotContext, bool>, ErrorObjectOwned>;

}


macro_rules! err {
    ($obj:expr, $msg:expr) => {
        if $obj.is_err() {
            log::error!("{} | {}", $msg, $obj.as_ref().err().unwrap());
            return Err(ErrorObjectOwned::owned(0, $msg.to_string(), Some($obj.err().unwrap().to_string())));
        }
    };
}



#[derive(Default, Debug)]
pub struct RpcServerImpl {
    pub rpc: String,
    pub block_engine: String
}

#[async_trait]
impl RpcServer for RpcServerImpl {
    async fn send_jito(&self, params: SendJitoParams) -> Result<(), ErrorObjectOwned> {
        let req = reqwest::Client::new().post(&format!("{}/api/v1/transactions", self.block_engine))
            .query(&[
                ("bundleOnly", "true")
            ])
            .json(&SolanaRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: "1".to_string(),
                method: "sendTransaction".to_string(),
                params: vec![serde_json::to_value(params.transaction).unwrap()]
            })
            .send()
            .await;

        err!(req, "Error sending transaction with Jito");

        Ok(())
    }

    async fn send_rpc(&self, params: SendRpcParams) -> Result<(), ErrorObjectOwned> {
        let req = reqwest::Client::new().post(&self.rpc)
            .json(&SolanaRpcRequest::send_transaction(params.transaction))
            .send()
            .await;


        err!(req, "Error sending transaction");

        Ok(())
    }

    async fn send_transaction(&self, tx: String) -> Result<String, ErrorObjectOwned> {
        let req = reqwest::Client::new().post(&self.rpc)
            .json(&SolanaRpcRequest::send_transaction(tx))
            .send()
            .await;

        err!(req, "Error sending transaction");

        let res = req.unwrap().json::<RpcResponse<String>>().await;

        err!(res, "Error sending transaction");

        Ok(res.unwrap().result)
    }

    async fn get_version(&self) -> Result<GetVersionResponse, ErrorObjectOwned> {
        let req = req(&self.rpc, SolanaRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getVersion".to_string(),
            params: vec![]
        }).await;

        err!(req, "Error getting version");

        let res = req.unwrap().json::<RpcResponse<GetVersionResponse>>().await;

        err!(res, "Error getting version");

        Ok(res.unwrap().result)
    }

    async fn get_latest_blockhash(&self) -> Result<SolanaCV<SlotContext, GetLatestBlockhashResponseValue>, ErrorObjectOwned> {
        let req = req(&self.rpc, SolanaRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getLatestBlockhash".to_string(),
            params: vec![]
        }).await;

        if req.is_err() {
            return Err(ErrorObjectOwned::owned(0, "Error getting latest blockhash".to_string(), Some(req.err().unwrap().to_string())));
        }

        err!(req, "Error getting latest blockhash");

        let res = req.unwrap().json::<RpcResponse<SolanaCV<SlotContext, GetLatestBlockhashResponseValue>>>().await;

        err!(res, "Error getting latest blockhash");

        Ok(res.unwrap().result)
    }

    async fn get_balance(&self, address: String) -> Result<SolanaCV<SlotContext, u64>, ErrorObjectOwned> {
        let req = req(&self.rpc, SolanaRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getBalance".to_string(),
            params: vec![serde_json::to_value(address).unwrap()]
        }).await;


        err!(req, "Error getting balance");

        let res = req.unwrap().json::<RpcResponse<SolanaCV<SlotContext, u64>>>().await;

        err!(res, "Error getting balance");

        Ok(res.unwrap().result)
    }

    async fn get_fee_for_message(&self, message: String) -> Result<SolanaCV<SlotContext, Option<u64>>, ErrorObjectOwned> {
        let req = req(&self.rpc, SolanaRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getFeeForMessage".to_string(),
            params: vec![serde_json::to_value(message).unwrap()]
        }).await;

        err!(req, "Error getting fee for message");

        let res = req.unwrap().json::<RpcResponse<SolanaCV<SlotContext, Option<u64>>>>().await;

        err!(res, "Error getting fee for message");

        Ok(res.unwrap().result)
    }

    async fn get_signature_statuses(&self, signatures: Vec<String>) -> Result<serde_json::Value, ErrorObjectOwned> {
        let req = req(&self.rpc, SolanaRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "getSignatureStatuses".to_string(),
            params: vec![serde_json::to_value(signatures).unwrap()]
        }).await;

        err!(req, "Error getting signature statuses");

        let res = req.unwrap().json::<RpcResponse<serde_json::Value>>().await;

        err!(res, "Error getting signature statuses");

        return Ok(res.unwrap().result);
    }

    async fn is_blockhash_valid(&self, blockhash: String) -> Result<SolanaCV<SlotContext, bool>, ErrorObjectOwned> {
        let req = req(&self.rpc, SolanaRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "isBlockhashValid".to_string(),
            params: vec![serde_json::to_value(blockhash).unwrap()]
        }).await;

        err!(req, "Error checking blockhash");

        let res = req.unwrap().json::<RpcResponse<SolanaCV<SlotContext, bool>>>().await;

        err!(res, "Error checking blockhash");

        Ok(res.unwrap().result)
    }
}

async fn req(url: &String, req: SolanaRpcRequest) -> Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new().post(url)
        .json(&req)
        .send()
        .await
}