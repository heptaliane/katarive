use regex::Regex;
use std::error::Error;
use tonic::transport::Channel;

use crate::pb::v1::fetcher_service_client::FetcherServiceClient;
use crate::pb::v1::{FetchRequest, FetchResponse, GetSupportedPatternsRequest};

pub struct Fetcher {
    client: FetcherServiceClient<Channel>,
    patterns: Vec<Regex>,
}

impl Fetcher {
    async fn new(addr: &str) -> Result<Self, Box<dyn Error>> {
        let mut client = FetcherServiceClient::connect(addr.to_string()).await?;
        let req = GetSupportedPatternsRequest {};
        let res = client.get_supported_patterns(req).await?;
        let patterns = res
            .into_inner()
            .patterns
            .iter()
            .map(|p| Regex::new(&p).expect("Invalid regex format"))
            .collect();

        Ok(Self {
            client: client,
            patterns: patterns,
        })
    }

    pub async fn fetch(&mut self, url: &str) -> Result<FetchResponse, Box<dyn Error>> {
        let req = FetchRequest {
            url: url.to_string(),
        };
        let res = self.client.fetch(req).await?;
        Ok(res.into_inner())
    }

    fn is_supported_url(&self, url: &str) -> bool {
        self.patterns.iter().any(|re| re.is_match(url))
    }
}

pub struct FetcherManager {
    fetchers: Vec<Fetcher>,
}

impl FetcherManager {
    pub fn new() -> Self {
        Self {
            fetchers: Vec::new(),
        }
    }

    pub async fn add_client(&mut self, addr: &str) -> Result<(), Box<dyn Error>> {
        self.fetchers.push(Fetcher::new(addr).await?);
        Ok(())
    }

    pub fn find_client(&self, url: &str) -> Option<&Fetcher> {
        self.fetchers.iter().find(|f| f.is_supported_url(url))
    }
}
