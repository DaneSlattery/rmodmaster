//! Serial port selector

use gloo::{console::info, net::http::Request};
use material_yew::{MatListItem, MatSelect};
use yew::{
    function_component, html, platform::spawn_local, use_effect, use_effect_with_deps, use_state,
    Html, Properties,
};

#[derive(PartialEq, Properties)]
pub struct PortListProps {
    ports: Vec<String>,
}

#[function_component]
pub fn PortList(PortListProps { ports }: &PortListProps) -> Html {
    ports
        .iter()
        .map(|port_name| {
            html! {
                <MatListItem value={port_name.clone()}>{port_name}</MatListItem>
            }
        })
        .collect::<Html>()
}

#[derive(PartialEq, Properties)]
pub struct SerialPortSelectProps {}

#[function_component]
pub fn SerialPortSelect(props: &SerialPortSelectProps) -> Html {
    let SerialPortSelectProps {} = props;
    let ports = use_state(|| vec![]);
    {
        let ports = ports.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    if let Ok(x) = Request::get("/api/ports").send().await {
                        info!("Request success");
                        let json = x.json::<Vec<String>>().await;
                        match json {
                            Ok(x) => {
                                ports.set(x.clone());
                                info!("Got Ports: {:?}", x);
                            }
                            Err(x) => info!("Could not deser"),
                        }
                    }
                })
            },
            (),
        );
    }

    html! {
            <div>
                <section>
        <MatSelect label="Filled" >
            <PortList ports={(*ports).clone()}></PortList>
            // <MatListItem>{""}</MatListItem>
            // <MatListItem value="0">{"Option 0"}</MatListItem>
            // <MatListItem value="1">{"Option 1"}</MatListItem>
            // <MatListItem value="2">{"Option 2"}</MatListItem>
            // <MatListItem value="3">{"Option 3"}</MatListItem>
        </MatSelect>

    </section>
            </div>
        }
}
