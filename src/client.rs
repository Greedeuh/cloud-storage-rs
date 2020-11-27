/// Wrap http client and authentication info to finally make the call to the google API
#[derive(Default)]
pub struct Client {
    /// internally maintain a pool so this instance is reusable
    pub http_client: reqwest::Client,
}
