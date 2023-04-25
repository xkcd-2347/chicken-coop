// pub mod data;

pub mod data {
    pub use trust_api_model::prelude::*;
}

use crate::backend::data::{
    Package, PackageDependencies, PackageDependents, PackageList, PackageRef, Vulnerability,
};
use packageurl::PackageUrl;
use serde::Deserialize;
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

    pub async fn lookup(&self, purl: PackageUrl<'_>) -> Result<Package, Error> {
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

    pub async fn lookup_batch<'a, I>(&self, purls: I) -> Result<Vec<PackageRef>, Error>
    where
        I: IntoIterator<Item = PackageUrl<'a>>,
    {
        self.batch_to_refs("/api/package", purls).await
    }

    pub async fn versions<'a, I>(&self, purls: I) -> Result<Vec<PackageRef>, Error>
    where
        I: IntoIterator<Item = PackageUrl<'a>>,
    {
        self.batch_to_refs("/api/package/versions", purls).await
    }

    pub async fn dependencies<'a, I>(&self, purls: I) -> Result<Vec<PackageDependencies>, Error>
    where
        I: IntoIterator<Item = PackageUrl<'a>>,
    {
        self.batch_to_refs("/api/package/dependencies", purls).await
    }

    pub async fn dependents<'a, I>(&self, purls: I) -> Result<Vec<PackageDependents>, Error>
    where
        I: IntoIterator<Item = PackageUrl<'a>>,
    {
        self.batch_to_refs("/api/package/dependents", purls).await
    }

    /// common call of getting some refs for a batch of purls
    async fn batch_to_refs<'a, I, R>(&self, path: &str, purls: I) -> Result<R, Error>
    where
        I: IntoIterator<Item = PackageUrl<'a>>,
        for<'de> R: Deserialize<'de>,
    {
        let purls = PackageList(
            purls
                .into_iter()
                .map(|purl| purl.to_string())
                .collect::<Vec<_>>(),
        );

        Ok(self
            .client
            .post(self.backend.url.join(path)?)
            .json(&purls)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

pub struct VulnerabilityService {
    backend: Backend,
    client: reqwest::Client,
}

impl VulnerabilityService {
    pub fn new(backend: Backend) -> Self {
        Self {
            backend,
            client: reqwest::Client::new(),
        }
    }

    pub async fn lookup(&self, cve: &String) -> Result<Vulnerability, Error> {
        Ok(self
            .client
            .get(self.backend.url.join("/api/vulnerability")?)
            .query(&[("cve", cve.to_string())])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
