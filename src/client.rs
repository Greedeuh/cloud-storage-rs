use tokio::sync::Mutex;

use crate::{service_account::ServiceAccount, token::Token};

/// Wrap http client and authentication info to finally make the call to the google API
pub struct Client {
    /// internally maintain a pool so this instance is reusable
    pub http_client: reqwest::Client,

    /// Service account auth token that auto-regenerate
    pub token_cache: Mutex<Token>,
    /// Auth json file parsed
    pub service_account: ServiceAccount,
}

impl Default for Client {
    fn default() -> Self {
        let http_client = reqwest::Client::default();
        let token_cache = Mutex::new(Token::new(
            "https://www.googleapis.com/auth/devstorage.full_control",
        ));
        let service_account = ServiceAccount::get();
        Client {
            http_client,
            token_cache,
            service_account,
        }
    }
}
