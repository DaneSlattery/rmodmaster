use serialport::SerialPortSelect;
use yew::prelude::*;
mod serialport;

#[function_component]
fn App() -> Html {
    html! {
        <div>

            <SerialPortSelect></SerialPortSelect>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
