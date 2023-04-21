use serde::{Deserialize, Serialize};

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
pub struct VulnerabilityRef {
    pub cve: String,
    pub href: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageRef {
    pub purl: String,
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnykData;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageDependencies(pub Vec<PackageRef>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageDependents(pub Vec<PackageRef>);
