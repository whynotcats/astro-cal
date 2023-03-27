use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct CalendarProps {
    #[prop_or(AttrValue::from("UTC"))]
    pub timezone: AttrValue,
    pub latitude: f64,
    pub longitude: f64,
}

#[function_component(CalendarForm)]
pub fn create_calendar_form(props: &CalendarProps) -> Html {
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
