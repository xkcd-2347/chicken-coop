use crate::{
    backend::data::{self, PackageRef},
    components::Trusted,
    pages::AppRoute,
};
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::str::FromStr;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PackageRefsProperties {
    pub refs: Vec<PackageRef>,
}

#[function_component(PackageReferences)]
pub fn package_refs(props: &PackageRefsProperties) -> Html {
    #[derive(PartialEq)]
    struct PackageRef<'a> {
        label: String,
        purl: PackageUrl<'a>,
        pkg: &'a data::PackageRef,
    }

    let mut refs = Vec::with_capacity(props.refs.len());
    for pkg in &props.refs {
        let purl = match PackageUrl::from_str(&pkg.purl) {
            Ok(purl) => purl,
            Err(_) => continue,
        };
        let label = match purl.namespace() {
            Some(namespace) => format!("{namespace} : {name}", name = purl.name()),
            None => purl.name().to_string(),
        };
        refs.push(PackageRef { label, purl, pkg });
    }

    html!(
        <List r#type={ListType::Plain}>
            {for refs.iter().map(|r|{
                html!(<>
                    <yew_nested_router::components::Link<AppRoute>
                        target={AppRoute::Package { package: r.purl.to_string()}}
                    >
                        { &r.label }
                    </yew_nested_router::components::Link<AppRoute>>
                    if r.pkg.trusted.unwrap_or_default() {
                        {" "}<Trusted/>
                    }
                </>)
            })}
        </List>
    )
}
