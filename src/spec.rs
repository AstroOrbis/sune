use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SuneRpcRequest {
	pub jsonrpc: String,
	pub id: String,
	pub method: String,
	pub params: Vec<serde_json::Value>,
}

impl<T: Serialize + Into<Methods> + Clone> From<T> for SuneRpcRequest {
	fn from(param: T) -> Self {
		Self {
			jsonrpc: "2.0".to_string(),
			id: "1".to_string(),
			method: String::from(param.clone().into()),
			params: vec![serde_json::to_value(param).unwrap()]
		}
	}
}


pub enum Methods {
	SendJito,
	SendRpc,
}

impl From<String> for Methods {
	fn from(method: String) -> Self {
		match method.as_str() {
			"send_jito" => Self::SendJito,
			"send_rpc" => Self::SendRpc,
			_ => panic!("Method not found")
		}
	}
}

impl From<Methods> for String {
	fn from(method: Methods) -> Self {
		match method {
			Methods::SendJito => "send_jito".to_string(),
			Methods::SendRpc => "send_rpc".to_string(),
		}
	}

}

macro_rules! into_method {
	($name:ident, $method:expr) => {
		impl From<$name> for Methods {
			fn from(_: $name) -> Self {
				$method
			}
		}
	};
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendJitoParams {
	pub transaction: String
}

into_method!(SendJitoParams, Methods::SendJito);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SendRpcParams {
	pub transaction: String
}

into_method!(SendRpcParams, Methods::SendRpc);