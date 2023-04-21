//! Re-usable component

pub mod backend;

use patternfly_yew::prelude::*;
use yew::prelude::*;

#[function_component(ExtLinkIcon)]
pub fn ext_link_icon() -> Html {
    html!(<span class="pf-u-icon-color-light pf-u-ml-sm pf-u-font-size-sm">{ Icon::ExternalLinkAlt }</span>)
}

#[function_component(Trusted)]
pub fn trusted() -> Html {
    html!(<Label color={Color::Gold} label="Trusted"/>)
}
