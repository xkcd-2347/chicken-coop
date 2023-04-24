mod deps;
mod lookup;
mod versions;

use deps::*;
use lookup::*;
use versions::*;

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
                    <Card
                        title={html!(<Title size={Size::XLarge}>
                            {"Support"}
                            if let UseAsyncState::Ready(Ok(data::Package{trusted: Some(true), ..})) = &*fetch_package {
                                {" "} <Trusted/>
                            }
                        </Title>)}
                    >
                        {
                            match &*fetch_package {
                                UseAsyncState::Pending | UseAsyncState::Processing => html!(<Spinner/>),
                                UseAsyncState::Ready(Ok(data)) => html!(
                                    <>
                                        <PackageDetails package={data.clone()}/>
                                        <PackageVulnerabilities package={data.clone()}/>
                                    </>
                                ),
                                UseAsyncState::Ready(Err(err)) => html!(<>{"Failed to load: "} { err } </>),
                            }
                        }
                    </Card>
                    <Card
                        title={html!(<Title size={Size::XLarge}>{"Versions"}</Title>)}
                    >
                        {
                            match &*fetch_versions {
                                UseAsyncState::Pending | UseAsyncState::Processing => html!(<Spinner/>),
                                UseAsyncState::Ready(Ok(data)) => html!(<PackageVersions versions={data.clone()}/>),
                                UseAsyncState::Ready(Err(err)) => html!(<>{"Failed to load: "} { err } </>),
                            }
                        }
                    </Card>
                    <Card
                        title={html!(<Title size={Size::XLarge}>
                            {"Dependencies"}
                            if let UseAsyncState::Ready(Ok(data)) = &*fetch_deps_out {
                                { " " } <Badge read=true> { data.first().as_ref().map(|d|d.0.len()).unwrap_or_default() } </Badge>
                            }
                        </Title>)}
                    >
                        {
                            match &*fetch_deps_out {
                                UseAsyncState::Pending | UseAsyncState::Processing => html!(<Spinner/>),
                                UseAsyncState::Ready(Ok(data)) => html!(<PackageReferences
                                    refs={data.first().cloned().map(|d|d.0).unwrap_or_default()}
                                />),
                                UseAsyncState::Ready(Err(err)) => html!(<>{"Failed to load: "} { err } </>),
                            }
                        }
                    </Card>
                    <Card
                        title={html!(<Title size={Size::XLarge}>
                            {"Dependents"}
                            if let UseAsyncState::Ready(Ok(data)) = &*fetch_deps_in {
                                { " " } <Badge read=true> { data.first().as_ref().map(|d|d.0.len()).unwrap_or_default() } </Badge>
                            }
                        </Title>)}
                    >
                        {
                            match &*fetch_deps_in {
                                UseAsyncState::Pending | UseAsyncState::Processing => html!(<Spinner/>),
                                UseAsyncState::Ready(Ok(data)) => html!(<PackageReferences
                                    refs={data.first().cloned().map(|d|d.0).unwrap_or_default()}
                                />),
                                UseAsyncState::Ready(Err(err)) => html!(<>{"Failed to load: "} { err } </>),
                            }
                        }
                    </Card>
                </Gallery>
            </GridItem>
        </Grid>
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
