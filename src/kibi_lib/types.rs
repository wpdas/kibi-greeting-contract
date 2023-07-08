use reqwest::header::{CONTENT_TYPE};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
  NONE,
  ACCOUNT,
  CONTRACT,
}

pub struct KibContract {}

#[derive(Deserialize, Debug, Serialize)]
struct Foo {
    data: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractTransactionData {
    pub tx_type: TransactionType,
    pub contract_id: String,
    pub timestamp: Option<u64>,
    pub data: String,
  }

// Store Contract Data
const API: &'static str = "http://localhost:8000";
const CONTRACT_TRANSACTION: &'static str = "/contract_transaction";

impl KibContract {
    pub async fn load(contract_id: String) -> Option<String> {

        // Load content from chain
        let url = format!("http://localhost:8000/contract_payload/{contract_id}", contract_id = "dev-1234");
        let res_json = reqwest::get(url).await
        .unwrap().text().await.unwrap();
        let response: ContractTransactionData = serde_json::from_str(&res_json).unwrap();
        
        // If it's contract tx
        if response.tx_type == TransactionType::CONTRACT {
            if response.contract_id == contract_id {
                return Some(response.data);
            }
        }

        None
    }

    pub async fn persist<C: Serialize>(contract_id: String, contract: C) {
        let client = reqwest::Client::new();
        let url = format!("{api}{route}", api = API, route = CONTRACT_TRANSACTION);

        let contract_data = serde_json::to_string(&contract).unwrap();
    
        let body = json!({
            "tx_type": TransactionType::CONTRACT,
            "contract_id": contract_id,
            "data": &contract_data
        });
    
        let json_body = serde_json::to_string(&body).unwrap();
    
        let _ = client.post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(json_body)
        .send()
        .await;

        println!("Done")
    }
}