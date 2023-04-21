use crate::pages::AppRoute;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
    let router = use_router::<AppRoute>();

    let primary = Callback::from(move |_| {
        if let Some(router) = &router {
            router.push(AppRoute::Package {
                package: String::default(),
            });
        }
    })
    .into_action("Check package");

    let secondaries = vec![];

    html!(
        <>
            <PageSection variant={PageSectionVariant::Light} fill=true>
                <Bullseye>
                    <EmptyState
                        full_height=true
                        title="Chicken coop"
                        icon={Icon::Catalog}
                        {primary}
                        {secondaries}
                    >
                        { "Good heavens, no! I'm a chicken! The Royal Air Force doesn't let chickens behind the controls of a complex aircraft. " }
                    </EmptyState>
                </Bullseye>
            </PageSection>
        </>
    )
}
