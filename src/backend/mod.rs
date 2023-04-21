pub mod data;

use crate::backend::data::Package;
use packageurl::PackageUrl;
use url::{ParseError, Url};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Backend {
    pub url: Url,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse backend URL: {0}")]
    Url(#[from] ParseError),
    #[error("Failed to request: {0}")]
    Request(#[from] reqwest::Error),
}

pub struct PackageService {
    backend: Backend,
    client: reqwest::Client,
}

impl PackageService {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            client: reqwest::Client::new(),
        }
    }

    pub async fn lookup(&self, purl: &PackageUrl<'_>) -> Result<Package, Error> {
        Ok(self
            .client
            .get(self.backend.url.join("/api/package")?)
            .query(&[("purl", purl.to_string())])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
