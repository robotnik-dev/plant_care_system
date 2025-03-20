use gloo_net::http::Request;
use shared::models::reading::Reading;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ReadingListProps {
    readings: Vec<Reading>,
}

#[function_component(ReadingList)]
fn reading_list(ReadingListProps { readings }: &ReadingListProps) -> Html {
    readings
        .iter()
        .map(|reading| {
            html! {
                <p key={reading.id}>{format!("value: {}", reading.value)}</p>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let readings = use_state(Vec::new);
    {
        let readings = readings.clone();
        use_effect_with((), move |_| {
            let readings = readings.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_readings = Request::get("http://10.43.3.210/readings")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                readings.set(fetched_readings);
            });
            || ()
        });
    }
    // let readings = vec![Reading::new(0, 42.0), Reading::new(1, 69.0)];

    html! {
        <>
            <h1>{ "Heading" }</h1>
            <div>
            <h2>{ "Readings" }</h2>
                <ReadingList readings={(*readings).clone()} />
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
