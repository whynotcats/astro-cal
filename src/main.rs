// use chrono::prelude::*;
pub mod api;
pub mod models;

pub use api::Api;
pub use models::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, KeyboardEvent, MouseEvent};
use yew::{
    function_component, props, use_effect_with_deps, use_memo, use_state, AttrValue, Html,
    Properties, UseStateHandle,
};
use yew::{html, Callback};
use yew_hooks::{use_debounce, UseDebounceHandle};

#[function_component]
fn Header() -> Html {
    html! {
        <div class="columns is-mobile is-centered">
            <div id="title" class="column is-three-quarters">
                <p class="title is-3 has-text-centered">{"Astrological Calendar Events"}</p>
                <p class="subtitle is-5 has-text-centered">{"Generate calendar events so you never miss a moonrise"}</p>
            </div>
        </div>
    }
}

fn input_keyup_callback(
    state: &UseStateHandle<String>,
    debounced_callback: &UseDebounceHandle,
) -> Callback<KeyboardEvent> {
    let state = state.clone();
    let debounced_callback = debounced_callback.clone();

    Callback::from(move |e: KeyboardEvent| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        state.set(
            input
                .clone()
                .expect("Should have valid input element")
                .value(),
        );

        debounced_callback.run();
    })
}

fn select_location_callback(
    location_state: UseStateHandle<Option<LocationData>>,
    result: &LocationData,
) -> Callback<MouseEvent> {
    let result = result.clone();
    Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        location_state.set(Some(result.clone()));
    })
}

#[derive(Properties, PartialEq, Clone)]
struct CalendarProps {
    #[prop_or(AttrValue::from("UTC"))]
    timezone: AttrValue,
    latitude: f64,
    longitude: f64,
}

#[function_component(CalendarForm)]
fn create_calendar_form(props: &CalendarProps) -> Html {
    html! {
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
                            <input class="input" id="days" name="number_of_days" value={30} type="number"/>
                        </label>
                        <label class="label" for="before">{"Minutes before moonrise to start calendar event"}
                            <input class="input" id="before" name="before" value={15} type="number"/>
                        </label>
                        <label class="label" for="after">{"Minutes after moonrise to end calendar event"}
                            <input class="input" id="after" name="after" value={0} type="number"/>
                        </label>
                        <input type="text" hidden=true name="timezone" value={props.timezone.clone()}/>
                        <input type="float" hidden=true name="lat" value={props.latitude.to_string()}/>
                        <input type="float" hidden=true name="lon" value={props.longitude.to_string()}/>
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
}

#[function_component]
fn App() -> Html {
    let location_text = use_state(String::new);
    let debounced_text = use_state(String::new);
    let selected_location: UseStateHandle<Option<LocationData>> = use_state(|| None);
    let client = use_memo(|_| Api::new(), ());
    let error = use_state(|| false);
    let results: UseStateHandle<Vec<LocationData>> = use_state(Vec::new);
    let html_results = results.clone();
    let debounce_callback = {
        let location_text = location_text.clone();
        let debounced_text = debounced_text.clone();
        use_debounce(
            move || {
                debounced_text.set((*location_text).clone());
            },
            300,
        )
    };

    let location_callback = input_keyup_callback(&location_text, &debounce_callback);

    let calendar_props = use_memo(
        |location| match location.clone() {
            Some(location) => {
                props! {
                    CalendarProps {
                        timezone: location.timezone,
                        latitude: location.latitude,
                        longitude: location.longitude,
                    }
                }
            }
            None => {
                props! {
                        CalendarProps {
                        latitude: 0.0,
                        longitude: 0.0,
                    }
                }
            }
        },
        (*selected_location).clone(),
    );

    use_effect_with_deps(
        move |l| {
            let l = l.clone();
            log::debug!("Location text changed: {}", l);
            if !l.is_empty() {
                let results = results.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let text = l.clone();
                    match client.query_location(&text).await {
                        Ok(result) => {
                            log::info!("{:?}", result);
                            results.set(result);
                        }
                        Err(err) => {
                            log::error!("{}", err);
                            error.set(true);
                        }
                    }
                });
            }
        },
        (*debounced_text).clone(),
    );

    html! {
        <>
        <Header />
            <div class="columns is-mobile is-centered">
            <div id="locations" class="column is-three-quarters">
                <nav class="panel">
                <p class="panel-heading">{"Enter your location"}</p>
                <div class="panel-block">
                <p class="control">
                <input class="input" onkeyup={location_callback} id="location" type="text"/>
                </p>
                </div>

                if (*selected_location).is_none() && html_results.len() != 0 {
                    <div>
                    {(*html_results).clone().into_iter().map(|result| {
                        let c_result = result.clone();
                        html!{
                            <a
                            class="panel-block"
                            href="#"
                            onclick={select_location_callback(selected_location.clone(), &c_result)}
                            id={result.html_key()}
                            key={result.html_key()}>
                                {format!("{}", result)}
                            </a>
                        }
                    }).collect::<Html>()
                    }
                    </div>
                } else if (*selected_location).is_some() {
                    <a class="panel-block" href="#">
                        {format!("{}", (*selected_location).clone().unwrap())}
                    </a>
                }
                </nav>

            if (*selected_location).is_some() {
                <CalendarForm ..(*calendar_props).clone()/>
            }

            </div>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
