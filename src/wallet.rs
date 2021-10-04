use crate::client::BitGoClient;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
pub trait BitGoWalletAPI {
    async fn generate_wallet(
        &self,
        name: &str,
        identifier: &str,
        passphrase: &str,
        enterprise_id: &str,
    ) -> Result<serde_json::Value>;
    async fn create_address(
        &self,
        wallet_id: &str,
        identifier: &str,
        forwarder_version: i32,
    ) -> Result<serde_json::Value>;
    async fn get_wallet_list(&self) -> Result<serde_json::Value>;
}

#[async_trait]
impl BitGoWalletAPI for BitGoClient {
    /// This API call creates a new wallet. Under the hood, the SDK (or BitGo Express) does the following:
    ///
    /// 1.Creates the user keychain locally on the machine, and encrypts it with the provided passphrase (skipped if userKey is provided).
    /// 2.Creates the backup keychain locally on the machine.
    /// 3.Uploads the encrypted user keychain and public backup keychain.
    /// 4.Creates the BitGo key (and the backup key if backupXpubProvider is set) on the service.
    /// 5.Creates the wallet on BitGo with the 3 public keys above.
    async fn generate_wallet(
        &self,
        name: &str,
        identifier: &str,
        passphrase: &str,
        enterprise_id: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/generate",
            url = self.endpoint,
            coin_type = identifier,
        );

        self.post_api(
            &request_url,
            &json!({
                "label": name,
                "passphrase": passphrase,
                "enterprise":enterprise_id,
            }),
        )
        .await
    }

    /// This API call is used to create a new receive address for your wallet.
    /// You may choose to call this API whenever a deposit is made.
    /// The BitGo API supports millions of addresses.
    async fn create_address(
        &self,
        wallet_id: &str,
        identifier: &str,
        forwarder_version: i32,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/address",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.post_api(
            &request_url,
            &json!({
                "forwarderVersion":forwarder_version,
            }),
        )
        .await
    }

    async fn get_wallet_list(&self) -> Result<serde_json::Value> {
        let request_url = format!("{url}/api/v2/wallets/", url = self.endpoint,);
        self.get_api(&request_url, &json!({})).await
    }
}
