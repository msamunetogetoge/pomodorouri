use chrono::{Duration, NaiveTime};
use wasm_bindgen_futures::spawn_local;
use yew::{classes, function_component, html, use_effect_with, use_state, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub time: NaiveTime,
}

#[function_component(Timer)]
pub fn timer(props: &Props) -> Html {
    let local_time = use_state(|| props.time);
    let display_time = use_state(|| props.time.format("%M:%S").to_string());

    html! {
        <>
            <div >
                <p>{display_time.to_string()}</p>
            </div>
        </>
    }
}

/// 0時0分0秒のNaiveTimeを返します
fn naive_time_from_zero() -> NaiveTime {
    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
}
