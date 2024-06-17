use chrono::NaiveTime;
use gloo::timers::callback::Interval;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::{html, Component, Context, Html};

use gloo::console;

use crate::components::timer::Timer;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub enum Msg {
    Start,
    Stop,
    Reset,
    Tick,
    Break,
}

pub struct App {
    value: NaiveTime,
    timer_handle: Option<Interval>,
    is_break: bool,
}

fn pomodoro_time() -> NaiveTime {
    NaiveTime::parse_from_str("00:25:00", "%H:%M:%S").unwrap()
}

fn rest_time() -> NaiveTime {
    NaiveTime::parse_from_str("00:05:00", "%H:%M:%S").unwrap()
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: pomodoro_time(),
            timer_handle: None,
            is_break: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                if self.timer_handle.is_none() {
                    let link = ctx.link().clone();
                    let handle = Interval::new(1000, move || link.send_message(Msg::Tick));
                    self.timer_handle = Some(handle);
                }
                false
            }
            Msg::Tick => {
                if self.value > NaiveTime::from_hms_opt(0, 0, 0).unwrap() {
                    self.value = self.value - chrono::Duration::seconds(1);
                    return true;
                } else if !self.is_break {
                    self.timer_handle.take();
                    spawn_local(async {
                        let args = to_value(&()).unwrap();
                        invoke("show_break_notification", args).await;
                    });
                    ctx.link().send_message(Msg::Break);
                    false
                } else {
                    self.value = rest_time();
                    self.is_break = true;
                    true
                }
            }
            Msg::Reset => {
                self.value = pomodoro_time();
                self.is_break = false;
                ctx.link().send_message(Msg::Stop);
                return true;
            }
            Msg::Stop => {
                self.timer_handle = None;
                false
            }
            Msg::Break => {
                self.value = rest_time();
                self.is_break = true;
                let link = ctx.link().clone();
                let handle = Interval::new(1000, move || link.send_message(Msg::Tick));
                self.timer_handle = Some(handle);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let formatted_time = self.value.format("%M:%S").to_string();
        html! {
            <>
                <main class="container">

                    <div class="row">
                        <button onclick={ctx.link().callback(|_| Msg::Start)}> {"Start"} </button>
                        <button onclick={ctx.link().callback(|_| Msg::Stop)}> {"Stop"} </button>
                        <button onclick={ctx.link().callback(|_| Msg::Reset)}> {"Reset"} </button>
                    </div>
                    <div>
                    {formatted_time}
                    </div>
                    // <Timer time={*timer_setting} />

                </main>
            </>
        }
    }
}
