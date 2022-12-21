// use chrono::prelude::*;
pub mod api;
pub mod models;

pub use api::Api;
pub use models::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::events::Event;
use yew::prelude::*;

enum Msg {
    InputValue(String),
    SetResults(Vec<LocationData>),
    SetError(String),
    SetLocation(Box<LocationData>),
    ResetLocation(),
    SetDays(i64),
    SetBefore(i64),
    SetAfter(i64),
}

struct Model {
    text: String,
    results: Vec<LocationData>,
    client: Api,
    location: Option<LocationData>,
    days: i64,
    before: i64,
    after: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            text: "".to_string(),
            results: Vec::<LocationData>::new(),
            client: Api::new(),
            location: None,
            days: 0,
            before: 15,
            after: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputValue(text) => {
                let cl = self.client.clone();
                ctx.link().send_future(async move {
                    match cl.query_location(&text).await {
                        Ok(result) => Msg::SetResults(result),
                        Err(err) => {
                            log::error!("{}", err);
                            Msg::SetError(err)
                        }
                    }
                });
                false
            }
            Msg::SetError(err) => {
                self.text = err;
                true
            }
            Msg::SetResults(results) => {
                self.results = results;
                true
            }
            Msg::SetLocation(location) => {
                self.location = Some(*location);
                true
            }
            Msg::ResetLocation() => {
                self.location = None;
                true
            }
            Msg::SetDays(days) => {
                self.days = days;
                true
            }
            Msg::SetBefore(before) => {
                self.before = before;
                true
            }
            Msg::SetAfter(after) => {
                self.after = after;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_change = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Msg::InputValue(input.value()))
        });

        let change_days = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Msg::SetDays(input.value().parse::<i64>().unwrap()))
        });

        let change_before = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Msg::SetBefore(input.value().parse::<i64>().unwrap()))
        });

        let change_after = link.batch_callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            input.map(|input| Msg::SetAfter(input.value().parse::<i64>().unwrap()))
        });

        html! {
            <>
            <div class="columns is-mobile is-centered">
            <div id="title" class="column is-three-quarters">
            <p class="title is-3 has-text-centered">{"Astrolo Calendar Events"}</p>
            <p class="subtitle is-5 has-text-centered">{"Generate calendar events so you never miss a moonrise"}</p>
            </div>
            </div>
            <div class="columns is-mobile is-centered">
            <div id="locations" class="column is-three-quarters">
                <nav class="panel">
                <p class="panel-heading">{"Enter your location"}</p>
                <div class="panel-block">
                <p class="control">
                <input class="input" onchange={on_change} id="location" type="text"/>
                </p>
                </div>
                if self.location.is_none() {
                    {self.results.clone().into_iter().map(|result| {
                        let c_result = result.clone();
                        html!{
                            <a
                            class="panel-block"
                            href="#"
                            onclick={link.callback(move |e: MouseEvent| {
                                e.prevent_default();
                                Msg::SetLocation(Box::new(c_result.clone()))
                            })}
                            key={format!("{}{}", result.name, result.country_code)}>
                                {format!("{}", result)}
                            </a>
                        }
                    }).collect::<Html>()
                    }
                } else {
                    <a class="panel-block" href="#" onclick={link.callback(move |e: MouseEvent| {
                        e.prevent_default();
                        Msg::ResetLocation()
                    })}>
                        {format!("{}", self.location.clone().unwrap())}
                    </a>
                }
                </nav>
            if self.location.is_some() {
                <div class="card">
                <header class="card-header">
                    <p class="card-header-title">
                    {"Generate Calendar Events"}
                    </p>
                </header>
                <div class="card-content">
                <div class="content">
                <form id="download-ical" action="https://api.whynotcats.com/ical" method="post" target="_blank">
                <label class="label" for="days">{"Number of days to generate"}
                    <input class="input" onchange={change_days} id="days" name="number_of_days" value={self.days.to_string()} type="number"/>
                </label>
                <label class="label" for="before">{"Minutes before moonrise to start calendar event"}
                    <input class="input" onchange={change_before} id="before" name="before" value={self.before.to_string()} type="number"/>
                </label>
                <label class="label" for="after">{"Minutes after moonrise to end calendar event"}
                    <input class="input" onchange={change_after} id="after" name="after" value={self.after.to_string()} type="number"/>
                </label>
                <input type="text" hidden=true name="timezone" value={self.location.clone().unwrap().timezone.to_string()}/>
                <input type="float" hidden=true name="lat" value={self.location.clone().unwrap().latitude.to_string()}/>
                <input type="float" hidden=true name="lon" value={self.location.clone().unwrap().longitude.to_string()}/>
                <div class="field">
                <div class="control">
                <input class="button is-primary" type="submit" id="submit-ical" value="Download iCal"/>
                </div>
                </div>
                </form>
                </div>
                </div>
                </div>
            }
            </div>
            </div>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
