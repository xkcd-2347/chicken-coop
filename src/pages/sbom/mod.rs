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
    let container = r#"syft packages <container> -o cyclonedx-json --file sbom.json"#;
    let container_example =
        r#"syft packages quay.io/keycloak/keycloak:latest -o cyclonedx-json --file sbom.json"#;

    use patternfly_yew::next::TextInput;

    html!(
        <Card
            title={html!(<Title>{"Generate"}</Title>)}
        >
            <Tabs r#box=true>
                <Tab label="Container">
                    <Content>
                        <p> { "Run the following command:" } </p>
                        <p> <TextInput readonly=true value={container}  /> </p>
                        <p> { "Be sure to replace " } <code> {"<container>"} </code> { "with the actual name of the container, for example:" } </p>
                        <p> <Clipboard readonly=true code=true value={container_example} variant={ClipboardVariant::Expanded} /> </p>
                        <p> { "The SBOM will be generated as: " } <code> { "target/sbom.json" } </code> </p>
                    </Content>
                </Tab>
                <Tab label="Maven">
                    <Content>
                        <p> { "Run the following command from the root of your project:" } </p>
                        <p> <Clipboard readonly=true code=true value={maven} variant={ClipboardVariant::Expanded} /> </p>
                        <p> { "The SBOM will be generated as: " } <code> { "sbom.json" } </code> </p>
                    </Content>
                </Tab>
            </Tabs>
        </Card>
    )
}
