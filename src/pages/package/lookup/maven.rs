use super::*;

const DEFAULT_SEARCH: &str = r#"<dependency>
    <groupId>io.quarkus</groupId>
    <artifactId>quarkus-core</artifactId>
    <version>2.16.2.Final</version>
</dependency>"#;

fn get_deps_value<'a>(doc: &'a roxmltree::Document, tag: &str) -> Option<&'a str> {
    doc.root()
        .descendants()
        .find(|n| n.tag_name().name() == tag)
        .and_then(|n| n.text())
}

#[function_component(MavenVariant)]
pub fn maven(props: &PackageLookupProperties) -> Html {
    let validator = Callback::from(|input: String| {
        if input.is_empty() {
            bail!(MSG_NOT_EMPTY);
        }

        let s = input.split(":").collect::<Vec<_>>();
        if s.len() > 2 {
            let mut purl = PackageUrl::new("maven", s[1].to_string())?;
            purl.with_namespace(s[0].to_string())
                .with_version(s[2].to_string());
            return Ok(purl);
        }

        let doc = match roxmltree::Document::parse(&input) {
            Err(err) => {
                bail!("Unable to parse as XML ({err})");
            }
            Ok(doc) => doc,
        };

        let artifact_id = get_deps_value(&doc, "artifactId")
            .ok_or_else(|| anyhow!("Missing required <artifactId>"))?;

        let mut purl = PackageUrl::new("maven", artifact_id.to_string())?;

        if let Some(group_id) = get_deps_value(&doc, "groupId") {
            purl.with_namespace(group_id.to_string());
        }
        if let Some(version) = get_deps_value(&doc, "version") {
            purl.with_version(version.to_string());
        }
        if let Some(r#type) = get_deps_value(&doc, "type") {
            let _ = purl.add_qualifier("type", r#type.to_string());
        }

        Ok(purl)
    });

    html!(
        <SingleEntryVariant
            onchange={props.onchange.clone()}
            {validator}
            label="Maven Coordinates"
            r#type={EntryType::Area}
            default={DEFAULT_SEARCH}
        />
    )
}
