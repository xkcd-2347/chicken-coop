use super::CommonHeader;
use crate::{
    backend::{Backend, PackageService},
    components::{deps::PackageReferences, remote_content},
};
use cyclonedx_bom::prelude::Bom;
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use std::str::FromStr;
use yew::prelude::*;
use yew_more_hooks::hooks::r#async::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InspectProperties {
    pub sbom: Rc<(String, Bom)>,
}

#[function_component(Inspect)]
pub fn inspect(props: &InspectProperties) -> Html {
    let tab = use_state_eq(|| 0);
    let onselect = {
        let tab = tab.clone();
        Callback::from(move |index: usize| {
            tab.set(index);
        })
    };

    let purls = use_memo(
        |sbom| match &sbom.1.components {
            Some(comps) => comps
                .0
                .iter()
                .filter_map(|c| c.purl.as_ref().map(|p| p.to_string()))
                .collect(),
            None => vec![],
        },
        props.sbom.clone(),
    );

    let backend = use_context::<Rc<Backend>>()
        .expect("Can only be called being wrapped by the 'Backend' component");

    let service = use_memo(
        |backend| PackageService::new((**backend).clone()),
        backend.clone(),
    );

    let fetch = {
        let service = service.clone();
        use_async_with_cloned_deps(
            |purls| async move {
                service
                    .lookup_batch(
                        purls
                            .iter()
                            .filter_map(|purl| PackageUrl::from_str(purl).ok()),
                    )
                    .await
            },
            purls.clone(),
        )
    };

    html!(
        <>
            <CommonHeader />

            <PageSection r#type={PageSectionType::Tabs} variant={PageSectionVariant::Light} sticky={[PageSectionSticky::Top]}>
                <Tabs inset={TabInset::Page} detached=true {onselect}>
                    <Tab label="SBOM"/>
                    <Tab label="Inspect"/>
                </Tabs>
            </PageSection>

            <PageSection hidden={*tab != 0} fill={PageSectionFill::Fill}>
                {
                    remote_content(&fetch, |data| {
                        html!(<PackageReferences refs={data.clone()} />)
                    })
                }
            </PageSection>

            <PageSection hidden={*tab != 1} variant={PageSectionVariant::Light} fill={PageSectionFill::Fill}>
                <CodeBlock>
                    <CodeBlockCode>
                        { &props.sbom.0 }
                    </CodeBlockCode>
                </CodeBlock>
           </PageSection>
        </>
    )
}
