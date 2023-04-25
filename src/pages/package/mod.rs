mod lookup;
mod versions;

use crate::{
    backend::{data, Backend, PackageService},
    components::{deps::PackageReferences, remote_content, Trusted},
    pages::AppRoute,
    utils::RenderOptional,
};
use lookup::*;
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use versions::*;
use yew::prelude::*;
use yew_more_hooks::hooks::r#async::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PackageProperties {
    #[prop_or_default]
    pub package: String,
}

#[function_component(Package)]
pub fn package(props: &PackageProperties) -> Html {
    let backdrop = use_backdrop();
    let primary = Callback::from(move |_| {
        if let Some(backdrop) = &backdrop {
            backdrop.open(html!(<LookupPackageModal/>));
        }
    })
    .into_action("Lookup");

    html!(
        <>
            <PageSection variant={PageSectionVariant::Light} sticky={[PageSectionSticky::Top]} >
                if let Some(purl) = purl(&props.package) {
                    <Title size={Size::XXXXLarge}>{package_title(purl)}</Title>
                } else {
                    <Title size={Size::XXXXLarge}>{"Packages"}</Title>
                }
                <Content>
                    <p>{ "Get detailed package information" }</p>
                </Content>
            </PageSection>

            // We need to set the main section to fill, as we have a footer section
            <PageSection variant={PageSectionVariant::Default} fill={PageSectionFill::Fill}>
                if let Some(purl) = purl(&props.package) {
                    <PackageInformation {purl} />
                } else {
                    <Bullseye>
                        <EmptyState
                            full_height=true
                            title="Package Information"
                            icon={Icon::Package}
                            {primary}
                        >
                            { "Lookup a package to find more information" }
                        </EmptyState>
                    </Bullseye>
                }
            </PageSection>
        </>
    )
}

fn package_title(purl: PackageUrl) -> Html {
    let mut title = vec![];
    Extend::extend(&mut title, purl.namespace());
    title.push(purl.name());
    Extend::extend(&mut title, purl.version());

    html!(
        <>
            { title.join(" : ") }
            {" "}
            <Label label={purl.ty().to_string()} color={Color::Blue}/>
        </>
    )
}

fn purl(package: &str) -> Option<PackageUrl<'static>> {
    if package.is_empty() {
        return None;
    }

    PackageUrl::from_str(package).ok()
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PackageInformationProperties {
    purl: PackageUrl<'static>,
}

#[function_component(PackageInformation)]
fn package_information(props: &PackageInformationProperties) -> Html {
    let backend = use_context::<Rc<Backend>>()
        .expect("Can only be called being wrapped by the 'Backend' component");

    let service = use_memo(
        |backend| PackageService::new((**backend).clone()),
        backend.clone(),
    );

    let fetch_package = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purl| async move { service.lookup(purl).await },
            props.purl.clone(),
        )
    };

    let fetch_versions = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purl| async move { service.versions([purl]).await },
            props.purl.clone(),
        )
    };

    let fetch_deps_out = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purl| async move { service.dependencies([purl]).await },
            props.purl.clone(),
        )
    };

    let fetch_deps_in = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purl| async move { service.dependents([purl]).await },
            props.purl.clone(),
        )
    };

    let pkg_name = match props.purl.namespace().clone() {
        Some(namespace) => html!(<> {namespace} {" : "} {props.purl.name()} </>),
        None => html!(props.purl.name()),
    };

    html!(
        <Grid gutter=true>
            <GridItem cols={[9]}>
                <Card compact=true>
                    <Tabs>
                        <Tab label={remote_refs_count_title(&fetch_deps_out, |data|data.first(), "Dependency", "Dependencies")}>
                            { remote_content(&fetch_deps_out, |data| html!(
                                <PackageReferences refs={data.first().cloned().map(|d|d.0).unwrap_or_default()} />
                            )) }
                        </Tab>

                        <Tab label={remote_refs_count_title(&fetch_deps_in, |data|data.first(), "Dependent", "Dependents")}>
                            { remote_content(&fetch_deps_in, |data| html!(
                                <PackageReferences refs={data.first().cloned().map(|d|d.0).unwrap_or_default()} />
                            )) }
                        </Tab>

                        <Tab label={remote_refs_count_title(&fetch_package, |data|Some(&data.vulnerabilities), "Vulnerability", "Vulnerabilities")}>
                            { remote_content(&fetch_package, |data| html!(
                                <PackageVulnerabilities package={data.clone()} />
                            )) }
                        </Tab>
                    </Tabs>
                </Card>
            </GridItem>

            <GridItem cols={[3]}>
                <Gallery style="--pf-l-gallery--GridTemplateColumns--min: 500px;" gutter=true>

                    <Card
                        title={html!(<Title size={Size::XLarge}>{ pkg_name }</Title>)}
                    >
                        <Clipboard readonly=true code=true value={props.purl.to_string()} />
                        <DescriptionList>
                            <DescriptionGroup term="Version">{props.purl.version().clone().or_none()}</DescriptionGroup>
                            if let Some(path) = props.purl.subpath() {
                                <DescriptionGroup term="Path">{path}</DescriptionGroup>
                            }
                            { for props.purl.qualifiers().iter().map(|(k, v)|{
                                html!(<DescriptionGroup term={k.to_string()}> { v } </DescriptionGroup>)
                            })}
                        </DescriptionList>
                    </Card>

                    { remote_card(&fetch_package, |data|
                        html!(<>
                            {"Support"}
                            if let Some(data::Package{trusted: Some(true), ..}) = data {
                                {" "} <Trusted/>
                            }
                        </>),
                    |data| html!( <>
                        <PackageDetails package={data.clone()}/>
                        <PackageVulnerabilities package={data.clone()}/>
                    </> )) }

                    { remote_card(&fetch_versions, |data|
                        remote_card_title_badge("Versions", data.map(|r|r.len())),
                    |data| html!(
                        <PackageVersions versions={data.clone()}/>
                    )) }

                </Gallery>
            </GridItem>
        </Grid>
    )
}

fn remote_refs_count_title<T, E, F, R, X>(
    fetch: &UseAsyncHandleDeps<T, E>,
    f: F,
    singular: &str,
    plural: &str,
) -> String
where
    F: FnOnce(&T) -> Option<&R>,
    R: Deref<Target = [X]>,
{
    match &**fetch {
        UseAsyncState::Ready(Ok(data)) => match f(data).map(|r| r.len()) {
            Some(1) => format!("1 {singular}"),
            Some(len) => format!("{len} {plural}"),
            None => plural.to_string(),
        },
        _ => plural.to_string(),
    }
}

fn remote_card_title_badge(title: &str, entries: Option<usize>) -> Html {
    html!(<>
        {title}
        if let Some(entries) = entries {
            { " " } <Badge read=true> { entries } </Badge>
        }
    </>)
}

fn remote_card<T, E, FT, FB>(fetch: &UseAsyncState<T, E>, title: FT, body: FB) -> Html
where
    FT: FnOnce(Option<&T>) -> Html,
    FB: FnOnce(&T) -> Html,
    E: std::error::Error,
{
    let fetch = &*fetch;
    html!(
        <Card
            title={html!(<Title size={Size::XLarge}>
                { title(fetch.data()) }
            </Title>)}
        >
            { remote_content(fetch, body) }
        </Card>
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PackageDetailsProperties {
    pub package: data::Package,
}

#[function_component(PackageDetails)]
fn package_details(_props: &PackageDetailsProperties) -> Html {
    html!()
}

#[function_component(PackageVulnerabilities)]
fn package_details(props: &PackageDetailsProperties) -> Html {
    struct Vuln<'a> {
        cve: &'a str,
        // FIXME: try checking if we can add the severity
    }

    let vulns = props
        .package
        .vulnerabilities
        .iter()
        .map(|v| Vuln { cve: &v.cve })
        .collect::<Vec<_>>();

    html!(
        if !vulns.is_empty() {
            <Title level={Level::H3}>{ "Known vulnerabilities" } </Title>
            <List r#type={ListType::Plain}>
                {for vulns.into_iter().map(|v|{
                    html!(<>
                        <yew_nested_router::components::Link<AppRoute>
                            target={AppRoute::Vulnerability { cve: v.cve.to_string() }}
                        >
                            { &v.cve }
                        </yew_nested_router::components::Link<AppRoute>>
                    </>)
                })}
            </List>
        }
    )
}
