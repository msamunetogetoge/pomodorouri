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
    StartBreak,
    ShowBreakNotification,
}

pub struct App {
    value: NaiveTime,
    timer_handle: Option<Interval>,
    is_break: bool,
}

/// 00:25:00を返す
fn pomodoro_time() -> NaiveTime {
    NaiveTime::parse_from_str("00:00:10", "%H:%M:%S").unwrap()
}

/// 00:05:00を返す
fn rest_time() -> NaiveTime {
    NaiveTime::parse_from_str("00:00:15", "%H:%M:%S").unwrap()
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
            Msg::Start => self.start_timer(ctx),
            Msg::Tick => self.tick(ctx),
            Msg::Reset => self.reset_timer(ctx),
            Msg::Stop => self.stop_timer(),
            Msg::StartBreak => self.start_break(ctx),
            Msg::ShowBreakNotification => self.show_break_notification(),
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

impl App {
    fn start_timer(&mut self, ctx: &Context<Self>) -> bool {
        if self.timer_handle.is_none() {
            let link = ctx.link().clone();
            let handle = Interval::new(1000, move || link.send_message(Msg::Tick));
            self.timer_handle = Some(handle);
        }
        false
    }

    fn tick(&mut self, ctx: &Context<Self>) -> bool {
        if self.value > NaiveTime::from_hms_opt(0, 0, 0).unwrap() {
            self.value = self.value - chrono::Duration::seconds(1);
            true
        } else if !self.is_break {
            // タイマーが0で、休憩モードでない場合
            self.timer_handle.take();
            ctx.link().send_message(Msg::ShowBreakNotification);
            ctx.link().send_message(Msg::StartBreak);
            false
        } else {
            false
        }
    }

    fn reset_timer(&mut self, ctx: &Context<Self>) -> bool {
        self.value = pomodoro_time();
        self.is_break = false;
        ctx.link().send_message(Msg::Stop);
        true
    }

    fn stop_timer(&mut self) -> bool {
        if let Some(handle) = self.timer_handle.take() {
            handle.cancel();
        }
        false
    }

    fn start_break(&mut self, ctx: &Context<Self>) -> bool {
        self.value = rest_time();
        self.is_break = true;
        let link = ctx.link().clone();
        let handle = Interval::new(1000, move || link.send_message(Msg::Tick));
        self.timer_handle = Some(handle);
        true
    }

    fn show_break_notification(&mut self) -> bool {
        spawn_local(async {
            let args = to_value(&()).unwrap();
            invoke("show_break_notification", args).await;
        });
        false
    }
}
