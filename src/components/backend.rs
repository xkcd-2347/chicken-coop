use std::rc::Rc;
use url::Url;
use yew::prelude::*;
use yew_more_hooks::hooks::r#async::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BackendProperties {
    #[prop_or_default]
    pub children: Children,
    pub bootstrap_url: String,
}

#[function_component(Backend)]
pub fn backend(props: &BackendProperties) -> Html {
    let backend = use_async_with_options(
        async {
            Ok::<_, ()>(crate::backend::Backend {
                // we cannot use reqwest here, as we might need to do a relative lookup, based on the
                // current web page. Which is something that Url (which is used by reqwest) doesn't
                // support. But gloo_net does.
                // FIXME: load from somewhere later on using props.bootstrap_url
                url: Url::parse("https://api-trusted.apps.sandbox.drogue.world").unwrap(),
            })
        },
        UseAsyncOptions::enable_auto(),
    );

    match &*backend {
        UseAsyncState::Pending | UseAsyncState::Processing => html!(),
        UseAsyncState::Ready(Err(())) => html!(<>{"Failed to initialize backend"}</>),
        UseAsyncState::Ready(Ok(backend)) => html!(
            <ContextProvider<Rc<crate::backend::Backend>> context={Rc::new(backend.clone())}>
                { for props.children.iter() }
            </ContextProvider<Rc<crate::backend::Backend>>>
        ),
    }
}
