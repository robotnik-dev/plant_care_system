use shared::models::reading::Reading;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let readings = [Reading::new(0, 0.43), Reading::new(1, 0.33)];

    let readings = readings
        .iter()
        .map(|reading| {
            html!(
                <p key={reading.id}>{format!("value: {}", reading.value)}</p>
            )
        })
        .collect::<Html>();

    html! {
        <div>
            { readings}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
