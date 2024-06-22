use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SolanaCV<C, V> {
    pub context: C,
    pub value: V
}

pub type SolanaSlotV<V> = SolanaCV<SlotContext, V>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlotContext {
    pub slot: u64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SingleValue<T> {
    pub value: T
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetVersionResponse {
    #[serde(rename = "feature-set")]
    pub feature_set: u32,
    #[serde(rename = "solana-core")]
    pub solana_core: String,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetLatestBlockhashResponseValue {
    pub blockhash: String,
    #[serde(rename = "lastValidBlockHeight")]
    pub last_valid_block_height: u64
}


#[derive(Serialize, Deserialize)]
pub struct SolanaSendTransactionParams {
    pub skip_preflight: bool,
	pub encoding: String
}

#[derive(Serialize, Deserialize)]
pub struct SolanaRpcRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Vec<serde_json::Value>
}

impl SolanaRpcRequest {
    pub fn send_transaction (transaction: String) -> Self {
        Self::send_tx_with_opts(transaction, SolanaSendTransactionParams {skip_preflight: true, encoding: "base64".to_string()})
    }

    pub fn send_tx_with_opts (transaction: String, opts: SolanaSendTransactionParams) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: "sendTransaction".to_string(),
            params: vec![serde_json::to_value(transaction).unwrap(), serde_json::to_value(opts).unwrap()]
        }
    }
}