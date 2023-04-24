use crate::{backend::data, components::Trusted, pages::AppRoute};
use packageurl::PackageUrl;
use patternfly_yew::prelude::*;
use std::str::FromStr;
use yew::prelude::*;
use yew_nested_router::components::Link;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PackageRefsProperties {
    pub refs: Vec<data::PackageRef>,
}

#[derive(PartialEq)]
struct PackageRef {
    label: String,
    purl: PackageUrl<'static>,
    pkg: data::PackageRef,
}

impl TableEntryRenderer for PackageRef {
    fn render_cell(&self, context: &CellContext) -> Cell {
        match context.column {
            0 => html!(
                <>
                    <Link<AppRoute> target={AppRoute::Package {package: self.pkg.purl.clone()}}>{&self.label}</Link<AppRoute>>
                    if let Some(true) = &self.pkg.trusted {
                        <Trusted />
                    }
                </>
            ),
            1 => self.purl.version().map(Html::from).unwrap_or_default(),
            2 => html!(self.purl.ty()),
            3 => html!( {
                for self.purl.qualifiers().iter().map(|(k,v)| html!(
                    <Label label={format!("{k}={v}")} />
                ))
            }),
            _ => html!(),
        }
        .into()
    }
}

#[function_component(PackageReferences)]
pub fn package_refs(props: &PackageRefsProperties) -> Html {
    let mut refs = Vec::with_capacity(props.refs.len());
    for pkg in &props.refs {
        let purl = match PackageUrl::from_str(&pkg.purl) {
            Ok(purl) => purl,
            Err(_) => continue,
        };
        let label = match purl.namespace() {
            Some(namespace) => format!("{namespace} : {name}", name = purl.name()),
            None => purl.name().to_string(),
        };
        refs.push(PackageRef {
            label,
            purl,
            pkg: pkg.clone(),
        });
    }

    let header = html_nested!(
        <TableHeader>
            <TableColumn label="Name" />
            <TableColumn label="Version"/>
            <TableColumn/>
            <TableColumn/>
        </TableHeader>
    );

    let entries = SharedTableModel::new(refs);

    html!(
        <Table<SharedTableModel<PackageRef>>
            mode={TableMode::CompactNoBorders}
            {header} {entries}
        />
        /*
        <List r#type={ListType::Plain}>
            {for refs.iter().map(|r|{
                html!(<>
                    <yew_nested_router::components::Link<AppRoute>
                        target={AppRoute::Package { package: r.purl.to_string()}}
                    >
                        { &r.label }
                    </yew_nested_router::components::Link<AppRoute>>
                    if r.pkg.trusted.unwrap_or_default() {
                        {" "}<Trusted/>
                    }
                </>)
            })}
        </List>*/
    )
}
