use super::{Backend, Error};
use crate::backend::data::Vulnerability;

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
