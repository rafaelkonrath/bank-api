use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Params {
    pub client_id: Arc<String>,
    pub client_secret: Arc<String>,
    pub token_uri: Arc<String>,
    pub redirect_uri: Arc<String>,
    pub auth_uri: Arc<String>,
    pub api_uri: Arc<String>,
}