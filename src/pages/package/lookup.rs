use crate::pages::AppRoute;
use anyhow::{anyhow, bail, Context};
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::str::FromStr;
use yew::prelude::*;
use yew_nested_router::prelude::*;

const DEFAULT_SEARCH: &str = "pkg:maven/io.quarkus/quarkus-core@2.13.7.Final-redhat-00003";

#[function_component(LookupPackageModal)]
pub fn lookup_package_modal() -> Html {
    let router = use_router::<AppRoute>();
    let backdrop = use_backdrop();

    let purl = use_state_eq(|| DEFAULT_SEARCH.to_string());
    let form_state = use_state_eq(InputState::default);

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

    let onchange = {
        let purl = purl.clone();
        let form_state = form_state.clone();
        Callback::from(move |data: Option<PackageUrl>| match data {
            Some(data) => {
                purl.set(data.to_string());
                form_state.set(InputState::Default)
            }
            None => form_state.set(InputState::Error),
        })
    };

    let tab = use_state_eq(|| 0);
    let onselect = {
        let tab = tab.clone();
        Callback::from(move |idx| {
            tab.set(idx);
        })
    };

    html!(
        <Bullseye plain=true>
            <Modal
                title="Lookup Package"
                variant={ModalVariant::Medium}
                {footer}
            >
                <Tabs {onselect}>
                    <Tab label="Package URL" />
                    <Tab label="Maven" />
                </Tabs>
                {
                    /* FIXME: we could do better here, we do reset each tab, but that keeps the
                       modal dialog form state sane. Ideally we'd have a per-tab state. */
                    match *tab {
                        0 => html!(<PurlVariant {onchange}/>),
                        1 => html!(<MavenVariant {onchange}/>),
                        _ => html!(),
                    }
                }
            </Modal>
        </Bullseye>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct PackageLookupProperties {
    onchange: Callback<Option<PackageUrl<'static>>>,
}

const MSG_NOT_EMPTY: &str = "Must not be empty";

#[function_component(PurlVariant)]
fn purl(props: &PackageLookupProperties) -> Html {
    let validator = Callback::from(|input: String| {
        if input.is_empty() {
            bail!(MSG_NOT_EMPTY);
        }

        Ok(PackageUrl::from_str(&input).context("Unable to parse as Package URL")?)
    });

    html!(
        <SingleEntryVariant
            onchange={props.onchange.clone()}
            {validator}
            label="Package URL (PURL)"
            r#type={EntryType::Input}
            default={DEFAULT_SEARCH}
        />
    )
}

#[function_component(MavenVariant)]
fn maven(props: &PackageLookupProperties) -> Html {
    const DEFAULT_SEARCH: &str = r#"<dependency>
    <groupId>io.quarkus</groupId>
    <artifactId>quarkus-core</artifactId>
    <version>2.13.7.Final-redhat-00003</version>
</dependency>"#;

    fn get_deps_value<'a>(doc: &'a roxmltree::Document, tag: &str) -> Option<&'a str> {
        doc.root()
            .descendants()
            .find(|n| n.tag_name().name() == tag)
            .and_then(|n| n.text())
    }

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
            purl.with_namespace(version.to_string());
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum EntryType {
    #[default]
    Input,
    Area,
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct SingleValueEntryProperties {
    onchange: Callback<Option<PackageUrl<'static>>>,
    validator: Callback<String, anyhow::Result<PackageUrl<'static>>>,
    label: String,
    #[prop_or_default]
    r#type: EntryType,
    default: String,
}

#[function_component(SingleEntryVariant)]
fn single_entry(props: &SingleValueEntryProperties) -> Html {
    use patternfly_yew::next::TextArea;
    use patternfly_yew::next::TextInput;

    let state = use_state_eq(|| Err(MSG_NOT_EMPTY.to_string()));

    let input = use_state_eq(|| props.default.clone());

    // feed input with changes
    let oninput = {
        let input = input.clone();
        Callback::from(move |data: String| {
            input.set(data.clone());
        })
    };

    {
        // turn input into state
        let state = state.clone();
        let validator = props.validator.clone();
        use_effect_with_deps(
            move |input| {
                state.set(
                    validator
                        .emit((**input).clone())
                        .map_err(|err| err.to_string()),
                );
            },
            input.clone(),
        );
    }
    {
        // report state to parent component
        let onchange = props.onchange.clone();
        use_effect_with_deps(
            move |state| match &**state {
                Ok(purl) => {
                    onchange.emit(Some((*purl).clone()));
                }
                Err(_) => {
                    onchange.emit(None);
                }
            },
            state.clone(),
        )
    }

    let (alert, helper_text) = match &*state {
        Ok(_) => (None, None),
        Err(err) => (
            Some(FormAlert {
                title: "The form contains fields with errors.".into(),
                r#type: AlertType::Danger,
                children: html!(),
            }),
            Some(FormHelperText::from((err.to_string(), InputState::Error))),
        ),
    };

    html! (
        <Form
            id="lookup-form"
            method="dialog"
            {alert}
        >
            <FormGroup
                label={props.label.clone()}
                required=true
                {helper_text}
            >
                {
                    match &props.r#type {
                        EntryType::Input => html!(
                            <TextInput
                                value={(*input).clone()}
                                {oninput}
                                autofocus=true
                            />
                        ),
                        EntryType::Area => html!(
                            <TextArea
                                value={(*input).clone()}
                                rows={5}
                                resize={ResizeOrientation::Vertical}
                                {oninput}
                                autofocus=true
                            />
                        )
                    }
                }
            </FormGroup>

        </Form>
    )
}
