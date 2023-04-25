use cyclonedx_bom::prelude::*;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

mod inspect;
mod upload;

use inspect::Inspect;
use upload::Upload;

#[function_component(SBOM)]
pub fn sbom() -> Html {
    let content = use_state_eq(|| None::<String>);

    let onsubmit = {
        let content = content.clone();
        Callback::from(move |data| {
            content.set(Some(data));
        })
    };

    let sbom = use_memo(
        |content| {
            content.as_ref().and_then(|data| {
                Bom::parse_from_json_v1_3(data.as_bytes())
                    .ok()
                    .map(|sbom| Rc::new((data.clone(), sbom)))
            })
        },
        content.clone(),
    );

    match sbom.as_ref() {
        Some(sbom) => {
            html!(<Inspect sbom={sbom.clone()}/>)
        }
        None => {
            let onvalidate =
                Callback::from(
                    |data: String| match Bom::parse_from_json_v1_3(data.as_bytes()) {
                        Ok(_sbom) => Ok(data),
                        Err(err) => Err(format!("Failed to parse SBOM: {err}")),
                    },
                );

            html!(
                <>
                    <CommonHeader />
                    <PageSection variant={PageSectionVariant::Default} fill=true>
                        <Grid gutter=true>
                            <GridItem cols={[8]}>
                                <Card
                                    title={html!(<Title> {"SBOM content"} </Title>)}
                                >
                                    <Upload {onsubmit} {onvalidate}/>
                                </Card>
                            </GridItem>
                            <GridItem cols={[4]}>
                                <GenerateCard />
                            </GridItem>
                        </Grid>
                    </PageSection>
                </>
            )
        }
    }
}

#[function_component(CommonHeader)]
fn common_header() -> Html {
    html!(
        <PageSection variant={PageSectionVariant::Light} fill=false>
            <Title level={Level::H1}>{"Inspect SBOM"}</Title>
            <Content>
                <p> {"Hm, let's have a look, …"} </p>
            </Content>
        </PageSection>
    )
}

#[function_component(GenerateCard)]
fn generate_card() -> Html {
    let maven = r#"mvn org.cyclonedx:cyclonedx-maven-plugin:2.7.7:makeAggregateBom -Dcyclonedx.skipAttach=true -DoutputFormat=json -DschemaVersion=1.3 -Dcyclonedx.verbose=false"#;

    html!(
        <Card
            title={html!(<Title>{"Generate"}</Title>)}
        >
            <Tabs>
                <Tab label="Maven">
                    <Content>
                        { "Run the following command from the root of your project:" }
                    </Content>
                    <Clipboard readonly=true code=true value={maven} variant={ClipboardVariant::Expanded}/>
                    <Content>
                        { "The SBOM will be generated as: " } <code> { "target/sbom.json" } </code>
                    </Content>
                </Tab>
            </Tabs>
        </Card>
    )
}
