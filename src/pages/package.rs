use crate::backend::{data::PackageRef, Backend, PackageService};
use crate::pages::AppRoute;
use crate::utils::RenderOptional;
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use std::str::FromStr;
use yew::prelude::*;
use yew_more_hooks::hooks::r#async::*;
use yew_nested_router::prelude::*;

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

    let fetch_package = {
        let backend = backend.clone();
        let purl = props.purl.clone();
        use_async_with_options(
            async move { PackageService::new((*backend).clone()).lookup(&purl).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let fetch_versions = {
        let purl = props.purl.clone();
        use_async_with_options(
            async move {
                PackageService::new((*backend).clone())
                    .versions([purl])
                    .await
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    html!(
        <Grid gutter=true>
            <GridItem>
                <Gallery style="--pf-l-gallery--GridTemplateColumns--min: 500px;" gutter=true>
                    <Card
                        title={html!(<Title size={Size::XLarge}>{"Package URL"}</Title>)}
                    >
                        <DescriptionList>
                            <DescriptionGroup term="Raw">{&props.purl}</DescriptionGroup>
                            <DescriptionGroup term="Type">{&props.purl.ty()}</DescriptionGroup>
                            <DescriptionGroup term="Namespace">{props.purl.clone().namespace().or_none()}</DescriptionGroup>
                            <DescriptionGroup term="Name">{&props.purl.name()}</DescriptionGroup>
                            <DescriptionGroup term="Version">{props.purl.version().clone().or_none()}</DescriptionGroup>
                        </DescriptionList>
                    </Card>
                    <Card>
                        {
                            match &*fetch_package {
                                UseAsyncState::Pending | UseAsyncState::Processing => html!(<Spinner/>),
                                UseAsyncState::Ready(Ok(data)) => html!(<PackageDetails package={data.clone()}/>),
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
                </Gallery>
            </GridItem>
        </Grid>
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PackageDetailsProperties {
    pub package: crate::backend::data::Package,
}

#[function_component(PackageDetails)]
fn package_details(props: &PackageDetailsProperties) -> Html {
    html!(
        <DescriptionList>
            <DescriptionGroup term="Trusted">{props.package.trusted.or_none()}</DescriptionGroup>
        </DescriptionList>
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PackageVersionsProperties {
    pub versions: Vec<PackageRef>,
}

#[function_component(PackageVersions)]
fn package_versions(props: &PackageVersionsProperties) -> Html {
    #[derive(PartialEq)]
    struct PackageVersion<'a> {
        version: String,
        purl: PackageUrl<'a>,
        pkg: &'a PackageRef,
    }

    let mut versions = Vec::with_capacity(props.versions.len());
    for pkg in &props.versions {
        let purl = match PackageUrl::from_str(&pkg.purl) {
            Ok(purl) => purl,
            Err(_) => continue,
        };
        let version = match purl.version() {
            Some(version) => version.to_string(),
            None => continue,
        };
        versions.push(PackageVersion { version, purl, pkg });
    }

    // do numeric version sorting
    versions.sort_unstable_by(|a, b| a.version.cmp(&b.version).reverse());

    html!(
        <List r#type={ListType::Plain}>
            {for versions.iter().map(|v|{
                html!(<>
                    <yew_nested_router::components::Link<AppRoute>
                        target={AppRoute::Package { package: v.purl.to_string() }}
                    >
                        {&v.version}
                    </yew_nested_router::components::Link<AppRoute>>
                    if v.pkg.trusted.unwrap_or_default() {
                        {" "}<Label color={Color::Gold} label="Trusted"/>
                    }
                </>)
            })}
        </List>
    )
}

#[function_component(LookupPackageModal)]
fn lookup_package_modal() -> Html {
    use patternfly_yew::next::TextInput;

    let form_state = use_state_eq(InputState::default);

    let onvalidated_form = {
        let form_state = form_state.clone();
        Callback::from(move |state| form_state.set(state))
    };

    let validator_purl = |ctx: ValidationContext<String>| {
        if ctx.value.is_empty() {
            return ValidationResult::error("Must not be empty");
        }

        if let Err(err) = PackageUrl::from_str(&ctx.value) {
            return ValidationResult::error(format!("Unable to parse as Package URL({err})"));
        }

        ValidationResult::ok()
    };
    let purl =
        use_state_eq(|| "pkg:maven/io.quarkus/quarkus-core@2.16.2.Final?type=jar".to_string());

    let router = use_router::<AppRoute>();
    let backdrop = use_backdrop();
    let onclick = {
        let purl = purl.clone();
        let backdrop = backdrop.clone();
        Callback::from(move |_| {
            if let Some(backdrop) = &backdrop {
                backdrop.close();
            }
            if let Some(router) = &router {
                router.push(AppRoute::Package {
                    package: (*purl).clone(),
                })
            }
        })
    };

    let footer = {
        html!(
            <Button
                variant={ButtonVariant::Primary}
                disabled={(*form_state) == InputState::Error}
                r#type={ButtonType::Submit}
                {onclick}
                form="lookup-form"
            >
                {"Lookup"}
            </Button>
        )
    };

    html!(
        <Bullseye plain=true>
            <Modal
                title="Lookup Package"
                variant={ModalVariant::Small}
                {footer}
            >
                <Form id="lookup-form" method="dialog"
                    onvalidated={onvalidated_form}
                >
                    <FormGroupValidated<TextInput>
                        label="Package URL (PURL)"
                        required=true
                        validator={Validator::from(validator_purl)}
                    >
                        <TextInput value={(*purl).clone()} oninput={Callback::from(move |data| purl.set(data))}/>
                    </FormGroupValidated<TextInput>>
                </Form>
            </Modal>
        </Bullseye>
    )
}
