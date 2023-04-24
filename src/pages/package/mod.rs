mod deps;
mod lookup;
mod versions;

use deps::*;
use lookup::*;
use std::ops::Deref;
use versions::*;

use crate::backend::data::PackageRef;
use crate::{
    backend::{
        data::{self},
        Backend, PackageService,
    },
    components::Trusted,
    pages::AppRoute,
    utils::RenderOptional,
};
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use std::str::FromStr;
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
                <Content>
                    <Title size={Size::XXXXLarge}>{"Package"}</Title>
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
            |purl| async move { service.lookup(&purl).await },
            props.purl.clone(),
        )
    };

    let fetch_versions = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purl| async move { service.versions([&purl]).await },
            props.purl.clone(),
        )
    };

    let fetch_deps_out = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purl| async move { service.dependencies([&purl]).await },
            props.purl.clone(),
        )
    };

    let fetch_deps_in = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purl| async move { service.dependents([&purl]).await },
            props.purl.clone(),
        )
    };

    let title = match props.purl.namespace().clone() {
        Some(namespace) => html!(<> {namespace} {" : "} {props.purl.name()} </>),
        None => html!(props.purl.name()),
    };

    html!(
        <Grid gutter=true>
            <GridItem>
                <Gallery style="--pf-l-gallery--GridTemplateColumns--min: 500px;" gutter=true>

                    <Card
                        title={html!(<Title size={Size::XLarge}>{ title } {" "} <Label label={props.purl.ty().to_string()} color={Color::Blue}/></Title>)}
                    >
                        <Clipboard readonly=true code=true value={props.purl.to_string()} />
                        <DescriptionList>
                            <DescriptionGroup term="Version">{props.purl.version().clone().or_none()}</DescriptionGroup>
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

                    { remote_card(&fetch_deps_out, |data|
                        remote_card_title_badge("Dependencies", refs_count(data)),
                    |data| html!(
                        <PackageReferences refs={data.first().cloned().map(|d|d.0).unwrap_or_default()} />
                    )) }

                    { remote_card(&fetch_deps_in, |data|
                        remote_card_title_badge("Dependents", refs_count(data)),
                    |data| html!(
                        <PackageReferences refs={data.first().cloned().map(|d|d.0).unwrap_or_default()} />
                    )) }

                </Gallery>
            </GridItem>
        </Grid>
    )
}

fn refs_count<T>(data: Option<&Vec<T>>) -> Option<usize>
where
    T: Deref<Target = [PackageRef]>,
{
    data.map(|d| d.first().map(|r| r.len()).unwrap_or_default())
}

fn remote_card_title_badge(title: &str, entries: Option<usize>) -> Html {
    html!(<>
        {title}
        if let Some(entries) = entries {
            { " " } <Badge read=true> { entries } </Badge>
        }
    </>)
}

fn remote_card<T, E, FT, FB>(fetch: &UseAsyncHandleDeps<T, E>, title: FT, body: FB) -> Html
where
    FT: FnOnce(Option<&T>) -> Html,
    FB: FnOnce(&T) -> Html,
    E: std::error::Error,
{
    let fetch = &**fetch;
    html!(
        <Card
            title={html!(<Title size={Size::XLarge}>
                { title(fetch.data()) }
            </Title>)}
        >
            {
                match &*fetch {
                    UseAsyncState::Pending | UseAsyncState::Processing => html!(<Spinner/>),
                    UseAsyncState::Ready(Ok(data)) => body(data),
                    UseAsyncState::Ready(Err(err)) => html!(<>{"Failed to load: "} { err } </>),
                }
            }
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
