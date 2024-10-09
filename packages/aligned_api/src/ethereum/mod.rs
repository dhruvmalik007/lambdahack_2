use std::{str::FromStr, sync::Arc};
use rocket::serde::{json::Json, Serialize };
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{NameOrAddress, TransactionRequest, U256},
};

use lazy_static::lazy_static;
use reqwest::Url;

const BATCHER_PAYMENTS_ADDRESS: &str = "0x815aeCA64a974297942D2Bbf034ABEe22a38A003";
const CHAIN_ID: u64 = 17000;

lazy_static! {
    pub static ref RPC_URL: String = std::env::var("RPC_URL").unwrap();

    /// Local signer for the application's wallet
    pub static ref WALLET: LocalWallet = {
        let mut wallet = LocalWallet::from_str(&std::env::var("PRIVATE_KEY").unwrap()).unwrap();
        wallet = wallet.with_chain_id(CHAIN_ID);
        wallet
    };

    /// Provider connected to the Ethereum network
    pub static ref PROVIDER: Provider<Http> = {
        let rpc_url = Url::parse(&RPC_URL.clone()).unwrap();
        Provider::<Http>::try_from(rpc_url.as_str())
        .expect("failed to connect to provider")
    };

    /// A signer connected to the Ethereum network
    pub static ref SIGNER: Arc<SignerMiddleware<Provider<Http>, LocalWallet>> = {
        let signer = SignerMiddleware::new(PROVIDER.clone(), WALLET.clone());
        Arc::new(signer)
    };
}

/// Uses the application's wallet to pay the batcher costs
pub async fn pay_batcher_costs(cost: u64) -> anyhow::Result<()> {
    tracing::info!("Paying batcher costs: {cost} wei");

    let tx = TransactionRequest {
        from: WALLET.address().into(),
        to: NameOrAddress::Address(BATCHER_PAYMENTS_ADDRESS.parse()?).into(),
        value: U256::from(cost).into(),
        ..Default::default()
    };

    let pending_tx = SIGNER.send_transaction(tx, None).await?;

    let maybe_receipt = pending_tx.await?;

    maybe_receipt
        .inspect(|receipt| {
            tracing::info!("Batcher costs paid at tx {:?}", receipt.transaction_hash)
        })
        .ok_or(anyhow::anyhow!("Failed to pay batcher costs"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pay_batcher_costs() {
        // Setup
        crate::utils::setup();

        // Given
        let cost = 1;

        // When
        // Then
        pay_batcher_costs(cost)
            .await
            .expect("failed to pay batcher costs")
    }
}
