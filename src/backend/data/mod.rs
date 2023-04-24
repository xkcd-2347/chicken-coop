use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purl: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "trustedVersions")]
    pub trusted_versions: Vec<PackageRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vulnerabilities: Vec<VulnerabilityRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snyk: Option<SnykData>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vulnerability {
    pub cve: String,
    pub severity: Option<String>,
    pub cvss3: Option<Cvss3>,
    pub summary: String,
    pub advisory: String,
    pub packages: Vec<PackageRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cvss3 {
    pub score: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VulnerabilityRef {
    pub cve: String,
    pub href: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageRef {
    pub purl: String,
    pub href: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnykData;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageDependencies(pub Vec<PackageRef>);

impl Deref for PackageDependencies {
    type Target = [PackageRef];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageDependents(pub Vec<PackageRef>);

impl Deref for PackageDependents {
    type Target = [PackageRef];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageList(pub Vec<String>);
