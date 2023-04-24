use crate::pages::AppRoute;
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::str::FromStr;
use yew::prelude::*;
use yew_nested_router::prelude::*;

const DEFAULT_SEARCH: &str = "pkg:maven/io.quarkus/quarkus-core@2.13.7.Final-redhat-00003";

#[function_component(LookupPackageModal)]
pub fn lookup_package_modal() -> Html {
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
    let purl = use_state_eq(|| DEFAULT_SEARCH.to_string());

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
