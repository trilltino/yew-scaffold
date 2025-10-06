use gloo_net::http::Request;
use shared::dto::soroban::{
    MetricsResponse, SorobanHealthResponse
};

#[derive(Default, Clone)]
pub struct SorobanApiClient {
    base_url: String,
}

impl SorobanApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:3001".to_string(),
        }
    }

    /// Get Soroban service metrics
    pub async fn get_metrics(&self) -> Result<MetricsResponse, String> {
        let url = format!("{}/api/soroban/metrics", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {e}"))?;

        if response.ok() {
            response
                .json::<MetricsResponse>()
                .await
                .map_err(|e| format!("Response parse error: {e}"))
        } else {
            Err(format!("HTTP error: {}", response.status()))
        }
    }

    /// Get Soroban service health status
    pub async fn get_health(&self) -> Result<SorobanHealthResponse, String> {
        let url = format!("{}/api/soroban/health", self.base_url);

        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {e}"))?;

        if response.ok() {
            response
                .json::<SorobanHealthResponse>()
                .await
                .map_err(|e| format!("Response parse error: {e}"))
        } else {
            Err(format!("HTTP error: {}", response.status()))
        }
    }

}
