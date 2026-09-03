use reqwest::Client;
use serde::Deserialize;

use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct StellarService {
    client: Client,
    horizon_url: String,
    soroban_rpc_url: String,
    contract_id: String,
}

#[derive(Debug, Deserialize)]
struct HorizonTransaction {
    successful: bool,
}

#[derive(Debug, Deserialize)]
struct HorizonAccount {
    balances: Vec<HorizonBalance>,
}

#[derive(Debug, Deserialize)]
struct HorizonBalance {
    balance: String,
    asset_type: String,
}

#[derive(Debug, Deserialize)]
struct SorobanRpcResponse {
    result: Option<serde_json::Value>,
    error: Option<SorobanRpcError>,
}

#[derive(Debug, Deserialize)]
struct SorobanRpcError {
    message: String,
}

impl StellarService {
    pub fn new(horizon_url: String, soroban_rpc_url: String, contract_id: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            horizon_url,
            soroban_rpc_url,
            contract_id,
        }
    }

    #[tracing::instrument(skip(self), fields(tx_hash = %tx_hash))]
    pub async fn verify_transaction(&self, tx_hash: &str) -> AppResult<bool> {
        let url = format!("{}/transactions/{}", self.horizon_url, tx_hash);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| AppError::Stellar(format!("Horizon request failed: {e}")))?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(false);
        }

        if !response.status().is_success() {
            return Err(AppError::Stellar(format!(
                "Horizon returned status {}",
                response.status()
            )));
        }

        let tx: HorizonTransaction = response
            .json()
            .await
            .map_err(|e| AppError::Stellar(format!("Failed to parse Horizon response: {e}")))?;

        tracing::debug!(tx_hash = %tx_hash, successful = %tx.successful, "Transaction verified");
        Ok(tx.successful)
    }

    #[tracing::instrument(skip(self), fields(account_id = %account_id))]
    pub async fn get_account_balance(&self, account_id: &str) -> AppResult<f64> {
        let url = format!("{}/accounts/{}", self.horizon_url, account_id);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| AppError::Stellar(format!("Horizon request failed: {e}")))?;

        if !response.status().is_success() {
            return Err(AppError::Stellar(format!(
                "Account {} not found on Horizon",
                account_id
            )));
        }

        let account: HorizonAccount = response
            .json()
            .await
            .map_err(|e| AppError::Stellar(format!("Failed to parse account: {e}")))?;

        let xlm_balance = account
            .balances
            .iter()
            .find(|b| b.asset_type == "native")
            .and_then(|b| b.balance.parse::<f64>().ok())
            .unwrap_or(0.0);

        Ok(xlm_balance)
    }

    #[tracing::instrument(skip(self), fields(contract_id = %self.contract_id, method = %method))]
    pub async fn invoke_contract(
        &self,
        method: &str,
        args: serde_json::Value,
    ) -> AppResult<serde_json::Value> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "simulateTransaction",
            "params": {
                "transaction": {
                    "contract_id": self.contract_id,
                    "method": method,
                    "args": args
                }
            }
        });

        let response = self
            .client
            .post(&self.soroban_rpc_url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Stellar(format!("Soroban RPC request failed: {e}")))?;

        let rpc_response: SorobanRpcResponse = response
            .json()
            .await
            .map_err(|e| AppError::Stellar(format!("Failed to parse RPC response: {e}")))?;

        if let Some(err) = rpc_response.error {
            return Err(AppError::Stellar(format!(
                "Contract error: {}",
                err.message
            )));
        }

        rpc_response
            .result
            .ok_or_else(|| AppError::Stellar("Empty RPC result".to_string()))
    }
}
