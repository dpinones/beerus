use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde_json::Value;
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryStateRootResponse {
    pub state_root: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryContractViewResponse {
    pub result: Vec<String>,
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetStorageAtResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryNonceResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryL1ToL2MessageCancellationsResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryL1ToL2MessagesResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryL2ToL1MessagesResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryL1ToL2MessageNonceResponse {
    pub result: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryChainIdResponse {
    pub chain_id: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockNumberResponse {
    pub block_number: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryBlockHashAndNumberResponse {
    pub block_hash: String,
    pub block_number: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetClassResponse {
    pub abi: Value,
    pub entry_points_by_type: Value,
    pub program: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetClassAtResponse {
    pub abi: Value,
    pub entry_points_by_type: Value,
    pub program: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QueryGetBlockTransactionCountResponse {
    pub block_transaction_count: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct QuerySyncing {
    pub data: Option<Value>,
    pub status: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddInvokeTransactionJson {
    /// The maximal fee that can be charged for including the transaction
    pub max_fee: String,
    pub signature: Vec<String>,
    pub nonce: String,
    pub contract_address: String,
    pub entry_point_selector: String,
    /// The parameters passed to the function
    pub calldata: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AddInvokeTransactionResponse {
    pub transaction_hash: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddDeployTransactionJson {
    pub contract_class: String,
    pub version: String,
    pub contract_address_salt: String,
    pub constructor_calldata: Vec<String>,
}

#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct AddDeployTransactionResponse {
    pub transaction_hash: String,
    pub contract_address: String,
}
