use tonic::{Request, Response, Status};

use crate::pb::v1::fetcher_service_server::FetcherService;
use crate::pb::v1::{
    FetchRequest, FetchResponse, GetSupportedPatternsRequest, GetSupportedPatternsResponse,
};

#[derive(Default)]
pub struct EchoFetcher {}

#[tonic::async_trait]
impl FetcherService for EchoFetcher {
    async fn get_supported_patterns(
        &self,
        _request: Request<GetSupportedPatternsRequest>,
    ) -> Result<Response<GetSupportedPatternsResponse>, Status> {
        let resp = GetSupportedPatternsResponse {
            patterns: vec!["^(?!https?://)".to_string()],
        };
        Ok(Response::new(resp))
    }

    async fn fetch(
        &self,
        request: Request<FetchRequest>,
    ) -> Result<Response<FetchResponse>, Status> {
        let req = request.into_inner();
        let resp = FetchResponse {
            title: req.url.clone(),
            content: vec![req.url.clone()],
        };
        Ok(Response::new(resp))
    }
}
