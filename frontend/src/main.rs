use serialport::SerialPortSelect;
use yew::prelude::*;
mod serialport;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <SerialPortSelect></SerialPortSelect>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
