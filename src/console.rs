use crate::{
    about,
    components::ExtLinkIcon,
    hooks::use_open,
    pages::{self, AppRoute},
};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

#[function_component(Console)]
pub fn console() -> Html {
    let logo = html! (
        <Brand src="assets/images/chicken-svgrepo-com.svg" alt="Chicken Logo" />
    );

    let sidebar = html_nested!(
        <PageSidebar>
            <Nav>
                <NavList>
                    <NavExpandable title="Home">
                        <NavRouterItem<AppRoute> to={AppRoute::Index}>{ "Overview" }</NavRouterItem<AppRoute>>
                        <NavItem to="https://docs.seedwing.io/" target="_blank">{ "Documentation" } <ExtLinkIcon/> </NavItem>
                        <NavItem to="https://docs.seedwing.io/examples/dev/index.html" target="_blank">{ "Examples" } <ExtLinkIcon/> </NavItem>
                    </NavExpandable>
                </NavList>
            </Nav>
        </PageSidebar>
    );

    let callback_docs = use_open("https://docs.seedwing.io/", "_blank");
    let callback_github = use_open("https://github.com/xkcd-2347/chicken-coop", "_blank");

    let backdropper = use_backdrop();

    let callback_about = Callback::from(move |_| {
        if let Some(backdropper) = &backdropper {
            backdropper.open(html!(<about::About/>));
        }
    });

    let tools = html!(
        <Toolbar>
            <ToolbarItem>
                <Button icon={Icon::Github} onclick={callback_github}/>
            </ToolbarItem>
            <ToolbarItem>
                <AppLauncher
                    position={Position::Right}
                    toggle={Icon::QuestionCircle}
                >
                    <AppLauncherItem onclick={callback_docs}>{ "Documentation" }</AppLauncherItem>
                    <AppLauncherItem onclick={callback_about}>{ "About" }</AppLauncherItem>
                </AppLauncher>
            </ToolbarItem>
        </Toolbar>
    );

    html!(
        <Router<AppRoute>>
            <Page {logo} {sidebar} {tools}>
                <RouterSwitch<AppRoute> {render}/>

                <PageSection variant={PageSectionVariant::Darker} fill={PageSectionFill::NoFill}>
                    {"Copyright © 2023 Red Hat, Inc. and "} <a href="https://github.com/seedwing-io" target="_blank"> {"The chickens"} </a> {"."}
                </PageSection>
            </Page>
        </Router<AppRoute>>
    )
}

fn render(route: AppRoute) -> Html {
    match route {
        AppRoute::Index => html!(<pages::Index/>),
    }
}
