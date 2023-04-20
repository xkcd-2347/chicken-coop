use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(Package)]
pub fn package() -> Html {
    html!(
        <>
            <PageSection sticky={[PageSectionSticky::Top]} >
                <Content>
                    <Title size={Size::XXXXLarge}>{"Chickens"}</Title>
                    <p>{ "Bock, Bock!" }</p>
                </Content>
            </PageSection>

            // We need to set the main section to fill, as the have a footer section
            <PageSection fill={PageSectionFill::Fill}>
                <img src="assets/images/ben-moreland-auijD19Byq8-unsplash.jpg" />
            </PageSection>
        </>
    )
}
