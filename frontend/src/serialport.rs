//! Serial port selector

use gloo_console::info;
use gloo_net::http::Request;

use yew::{
    function_component, html, platform::spawn_local, use_effect_with_deps, use_state, Callback,
    Html, Properties, UseStateHandle,
};

fn get_ports(ports: UseStateHandle<Vec<String>>) {
    spawn_local(async move {
        if let Ok(x) = Request::get("/api/ports").send().await {
            info!("Request success");
            let json = x.json::<Vec<String>>().await;
            match json {
                Ok(x) => {
                    ports.set(x.clone());
                    info!("Got Ports: {:?}", x);
                }
                Err(_x) => info!("Could not deser"),
            }
        }
    })
}

fn set_port(port: String) {
    spawn_local(async move {
        let response = Request::post("/api/select_port").header("Content-Type", "application/json");
        let response = response.json(&port).unwrap().send().await;
        if let Ok(x) = response {
            info!("Request success");
            // let json = x.json::<Vec<String>>().await;
            // match json {
            //     Ok(x) => {
            //         ports.set(x.clone());
            //         info!("Got Ports: {:?}", x);
            //     }
            //     Err(_x) => info!("Could not deser"),
            // }
        }
    });
}
// #[derive(PartialEq, Properties)]
// pub struct PortListProps {
//     ports: Vec<String>,
// }

// #[function_component]
// pub fn PortList(PortListProps { ports }: &PortListProps) -> Html {
//     ports
//         .iter()
//         .map(|port_name| {
//             html! {
//                 <MatListItem value={port_name.clone()}>{port_name}</MatListItem>
//             }
//         })
//         .collect::<Html>()
// }

#[derive(PartialEq, Properties)]
pub struct SerialPortSelectProps {}

#[function_component]
pub fn SerialPortSelect(props: &SerialPortSelectProps) -> Html {
    let SerialPortSelectProps {} = props;
    let port: UseStateHandle<Option<String>> = use_state(|| Some("/dev/ttyS4".to_string()));
    let ports = use_state(|| vec![]);
    {
        let ports = ports.clone();
        use_effect_with_deps(
            move |_| {
                get_ports(ports);
            },
            (),
        );
    }

    let on_click = {
        let port = (*port).clone().unwrap().clone();
        Callback::from(move |_| set_port(port.clone()))
    };

    html! {
        <div>
            <section>
            <select name="serial" id="serail"></select>
            // <MatSelect label="Select Serial Port" >
            //     <PortList ports={(*ports).clone()}></PortList>

            // </MatSelect>

            // <MatButton label="Open" raised=true onclick={on_click}/>
            <button onclick={on_click}>{"Open"}</button>
            </section>
        </div>
    }
}
