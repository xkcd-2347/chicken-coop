mod maven;

use crate::pages::AppRoute;
use anyhow::{anyhow, bail, Context};
use maven::MavenVariant;
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::str::FromStr;
use yew::prelude::*;
use yew_nested_router::prelude::*;

const DEFAULT_SEARCH: &str = "pkg:maven/io.quarkus/quarkus-core@2.16.2.Final?type=jar";

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
pub struct PackageLookupProperties {
    onchange: Callback<Option<PackageUrl<'static>>>,
}

const MSG_NOT_EMPTY: &str = "Must not be empty";

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
